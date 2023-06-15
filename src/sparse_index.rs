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
        let cache_path = self.cache_path(name)
            .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidInput, "bad name"))?;

        let cache_bytes = std::fs::read(cache_path)?;
        Ok(Crate::from_cache_slice(&cache_bytes, None)?)
    }

    /// The HTTP url of the index
    #[inline]
    pub fn url(&self) -> &str {
        self.url.strip_prefix("sparse+").unwrap_or(&self.url)
    }

    /// Get the URL that can be used to fetch the index entry for the specified
    /// crate
    ///
    /// The body of a successful response for the returned URL can be parsed
    /// via [`Crate::from_slice`]
    #[inline]
    pub fn crate_url(&self, name: &str) -> Option<String> {
        let rel_path = crate::crate_name_to_relative_path(name)?;
        Some(format!("{}{rel_path}", self.url()))
    }

    /// Gets the full path to the cache file for the specified crate
    fn cache_path(&self, name: &str) -> Option<PathBuf> {
        let rel_path = crate::crate_name_to_relative_path(name)?;

        // avoid realloc on each push
        let mut cache_path = PathBuf::with_capacity(path_max_byte_len(&self.path) + 8 + rel_path.len());
        cache_path.push(&self.path);
        cache_path.push(".cache");
        cache_path.push(rel_path);

        Some(cache_path)
    }

    /// Reads the version of the cache entry for the specified crate, if it exists
    /// 
    /// The version is of the form `key:value`, where, currently, the key is either
    /// `etag` or `last-modified`
    pub fn read_cache_version(&self, name: &str) -> Option<String> {
        let cache_path = self.cache_path(name)?;
        let bytes = std::fs::read(cache_path).ok()?;

        const CURRENT_CACHE_VERSION: u8 = 3;
        const CURRENT_INDEX_FORMAT_VERSION: u32 = 2;

        let (&first_byte, rest) = bytes.split_first()?;

        if first_byte != CURRENT_CACHE_VERSION {
            return None;
        }

        let index_v_bytes = rest.get(..4)?;
        let index_v = u32::from_le_bytes(index_v_bytes.try_into().unwrap());
        if index_v != CURRENT_INDEX_FORMAT_VERSION {
            return None;
        }
        let rest = &rest[4..];

        let version = crate::split(rest, 0).next().and_then(|version| {
            std::str::from_utf8(version).ok().map(String::from)
        });

        version
    }

    /// Creates an HTTP request that can be sent via your HTTP client of choice
    /// to retrieve the current metadata for the specified crate
    /// 
    /// See [`Self::parse_cache_response`] processing the response from the remote
    /// index
    /// 
    /// It is highly recommended to assume HTTP/2 when making requests to remote
    /// indices, at least crates.io
    #[cfg(feature = "sparse-http")]
    pub fn make_cache_request(&self, name: &str) -> Result<http::Request<()>, Error> {
        use http::header;

        let url = self.crate_url(name).ok_or_else(|| {
            io::Error::new(io::ErrorKind::InvalidInput, "crate name is invalid")
        })?;

        let cache_version = self.read_cache_version(name);

        let mut req = http::Request::get(url).version(http::Version::HTTP_2);

        {
            let headers = req.headers_mut().unwrap();

            // AFAICT this does not affect responses at the moment, but could in the future
            // if there are changes
            headers.insert(
                "cargo-protocol",
                header::HeaderValue::from_static("version=1"),
            );
            // All index entries are just files with lines of JSON
            headers.insert(
                header::ACCEPT,
                header::HeaderValue::from_static("text/plain"),
            );
            // We need to accept both identity and gzip, as otherwise cloudfront will
            // always respond to requests with strong etag's, which will differ from
            // cache entries generated by cargo
            headers.insert(
                header::ACCEPT_ENCODING,
                header::HeaderValue::from_static("gzip,identity"),
            );

            // If we have a local cache entry, include its version with the
            // appropriate header, this allows the server to respond with a
            // cached, or even better, empty response if its version matches
            // the local one making the request/response loop basically free
            if let Some(cache_version) = cache_version {
                if let Some((key, value)) = cache_version.split_once(':') {
                    if let Ok(value) = header::HeaderValue::from_str(value.trim()) {
                        if key == header::ETAG {
                            headers.insert(header::IF_NONE_MATCH, value);
                        } else if key == header::LAST_MODIFIED {
                            headers.insert(header::IF_MODIFIED_SINCE, value);
                        } else {
                            // We could error here, but that's kind of pointless
                            // since the response will be sent in full if we haven't
                            // specified one of the above headers. Though it does
                            // potentially indicate something weird is going on
                        }
                    }
                }
            }
        }

        Ok(req.body(()).unwrap())
    }

    /// Process the response to a request created by [`Self::make_cache_request`]
    /// 
    /// This handles both the scenario where the local cache is missing the specified
    /// crate, or it is out of date, as well as the local entry being up to date
    /// and can just be read from disk
    /// 
    /// You may specify whether an updated index entry is written locally to the
    /// cache or not
    /// 
    /// Note that responses from sparse HTTP indices, at least crates.io, may
    /// send responses with `gzip` compression, it is your responsibility to
    /// decompress it before sending to this function
    #[cfg(feature = "sparse-http")]
    pub fn parse_cache_response(&self, name: &str, response: http::Response<Vec<u8>>, write_cache_entry: bool) -> Result<Option<Crate>, Error> {
        use http::{header, StatusCode};
        let (parts, body) = response.into_parts();

        match parts.status {
            // The server responded with the full contents of the index entry
            StatusCode::OK => {
                let krate = Crate::from_slice(&body)?;

                if write_cache_entry {
                    // The same as cargo, prefer etag over last-modified
                    let version = if let Some(etag) = parts.headers.get(header::ETAG) {
                        etag.to_str().ok().map(|etag| format!("{}: {etag}", header::ETAG))
                    } else if let Some(lm) = parts.headers.get(header::LAST_MODIFIED) {
                        lm.to_str().ok().map(|lm| format!("{}: {lm}", header::LAST_MODIFIED))
                    } else {
                        None
                    };

                    let version = version.unwrap_or_else(|| "Unknown".to_owned());

                    // This should always succeed, but no need to panic or fail
                    if let Some(cache_path) = self.cache_path(name) {
                        if std::fs::create_dir_all(cache_path.parent().unwrap()).is_ok() {
                            // It's unfortunate if this fails for some reason, but
                            // not writing the cache entry shouldn't stop the user
                            // from getting the crate's metadata
                            let _ = krate.write_cache_entry(&cache_path, &version);
                        }
                    }
                }

                Ok(Some(krate))
            }
            // The local cache entry is up to date with the latest entry on the
            // server, we can just return the local one
            StatusCode::NOT_MODIFIED => {
                self.crate_from_cache(name).map(Option::Some)
            }
            // The server requires authorization but the user didn't provide it
            StatusCode::UNAUTHORIZED => {
                Err(io::Error::new(io::ErrorKind::PermissionDenied, "the request was not authorized").into())
            }
            // The crate does not exist, or has been removed
            StatusCode::NOT_FOUND | StatusCode::GONE | StatusCode::UNAVAILABLE_FOR_LEGAL_REASONS => {
                Ok(None)
            }
            other => {
                Err(io::Error::new(io::ErrorKind::Unsupported, format!("the server responded with status code '{other}', which is not supported in the current protocol")).into())
            }
        }
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

#[cfg(all(test, feature = "http"))]
mod http_tests {
    #[inline]
    fn crates_io() -> super::Index {
        super::Index::with_path(
            std::path::Path::new(&std::env::var_os("CARGO_MANIFEST_DIR").unwrap()).join("tests/testdata/sparse_registry_cache/cargo_home"),
            crate::CRATES_IO_HTTP_INDEX
        ).unwrap()
    }

    use http::header;

    // Validates that a valid request is generated when there is no cache entry
    // for a crate
    #[test]
    fn generates_request_for_missing_cache_entry() {
        let index = crates_io();
        let req = index.make_cache_request("serde").unwrap();

        assert_eq!(req.uri(), format!("{}se/rd/serde", index.url()).as_str());
        assert!(req.headers().get(header::IF_NONE_MATCH).is_none());
        assert!(req.headers().get(header::IF_MODIFIED_SINCE).is_none());
        assert_eq!(req.headers().get(header::ACCEPT_ENCODING).unwrap(), "gzip,identity");
        assert_eq!(req.headers().get(header::HeaderName::from_static("cargo-protocol")).unwrap(), "version=1");
        assert_eq!(req.headers().get(header::ACCEPT).unwrap(), "text/plain");
    }

    // Validates that a valid request is generated when there is a local cache
    // entry for a crate
    #[test]
    fn generates_request_for_local_cache_entry() {
        let index = crates_io();
        let req = index.make_cache_request("autocfg").unwrap();

        assert_eq!(req.uri(), format!("{}au/to/autocfg", index.url()).as_str());
        assert_eq!(req.headers().get(header::IF_NONE_MATCH).unwrap(), "W/\"aa975a09419f9c8f61762a3d06fdb67d\"");
        assert!(req.headers().get(header::IF_MODIFIED_SINCE).is_none());
    }

    // curl -v -H 'accept-encoding: gzip,identity' -H 'if-none-match: W/"aa975a09419f9c8f61762a3d06fdb67d"' https://index.crates.io/au/to/autocfg
    // as of 2023-06-15
    const AUTOCFG_INDEX_ENTRY: &[u8] = include_bytes!("../tests/testdata/autocfg.txt");

    // Validates that a response with the full index contents are properly parsed
    #[test]
    fn parses_modified_response() {
        let index = crates_io();
        let response = http::Response::builder()
            .status(http::StatusCode::OK)
            .header(header::ETAG, "W/\"5f15de4a723e10b3f9eaf048d693cccc\"")
            .body(AUTOCFG_INDEX_ENTRY.to_vec()).unwrap();

        let krate = index.parse_cache_response("autocfg", response, false).unwrap().unwrap();
        assert_eq!(krate.highest_version().version(), "1.1.0");
    }

    // Validates that a response for an index entry that has not been modified is
    // parsed correctly
    #[test]
    fn parses_unmodified_response() {
        let index = crates_io();
        let response = http::Response::builder()
            .status(http::StatusCode::NOT_MODIFIED)
            .header(header::ETAG, "W/\"5f15de4a723e10b3f9eaf048d693cccc\"")
            .body(Vec::new()).unwrap();

        let krate = index.parse_cache_response("autocfg", response, false).unwrap().unwrap();
        assert_eq!(krate.name(), "autocfg");
        assert_eq!(krate.versions().len(), 13);
        assert_eq!(krate.earliest_version().version(), "0.0.1");
        assert_eq!(krate.highest_version().version(), "1.1.0");
    }

    // Validates that a response for an index entry that does not exist is
    // parsed correcty
    #[test]
    fn parses_missing_response() {
        let index = crates_io();
        let response = http::Response::builder()
            .status(http::StatusCode::NOT_FOUND)
            .body(Vec::new()).unwrap();

        assert!(index.parse_cache_response("serde", response, false).unwrap().is_none());
    }

    // curl -v -H 'accept-encoding: gzip,identity' https://index.crates.io/cr/at/crates-index
    const CRATES_INDEX_INDEX_ENTRY: &[u8] = include_bytes!("../tests/testdata/crates-index.txt");

    // Validates that a valid cache entry is written if the index entry has been
    // modified
    #[test]
    fn writes_cache_entry() {
        let index = crates_io();

        let path = index.cache_path("crates-index").unwrap();
        if path.exists() {
            std::fs::remove_file(path).expect("failed to remove existing crates-index cache file");
        }

        let response = http::Response::builder()
            .status(http::StatusCode::OK)
            .header(header::ETAG, "W/\"7fbfc422231ec53a9283f2eb2fb4f459\"")
            .body(CRATES_INDEX_INDEX_ENTRY.to_vec()).unwrap();

        let http_krate = index.parse_cache_response("crates-index", response, true).unwrap().unwrap();
        let cache_krate = index.crate_from_cache("crates-index").unwrap();

        for (http, cache) in http_krate.versions().iter().zip(cache_krate.versions().iter()) {
            assert_eq!(http.version(), cache.version());
        }
    }
}