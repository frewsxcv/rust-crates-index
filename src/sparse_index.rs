use crate::{path_max_byte_len, dirs::get_index_details, Crate, Error, IndexConfig};
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

    /// Reads a crate from the local cache of the index. There are no guarantees around freshness,
    /// and if the crate is not known in the cache, no fetch will be performed.
    pub fn crate_from_cache(&self, name: &str) -> Result<Crate, Error> {
        let rel_path = crate::crate_name_to_relative_path(name)
            .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidInput, "bad name"))?;

        // avoid realloc on each push
        let mut cache_path = PathBuf::with_capacity(path_max_byte_len(&self.path) + 8 + rel_path.len());
        cache_path.push(&self.path);
        cache_path.push(".cache");
        cache_path.push(rel_path);
        let cache_bytes = std::fs::read(&cache_path)?;
        Ok(Crate::from_cache_slice(&cache_bytes, None)?)
    }

    /// The HTTP url of the index
    #[inline]
    pub fn url(&self) -> &str {
        self.url.strip_prefix("sparse+").unwrap_or(&self.url)
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn parses_cache() {
        let index = super::Index::with_path(
            std::path::Path::new(&std::env::var_os("CARGO_MANIFEST_DIR").unwrap()).join("tests/testdata/sparse_registry_cache/cargo_home"),
            crate::CRATES_IO_HTTP_INDEX
        ).unwrap();

        let crate_ = index.crate_from_cache("autocfg").unwrap();

        assert_eq!(crate_.name(), "autocfg");
        assert_eq!(crate_.versions().len(), 13);
        assert_eq!(crate_.earliest_version().version(), "0.0.1");
        assert_eq!(crate_.highest_version().version(), "1.1.0");
    }
}
