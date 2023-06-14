use crate::{config, dirs, Error, Index, SparseIndex};
use std::{io, path::Path};

/// A unified interface to either a git or sparse HTTP registry index
pub enum RegistryIndex {
    /// The (formerly as of 1.70.0) standard git based registry index
    Git(Index),
    /// An HTTP sparse index
    Sparse(SparseIndex),
}

impl RegistryIndex {
    /// Opens the default index for crates.io, depending on the configuration and
    /// version of cargo
    ///
    /// 1. Determines if the crates.io registry has been replaced
    /// 2. Determines the protocol explicitly configured by the user <https://doc.rust-lang.org/cargo/reference/config.html#registriescrates-ioprotocol>
    /// 3. Detects the version of cargo if not specified and uses that to determine the appropriate default
    pub fn crates_io(
        root: Option<&Path>,
        cargo_home: Option<&Path>,
        cargo_version: Option<&str>,
    ) -> Result<Self, Error> {
        // If the crates.io registry has been replaced it doesn't matter what
        // the protocol for it has been changed to
        if let Some(replacement) = config::get_crates_io_replacement(root, cargo_home)? {
            let (path, canonical) = dirs::get_index_details(&replacement, cargo_home)?;

            return if canonical.starts_with("sparse+http") {
                Ok(Self::Sparse(SparseIndex::at_path(path, canonical)))
            } else {
                Index::with_path(path, canonical).map(Self::Git)
            };
        }

        let sparse_index = match std::env::var("CARGO_REGISTRIES_CRATES_IO_PROTOCOL")
            .ok()
            .as_deref()
        {
            Some("sparse") => true,
            Some("git") => false,
            _ => {
                let sparse_index = config::read_cargo_config(root, cargo_home, |config| {
                    config
                        .get("registries")
                        .and_then(|v| v.get("crates-io"))
                        .and_then(|v| v.get("protocol"))
                        .and_then(|v| v.as_str())
                        .and_then(|v| match v {
                            "sparse" => Some(true),
                            "git" => Some(false),
                            _ => None,
                        })
                })?;

                match sparse_index {
                    Some(si) => si,
                    None => {
                        let semver = match cargo_version {
                            Some(v) => std::borrow::Cow::Borrowed(v),
                            None => Self::cargo_version()?.into(),
                        };

                        // Note this would need to change if there was ever a major version
                        // bump of cargo, but that's unlikely (famous last words)
                        let minor = semver.split('.').nth(1).ok_or_else(|| {
                            io::Error::new(
                                io::ErrorKind::InvalidData,
                                "cargo semver was in an invalid format",
                            )
                        })?;

                        let minor: u32 = minor
                            .parse()
                            .map_err(|err| io::Error::new(io::ErrorKind::InvalidData, err))?;

                        minor >= 70
                    }
                }
            }
        };

        let url = if sparse_index {
            crate::CRATES_IO_HTTP_INDEX
        } else {
            crate::INDEX_GIT_URL
        };

        let (path, canonical) = dirs::get_index_details(url, cargo_home)?;

        if sparse_index {
            Ok(Self::Sparse(SparseIndex::at_path(path, canonical)))
        } else {
            Index::with_path(path, canonical).map(Self::Git)
        }
    }

    /// Retrieves the current version of cargo being used
    pub fn cargo_version() -> Result<String, Error> {
        let mut cargo = std::process::Command::new(
            std::env::var_os("CARGO")
                .as_deref()
                .unwrap_or_else(|| std::ffi::OsStr::new("cargo")),
        );

        cargo.arg("-V");
        cargo.stdout(std::process::Stdio::piped());

        let output = cargo.output()?;
        if !output.status.success() {
            return Err(io::Error::new(
                io::ErrorKind::Other,
                "failed to request cargo version information",
            )
            .into());
        }

        let stdout = String::from_utf8(output.stdout)
            .map_err(|err| io::Error::new(io::ErrorKind::InvalidData, err))?;

        let semver = stdout.split(' ').nth(1).ok_or_else(|| {
            io::Error::new(
                io::ErrorKind::InvalidData,
                "cargo version information was in an invalid format",
            )
        })?;

        Ok(semver.to_owned())
    }

    /// Retrieves the index metadata for the specified crate name
    #[inline]
    pub fn krate(&self, name: &str) -> Result<crate::Crate, Error> {
        match self {
            Self::Git(index) => index.crate_(name).ok_or_else(|| {
                io::Error::new(
                    io::ErrorKind::NotFound,
                    format!("unable to locate crate '{name}'"),
                )
                .into()
            }),
            Self::Sparse(index) => index.crate_from_cache(name),
        }
    }

    /// If using the sparse index, returns the url that can be used to fetch
    /// the index entry for the specified crate
    #[inline]
    pub fn crate_url(&self, name: &str) -> Option<String> {
        // MSRV is 1.60.0 :(
        //let Self::Sparse(index) = self else { return None; };
        match self {
            Self::Git(_) => None,
            Self::Sparse(index) => index.crate_url(name),
        }
    }
}

impl From<Index> for RegistryIndex {
    #[inline]
    fn from(index: Index) -> Self {
        Self::Git(index)
    }
}

impl From<SparseIndex> for RegistryIndex {
    #[inline]
    fn from(index: SparseIndex) -> Self {
        Self::Sparse(index)
    }
}

#[cfg(test)]
mod test {
    use crate::RegistryIndex;

    // Current stable is 1.70.0, and CI only runs beta and nightly, so...just assume
    // this is fine. If this tests fail you are running an old cargo :(

    #[test]
    fn gets_cargo_version() {
        assert!(RegistryIndex::cargo_version().unwrap().as_str() >= "1.70.0");
    }

    #[test]
    fn opens_sparse() {
        assert!(std::env::var_os("CARGO_REGISTRIES_CRATES_IO_PROTOCOL").is_none());
        assert!(matches!(
            RegistryIndex::crates_io(None, None, None).unwrap(),
            RegistryIndex::Sparse(_)
        ));
    }
}
