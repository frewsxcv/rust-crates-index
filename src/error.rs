#[cfg(feature = "git-index")]
pub use git2::Error as GitError;
pub use serde_json::Error as SerdeJsonError;
use std::{fmt, io};
pub use toml::de::Error as TomlDeError;

/// Oops
#[derive(Debug)]
pub enum Error {
    /// git2 library failed. If problems persist, delete `~/.cargo/registry`
    #[cfg(feature = "git-index")]
    Git(GitError),
    /// `Index::from_url` got a bogus URL
    Url(String),
    /// Filesystem error
    Io(io::Error),
    /// If this happens, the registry is seriously corrupted. Delete `~/.cargo/registry`.
    Json(SerdeJsonError),
    /// Cargo config.toml deserialization error
    Toml(TomlDeError),
}

impl fmt::Display for Error {
    #[cold]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            #[cfg(feature = "git-index")]
            Self::Git(e) => fmt::Display::fmt(&e, f),
            Self::Url(u) => f.write_str(u),
            Self::Io(e) => fmt::Display::fmt(&e, f),
            Self::Json(e) => fmt::Display::fmt(&e, f),
            Self::Toml(e) => fmt::Display::fmt(&e, f),
        }
    }
}

impl std::error::Error for Error {
    #[cold]
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            #[cfg(feature = "git-index")]
            Self::Git(e) => Some(e),
            Self::Io(e) => Some(e),
            _ => None,
        }
    }
}

#[cfg(feature = "git-index")]
impl From<GitError> for Error {
    #[cold]
    fn from(e: GitError) -> Self {
        Self::Git(e)
    }
}

impl From<io::Error> for Error {
    #[cold]
    fn from(e: io::Error) -> Self {
        Self::Io(e)
    }
}

#[test]
fn error_is_send() {
    fn is_send<T: Send>() {}
    is_send::<Error>();
}

/// Unknown error from [`crate::Index::crates_parallel`]
#[derive(Debug)]
pub struct CratesIterError;

impl std::error::Error for CratesIterError {}

impl fmt::Display for CratesIterError {
    #[cold]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("error while iterating git repository")
    }
}
