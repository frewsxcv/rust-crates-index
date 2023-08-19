#![allow(clippy::result_large_err)]

#[cfg(all(doc, feature = "git"))]
use crate::GitIndex;

/// The default URL of the crates.io index for use with git, see [`GitIndex::with_path`]
pub const URL: &str = "https://github.com/rust-lang/crates.io-index";

///
#[cfg(feature = "git")]
mod changes;
#[cfg(feature = "git")]
pub use changes::Changes;

#[cfg(feature = "git")]
mod config;

#[cfg(feature = "git")]
mod impl_;
#[cfg(feature = "git")]
use impl_::fetch_remote;
#[cfg(feature = "git")]
pub use impl_::{Change, Crates};

#[cfg(test)]
#[cfg(feature = "git-https")]
mod test;
