use git2::Error as GitErr;
use serde_json::Error as SerdeJsonError;
use std::{fmt, io::Error as IoErr};

/// Oops
#[derive(Debug)]
pub enum Error {
    /// git2 library failed. If problems persist, delete `~/.cargo/registry`
    Git(GitErr),
    /// `Index::from_url` got a bogus URL
    Url(String),
    /// Filesystem error
    Io(IoErr),
    /// If this happens, the registry is seriously corrupted. Delete `~/.cargo/registry`.
    Json(SerdeJsonError),
}

impl fmt::Display for Error {
    #[cold]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Git(e) => fmt::Display::fmt(&e, f),
            Self::Url(u) => f.write_str(u),
            Self::Io(e) => fmt::Display::fmt(&e, f),
            Self::Json(e) => fmt::Display::fmt(&e, f),
        }
    }
}

impl std::error::Error for Error {
    #[cold]
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Git(e) => Some(e),
            Self::Io(e) => Some(e),
            _ => None,
        }
    }
}

impl From<GitErr> for Error {
    #[cold]
    fn from(e: GitErr) -> Self {
        Self::Git(e)
    }
}

#[test]
fn error_is_send() {
    fn is_send<T: Send>() {}
    is_send::<Error>();
}

/// Unknown error from `crates_parallel`
#[derive(Debug)]
pub struct CratesIterError;

impl std::error::Error for CratesIterError {}

impl fmt::Display for CratesIterError {
    #[cold]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("error while iterating git repository")
    }
}
