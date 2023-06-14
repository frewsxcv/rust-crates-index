use crate::{dirs::get_index_details, path_max_byte_len, Crate, Error, IndexConfig};
use std::io;
use std::path::{Path, PathBuf};

/// The default URL of the crates.io HTTP index, see [`Index::from_url`] and [`Index::new_cargo_default`]
pub const CRATES_IO_HTTP_INDEX: &str = "sparse+https://index.crates.io/";

/// Wrapper around managing a sparse HTTP index, re-using Cargo's local disk caches.
///
/// Currently it only uses local Cargo cache, and does not access the network in any way.
pub struct Index {
    path: PathBuf,
    url: String,
}

impl Index {
    /// Creates a view over the sparse HTTP index from a provided URL, opening
    /// the same location on disk that Cargo uses for that registry index's
    /// metadata and cache.
    ///
    /// Note this function takes the `CARGO_HOME` environment variable into account
    #[inline]
    pub fn from_url(url: &str) -> Result<Self, Error> {
        Self::with_path(home::cargo_home()?, url)
    }

    /// Creates an index for the default crates.io registry, using the same
    /// disk location as Cargo itself.
    ///
    /// This is the recommended way to access the crates.io sparse index.
    ///
    /// Note this function takes the `CARGO_HOME` environment variable into account
    #[inline]
    pub fn new_cargo_default() -> Result<Self, Error> {
        Self::from_url(CRATES_IO_HTTP_INDEX)
    }

    /// Creates a view over the sparse HTTP index from the provided URL, rooted
    /// at the specified location
    #[inline]
    pub fn with_path(cargo_home: impl AsRef<Path>, url: impl AsRef<str>) -> Result<Self, Error> {
        let url = url.as_ref();
        // It is required to have the sparse+ scheme modifier for sparse urls as
        // they are part of the short ident hash calculation done by cargo
        if !url.starts_with("sparse+http") {
            return Err(Error::Url(url.to_owned()));
        }

        let (path, url) = get_index_details(url, Some(cargo_home.as_ref()))?;
        Ok(Self::at_path(path, url))
    }

    /// Creates a view over the sparse HTTP index at the exact specified path
    #[inline]
    pub fn at_path(path: PathBuf, mut url: String) -> Self {
        if !url.ends_with('/') {
            url.push('/');
        }
        Self { path, url }
    }

    /// Get the global configuration of the index.
    pub fn index_config(&self) -> Result<IndexConfig, Error> {
        let path = self.path.join("config.json");
        let bytes = std::fs::read(path).map_err(Error::Io)?;

        serde_json::from_slice(&bytes).map_err(Error::Json)
    }

    /// Reads a crate from the local cache of the index.
    ///
    /// There are no guarantees around freshness, and if the crate is not known
    /// in the cache, no fetch will be performed.
    pub fn crate_from_cache(&self, name: &str) -> Result<Crate, Error> {
        let rel_path = crate::crate_name_to_relative_path(name, std::path::MAIN_SEPARATOR)
            .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidInput, "bad name"))?;

        // avoid realloc on each push
        let mut cache_path =
            PathBuf::with_capacity(path_max_byte_len(&self.path) + 8 + rel_path.len());
        cache_path.push(&self.path);
        cache_path.push(".cache");
        cache_path.push(rel_path);
        let cache_bytes = std::fs::read(&cache_path).map_err(|err| {
            if err.kind() == io::ErrorKind::NotFound {
                Error::Io(io::Error::new(
                    io::ErrorKind::NotFound,
                    cache_path.to_string_lossy().to_owned(),
                ))
            } else {
                err.into()
            }
        })?;
        Ok(Crate::from_cache_slice(&cache_bytes, None)?)
    }

    /// Get the URL that can be used to fetch the index entry for the specified
    /// crate
    ///
    /// The body of a successful response for the returned URL can be parsed
    /// via [`Crate::from_slice`]
    #[inline]
    pub fn crate_url(&self, name: &str) -> Option<String> {
        let rel_path = crate::crate_name_to_relative_path(name, '/')?;
        Some(format!("{}{rel_path}", self.url()))
    }

    /// The HTTP url of the index
    #[inline]
    pub fn url(&self) -> &str {
        self.url.strip_prefix("sparse+").unwrap_or(&self.url)
    }
}

#[cfg(test)]
mod test {
    use super::Index;
    use std::path::PathBuf;

