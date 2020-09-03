use std::{fmt, io::Error as IoErr};
use git2::Error as GitErr;

#[derive(Debug)]
pub enum Error {
    Git(GitErr),
    Url(String),
    Io(IoErr),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Git(e) => fmt::Display::fmt(&e, f),
            Self::Url(u) => f.write_str(&u),
            Self::Io(e) => fmt::Display::fmt(&e, f),
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Git(e) => Some(e),
            Self::Io(e) => Some(e),
            _ => None,
        }
    }
}

impl From<GitErr> for Error {
    fn from(e: GitErr) -> Self {
        Self::Git(e)
    }
}