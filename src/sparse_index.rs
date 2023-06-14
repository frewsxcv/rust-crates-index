use crate::{path_max_byte_len, dirs::url_to_local_dir, Crate, Error, IndexConfig};
use std::io;
use std::path::PathBuf;

/// Wrapper around managing a sparse HTTP index, re-using Cargo's local disk caches.
///
/// Currently it only uses local Cargo cache, and does not access the network in any way.
pub struct Index {
    path: PathBuf,
}

impl Index {
    /// Creates a view over the sparse HTTP index from a provided URL, opening the same location on
    /// disk that Cargo uses for that registry index's metadata and cache.
    pub fn from_url(url: &str) -> Result<Self, Error> {
        let (dir_name, _) = url_to_local_dir(url)?;
        let mut path = home::cargo_home()?;

        path.push("registry");
        path.push("index");
        path.push(dir_name);

        Ok(Self { path })
    }

    /// Get the global configuration of the index.
    pub fn index_config(&self) -> Result<IndexConfig, Error> {
        let path = self.path.join("config.json");
        let bytes = std::fs::read(path).map_err(Error::Io)?;

        serde_json::from_slice(&bytes).map_err(Error::Json)
    }

    /// Reads a crate from the local cache of the index. There are no guarantees around freshness,
    /// and if the crate is not known in the cache, no fetch will be performed.
    #[must_use] pub fn crate_from_cache(&self, name: &str) -> Result<Crate, Error> {
        let rel_path = crate::crate_name_to_relative_path(name)
            .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidInput, "bad name"))?;

        // avoid realloc on each push
        let mut cache_path = PathBuf::with_capacity(path_max_byte_len(&self.path) + 8 + rel_path.len());
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
}

#[cfg(test)]
mod test {
    use std::ffi::OsString;
    use std::path::PathBuf;

    #[test]
    fn parses_cache() {
        let _resetter = EnvVarResetter::set(
            "CARGO_HOME",
            PathBuf::from(std::env::var_os("CARGO_MANIFEST_DIR").unwrap())
                .join("tests")
                .join("testdata")
                .join("sparse_registry_cache")
                .join("cargo_home"),
        );

        let index = super::Index::from_url("sparse+https://index.crates.io/").unwrap();

        let crate_ = index.crate_from_cache("autocfg").unwrap();

        assert_eq!(crate_.name(), "autocfg");
        assert_eq!(crate_.versions().len(), 13);
        assert_eq!(crate_.earliest_version().version(), "0.0.1");
        assert_eq!(crate_.highest_version().version(), "1.1.0");
    }

    struct EnvVarResetter {
        key: OsString,
        value: Option<OsString>,
    }

    impl EnvVarResetter {
        fn set<K: Into<OsString>, V: Into<OsString>>(key: K, value: V) -> EnvVarResetter {
            let key = key.into();
            let value = value.into();
            let old_value = std::env::var_os(&key);

            std::env::set_var(&key, value);

            EnvVarResetter {
                key,
                value: old_value,
            }
        }
    }

    impl Drop for EnvVarResetter {
        fn drop(&mut self) {
            if let Some(old_value) = self.value.as_ref() {
                std::env::set_var(&self.key, old_value);
            } else {
                std::env::remove_var(&self.key);
            }
        }
    }
}
