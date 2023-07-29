// Copyright 2015 Corey Farwell
// Copyright 2015 Contributors of github.com/huonw/crates.io-graph
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//	http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! Library for retrieving and interacting with the
//! [crates.io git index](https://github.com/rust-lang/crates.io-index).
//!
//! ## Examples
//!
//! ### Getting information about a single crate
//!
//! ```rust
//! # #[cfg(all(not(debug_assertions), feature = "git-index"))]
//! # {
//! let index = crates_index::GitIndex::new_cargo_default()?;
//! let serde_crate = index.crate_("serde").expect("you should handle errors here");
//! println!("Serde is at v{}", serde_crate.highest_normal_version().unwrap().version());
//! # }
//! # Ok::<_, crates_index::Error>(())
//! ```
//!
//! ### Iterating over *all* crates in the index
//!
//! ```rust
//! # #[cfg(all(not(debug_assertions), feature = "parallel", feature = "git-index"))]
//! # {
//! let index = crates_index::GitIndex::new_cargo_default()?;
//! for crate_ in index.crates() {
//!    let latest = crate_.most_recent_version();
//!    println!("crate name: {}", latest.name());
//!    println!("most recently released version: {}", latest.version());
//! }
//!
//! // or faster:
//! use rayon::prelude::*;
//! index.crates_parallel().for_each(|crate_| {
//!     /* etc. */
//! });
//!
//! # }
//! # Ok::<_, crates_index::Error>(())
//! ```
//!
//! ### Getting most recently published or yanked crates 
//!
//! ```rust
//! # #[cfg(feature = "git-index")]
//! # {
//! let index = crates_index::GitIndex::new_cargo_default()?;
//!
//! for c in index.changes()?.take(20) {
//!     let c = c?;
//!     println!("{} has changed in the index commit {}", c.crate_name(), c.commit_hex());
//! }
//!
//! # }
//! # Ok::<_, crates_index::Error>(())
//! ```
//! 
//! ## Auto-cloning and parallelism
//! 
//! When using any means of instantiating the [`GitIndex`] type, we  will 
//! clone the default crates index (or the given one) if it no git
//! repository is present at the destination path.
//! 
//! In order to protect from parallel operations of this kind, a 
//! file-based lock is used. When interrupting the program with `Ctrl + C`,
//! by default the program will be aborted which won't run destructors.
//! This will cause the file lock to be stranded, causing all future operations
//! to fail.
//! 
//! To prevent this issue, the application must integrate with the
//! [`gix-tempfile` signal handler](https://docs.rs/gix-tempfile/latest/gix_tempfile/#initial-setup),
//! which allows locks to be deleted when typical signals are received.
//!
//! ## Git Repository Performance
//!
//! By default, `gix` is compiled with `max-performance-safe`, which maximizes support for compilation environments but which 
//! may be slower as it uses a pure-Rust Zlib implementation.
//! To get best possible performance, use the `git-index-performance` feature toggle.
//! 
//! ## Using `rustls` instead of `openssl` when using the `https` feature in applications
//! 
//! When using the `https` feature, a choice will be made for you that involves selecting the `curl` backend for making
//! the `https` protocol available. As using a different backend isn't additive, as cargo features should be, one will have
//! to resort to the following.
//! 
//! * Change the `crates-index` dependency to not use any default features with `default-features = false`, and turn on
//!   `features = ["git-index", …(everything else *but* "https")]`
//! * Add the `gix` dependency with `default-features = false` and `features = ["blocking-http-transport-reqwest-rust-tls"]`.
//!   Consider renaming the crate to `gix-for-configuration-only = { package = "gix", … }` to make the intend clear.
//! 
//! Please note that this should only be done in application manifests, who have the final say over the protocol and backend choices.
//! ## Feature Flags
#![cfg_attr(
    feature = "document-features",
    cfg_attr(doc, doc = ::document_features::document_features!())
)]
#![cfg_attr(docsrs, feature(doc_cfg, doc_auto_cfg))]
#![forbid(unsafe_code)]
#![deny(rust_2018_compatibility, missing_docs)]
use std::path::{PathBuf, Path};

/// Wrapper around managing the crates.io-index git repository
///
/// Uses a "bare" git index that fetches files directly from the repo instead of local checkout.
/// Uses Cargo's cache.
#[cfg(feature = "git-index")]
pub struct GitIndex {
    path: std::path::PathBuf,
    url: String,

    pub(crate) repo: gix::Repository,
    pub(crate) head_commit: gix::ObjectId,
    head_commit_hex: String,
}

///
#[cfg(feature = "git-index")]
pub mod git;

mod config;
pub use config::IndexConfig;

mod dedupe;
mod dirs;

/// Re-exports in case you want to inspect specific error details
pub mod error;
#[doc(hidden)]
pub use error::{Error};
#[doc(hidden)]
#[cfg(feature = "parallel")]
pub use error::{CratesIterError};

/// Wrapper around managing a sparse HTTP index, re-using Cargo's local disk caches.
///
/// Currently it only uses local Cargo cache, and does not access the network in any way.
pub struct SparseIndex {
    path: PathBuf,
    url: String,
}

///
pub mod sparse;


mod types;
pub use types::{Crate, Version, Dependency, DependencyKind};


pub(crate) fn split(haystack: &[u8], needle: u8) -> impl Iterator<Item = &[u8]> + '_ {
    struct Split<'a> {
        haystack: &'a [u8],
        needle: u8,
    }

    impl<'a> Iterator for Split<'a> {
        type Item = &'a [u8];

        #[inline]
        fn next(&mut self) -> Option<&'a [u8]> {
            if self.haystack.is_empty() {
                return None;
            }
            let (ret, remaining) = match memchr::memchr(self.needle, self.haystack) {
                Some(pos) => (&self.haystack[..pos], &self.haystack[pos + 1..]),
                None => (self.haystack, &[][..]),
            };
            self.haystack = remaining;
            Some(ret)
        }
    }

    Split { haystack, needle }
}

#[cfg(unix)]
fn path_max_byte_len(path: &Path) -> usize {
    use std::os::unix::prelude::OsStrExt;
    path.as_os_str().as_bytes().len()
}

#[cfg(not(unix))]
fn path_max_byte_len(path: &Path) -> usize {
    path.to_str().map_or(0, |p| p.len())
}