    #[test]
    fn opens_crates_io() {
        let index = Index::with_path(
            PathBuf::from(std::env::var_os("CARGO_MANIFEST_DIR").unwrap())
                .join("tests/testdata/sparse_registry_cache/cargo_home"),
            super::CRATES_IO_HTTP_INDEX,
        )
        .unwrap();

        assert_eq!(index.url(), "https://index.crates.io/");
        assert_eq!(
            index.crate_url("autocfg").unwrap(),
            "https://index.crates.io/au/to/autocfg"
        );
    }

    #[test]
    fn parses_cache() {
        let index = Index::with_path(
            PathBuf::from(std::env::var_os("CARGO_MANIFEST_DIR").unwrap())
                .join("tests/testdata/sparse_registry_cache/cargo_home"),
            super::CRATES_IO_HTTP_INDEX,
        )
        .unwrap();

        let crate_ = index.crate_from_cache("autocfg").unwrap();

        assert_eq!(crate_.name(), "autocfg");
        assert_eq!(crate_.versions().len(), 13);
        assert_eq!(crate_.earliest_version().version(), "0.0.1");
        assert_eq!(crate_.highest_version().version(), "1.1.0");
    }

    struct TestIndex {
        client: reqwest::blocking::Client,
        index: Index,
        _temp_dir: tempfile::TempDir,
    }

    use reqwest::header;

    impl TestIndex {
        fn new() -> Self {
            use header::HeaderValue;
            let mut headers = header::HeaderMap::new();
            headers.insert("cargo-protocol", HeaderValue::from_static("version=1"));
            // All index entries are just files with lines of JSON
            headers.insert(header::ACCEPT, HeaderValue::from_static("text/plain"));
            // We need to accept both identity and gzip, as otherwise cloudfront will
            // always respond to requests with strong etag's
            headers.insert(
                header::ACCEPT_ENCODING,
                HeaderValue::from_static("gzip,identity"),
            );

            let client = reqwest::blocking::ClientBuilder::new()
                .http2_prior_knowledge()
                .default_headers(headers)
                .build()
                .unwrap();

            let temp_dir = tempfile::tempdir().unwrap();
            let index = Index::at_path(
                temp_dir.path().to_owned(),
                super::CRATES_IO_HTTP_INDEX.to_owned(),
            );

            Self {
                client,
                index,
                _temp_dir: temp_dir,
            }
        }

        // Write a cache entry for the item from crates.io
        fn write_cache_entry(&self, name: &str) {
            let url = dbg!(self.index.crate_url(name).unwrap());

            let res = self
                .client
                .get(url)
                .send()
                .unwrap()
                .error_for_status()
                .unwrap();

            let hdrs = res.headers();
            // Prefer etag, same as cargo
            let version = if let Some(etag) = hdrs.get(header::ETAG) {
                format!("{}: {}", header::ETAG, etag.to_str().unwrap())
            } else if let Some(lm) = hdrs.get(header::LAST_MODIFIED) {
                format!("{}: {}", header::LAST_MODIFIED, lm.to_str().unwrap())
            } else {
                "Unknown".to_owned()
            };

            let body = res.bytes().unwrap();

            // Might as well exercise Crate serde as well
            let krate = crate::Crate::from_slice(&body).unwrap();

            let rel_path =
                crate::crate_name_to_relative_path(name, std::path::MAIN_SEPARATOR).unwrap();
            let mut cache_path = self.index.path.join(".cache");
            cache_path.push(rel_path);
            std::fs::create_dir_all(cache_path.parent().unwrap()).unwrap();

            krate.write_to_file(&version, &cache_path);
        }
    }

    #[test]
    fn end_to_end() {
        let ti = TestIndex::new();
        ti.write_cache_entry("sval");
        ti.write_cache_entry("serde");

        let krate = ti
            .index
            .crate_from_cache("sval")
            .expect("Could not find the crate libnotify in the index");

        let version = krate
            .versions()
            .iter()
            .find(|v| v.version() == "0.0.1")
            .expect("Version 0.0.1 of sval does not exist?");
        let dep_with_package_name = version
            .dependencies()
            .iter()
            .find(|d| d.name() == "serde_lib")
            .expect("sval does not have expected dependency?");
        assert_ne!(
            dep_with_package_name.name(),
            dep_with_package_name.package().unwrap()
        );
        assert_eq!(
            dep_with_package_name.crate_name(),
            dep_with_package_name.package().unwrap()
        );

        let serde = ti
            .index
            .crate_from_cache("serde")
            .expect("failed to find serde");
        serde
            .versions()
            .iter()
            .find(|sv| sv.version() == "1.0.69")
            .expect("not nice");
    }
}
