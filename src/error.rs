#[cfg(feature = "git-index")]
pub use git2::Error as Git2Error;
pub use serde_json::Error as SerdeJsonError;
use std::{fmt, io};
pub use toml::de::Error as TomlDeError;

/// Oops
#[derive(Debug)]
pub enum Error {
    /// git2 library failed. If problems persist, delete `~/.cargo/registry`
    #[cfg(feature = "git-index")]
    Git2(Git2Error),
    /// `gix` crate failed. If problems persist, delete `~/.cargo/registry`
    #[cfg(feature = "git-index")]
    Git(GixError),
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
            Self::Git2(e) => fmt::Display::fmt(&e, f),
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
            Self::Git2(e) => Some(e),
            #[cfg(feature = "git-index")]
            Self::Git(e) => Some(e),
            Self::Io(e) => Some(e),
            _ => None,
        }
    }
}

/// Any error produced by `gix` or the `gix-*` family of crates.
#[derive(Debug, thiserror::Error)]
#[allow(missing_docs)]
#[cfg(feature = "git-index")]
pub enum GixError {
    #[error(transparent)]
    HeadCommit(#[from] gix::reference::head_commit::Error),
    #[error(transparent)]
    TreeOfCommit(#[from] gix::object::commit::Error),
    #[error(transparent)]
    DecodeObject(#[from] gix::objs::decode::Error),
    #[error(transparent)]
    FindExistingObject(#[from] gix::object::find::existing::Error),
    #[error(transparent)]
    IntoObjectKind(#[from] gix::object::try_into::Error)
}

#[cfg(feature = "git-index")]
impl From<Git2Error> for Error {
    #[cold]
    fn from(e: Git2Error) -> Self {
        Self::Git2(e)
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

/// Unknown error from [`crate::Index::git2_crates_parallel`]
#[derive(Debug)]
pub struct CratesIterError;

impl std::error::Error for CratesIterError {}

impl fmt::Display for CratesIterError {
    #[cold]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("error while iterating git repository")
    }
}
