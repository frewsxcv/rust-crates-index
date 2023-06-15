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
//! let index = crates_index::Index::new_cargo_default()?;
//! let serde_crate = index.crate_("serde").expect("you should handle errors here");
//! println!("Serde is at v{}", serde_crate.highest_normal_version().unwrap().version());
//! # Ok::<_, crates_index::Error>(())
//! ```
//!
//! ### Iterating over *all* crates in the index
//!
//! ```rust
//! let index = crates_index::Index::new_cargo_default()?;
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
//! # Ok::<_, crates_index::Error>(())
//! ```
#![forbid(unsafe_code)]
#![deny(missing_docs)]

use dedupe::DedupeContext;
use git2::CredentialType;
use git2::{Config, Cred, CredentialHelper, RemoteCallbacks};
use semver::Version as SemverVersion;
use serde_derive::{Deserialize, Serialize};
use smol_str::SmolStr;
use std::collections::HashMap;
use std::io;
use std::path::Path;
use std::sync::Arc;

mod bare_index;
mod dedupe;
mod dirs;
/// Re-exports in case you want to inspect specific error details
pub mod error;
mod sparse_index;

pub use bare_index::Crates;
pub use bare_index::Index;
pub use bare_index::INDEX_GIT_URL;

#[doc(hidden)]
pub use error::CratesIterError;
pub use error::Error;
pub use sparse_index::Index as SparseIndex;
pub use sparse_index::CRATES_IO_HTTP_INDEX;

/// A single version of a crate (package) published to the index
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Version {
    name: SmolStr,
    vers: SmolStr,
    deps: Arc<[Dependency]>,
    features: Arc<HashMap<String, Vec<String>>>,
    /// It's wrapped in `Option<Box>` to reduce size of the struct when the field is unused (i.e. almost always)
    /// <https://rust-lang.github.io/rfcs/3143-cargo-weak-namespaced-features.html#index-changes>
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[allow(clippy::box_collection)]
    features2: Option<Box<HashMap<String, Vec<String>>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    links: Option<Box<SmolStr>>,
    #[serde(default)]
    rust_version: Option<SmolStr>,
    #[serde(with = "hex")]
    cksum: [u8; 32],
    #[serde(default)]
    yanked: bool,
}

impl Version {
    /// Name of the crate
    #[inline]
    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Name of this version
    #[inline]
    #[must_use]
    pub fn version(&self) -> &str {
        &self.vers
    }

    /// Dependencies for this version
    #[inline]
    #[must_use]
    pub fn dependencies(&self) -> &[Dependency] {
        &self.deps
    }

    /// Checksum of the package for this version
    ///
    /// SHA256 of the .crate file
    #[inline]
    #[must_use]
    pub fn checksum(&self) -> &[u8; 32] {
        &self.cksum
    }

    /// Explicit features this crate has. This list is not exhaustive,
    /// because any optional dependency becomes a feature automatically.
    ///
    /// `default` is a special feature name for implicitly enabled features.
    #[inline]
    #[must_use]
    pub fn features(&self) -> &HashMap<String, Vec<String>> {
        &self.features
    }

    /// Exclusivity flag. If this is a sys crate, it informs it
    /// conflicts with any other crate with the same links string.
    ///
    /// It does not involve linker or libraries in any way.
    #[inline]
    #[must_use]
    pub fn links(&self) -> Option<&str> {
        self.links.as_ref().map(|s| s.as_str())
    }

    /// Whether this version was [yanked](http://doc.crates.io/crates-io.html#cargo-yank) from the
    /// index
    #[inline]
    #[must_use]
    pub fn is_yanked(&self) -> bool {
        self.yanked
    }

    /// Required version of rust
    ///
    /// Corresponds to `package.rust-version`.
    ///
    /// Added in 2023 (see <https://github.com/rust-lang/crates.io/pull/6267>),
    /// can be `None` if published before then or if not set in the manifest.
    #[inline]
    #[must_use]
    pub fn rust_version(&self) -> Option<&str> {
        self.rust_version.as_deref()
    }

    /// Where to find crate tarball
    #[must_use]
    pub fn download_url(&self, index: &IndexConfig) -> Option<String> {
        index.download_url(&self.name, &self.vers)
    }
}

/// A single dependency of a specific crate version
#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq, Hash)]
pub struct Dependency {
    name: SmolStr,
    req: SmolStr,
    /// Double indirection to remove size from this struct, since the features are rarely set
    features: Box<Box<[String]>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    package: Option<Box<SmolStr>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    kind: Option<DependencyKind>,
    #[serde(skip_serializing_if = "Option::is_none")]
    target: Option<Box<SmolStr>>,
    optional: bool,
    default_features: bool,
}

impl Dependency {
    /// Dependency's arbitrary nickname (it may be an alias). Use [`Dependency::crate_name`] for actual crate name.
    #[inline]
    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Semver version pattern
    #[inline]
    #[must_use]
    pub fn requirement(&self) -> &str {
        &self.req
    }

    /// Features unconditionally enabled when using this dependency,
    /// in addition to [`Dependency::has_default_features`] and features enabled through
    /// parent crate's feature list.
    #[inline]
    #[must_use]
    pub fn features(&self) -> &[String] {
        &self.features
    }

    /// If it's optional, it implies a feature of its [`Dependency::name`], and can be enabled through
    /// the crate's features.
    #[inline]
    #[must_use]
    pub fn is_optional(&self) -> bool {
        self.optional
    }

    /// If `true` (default), enable `default` feature of this dependency
    #[inline]
    #[must_use]
    pub fn has_default_features(&self) -> bool {
        self.default_features
    }

    /// This dependency is only used when compiling for this `cfg` expression
    #[inline]
    #[must_use]
    pub fn target(&self) -> Option<&str> {
        self.target.as_ref().map(|s| s.as_str())
    }

    /// Dev or not
    #[inline]
    #[must_use]
    pub fn kind(&self) -> DependencyKind {
        self.kind.unwrap_or_default()
    }

    /// Set if dependency's crate name is different from the `name` (alias)
    #[inline]
    #[must_use]
    pub fn package(&self) -> Option<&str> {
        self.package.as_ref().map(|s| s.as_str())
    }

    /// Returns the name of the crate providing the dependency.
    /// This is equivalent to `name()` unless `self.package()`
    /// is not `None`, in which case it's equal to `self.package()`.
    ///
    /// Basically, you can define a dependency in your `Cargo.toml`
    /// like this:
    ///
    /// ```toml
    /// serde_lib = {version = "1", package = "serde"}
    /// ```
    ///
    /// ...which means that it uses the crate `serde` but imports
    /// it under the name `serde_lib`.
    #[inline]
    #[must_use]
    pub fn crate_name(&self) -> &str {
        match self.package {
            Some(ref s) => s,
            None => self.name(),
        }
    }
}

/// Section in which this dependency was defined
#[derive(Debug, Copy, Clone, Serialize, Deserialize, Eq, PartialEq, Hash)]
#[serde(rename_all = "lowercase")]
pub enum DependencyKind {
    /// Used at run time
    Normal,
    /// Not fetched and not used, except for when used direclty in a workspace
    Dev,
    /// Used at build time, not available at run time
    Build,
}

impl Default for DependencyKind {
    fn default() -> Self {
        Self::Normal
    }
}

fn fetch_opts<'cb>() -> git2::FetchOptions<'cb> {
    let mut proxy_opts = git2::ProxyOptions::new();
    proxy_opts.auto();
    let mut fetch_opts = git2::FetchOptions::new();
    fetch_opts.proxy_options(proxy_opts);

    let mut remote_callbacks = RemoteCallbacks::new();
    remote_callbacks.credentials(|url, username_from_url, allowed_types| {
        let config = Config::open_default()?;

        if allowed_types.contains(CredentialType::USER_PASS_PLAINTEXT) {
            if let Some((username, password)) = CredentialHelper::new(url)
                .config(&config)
                .username(username_from_url)
                .execute()
            {
                let cred = Cred::userpass_plaintext(&username, &password)?;
                return Ok(cred);
            }
        }

        #[cfg(feature = "ssh")]
        if allowed_types.contains(CredentialType::SSH_KEY) {
            if let Some(username) = username_from_url {
                if let Ok(cred) = Cred::ssh_key_from_agent(username) {
                    return Ok(cred);
                }
            }
        }

        Err(git2::Error::from_str(
            "failed to acquire appropriate credentials from local configuration",
        ))
    });
    fetch_opts.remote_callbacks(remote_callbacks);

    fetch_opts
}

fn crate_prefix(accumulator: &mut String, crate_name: &str, separator: char) -> Option<()> {
    match crate_name.len() {
        0 => return None,
        1 => accumulator.push('1'),
        2 => accumulator.push('2'),
        3 => {
            accumulator.push('3');
            accumulator.push(separator);
            accumulator.extend(crate_name.as_bytes().get(0..1)?.iter().map(|c| c.to_ascii_lowercase() as char));
        }
        _ => {
            accumulator.extend(crate_name.as_bytes().get(0..2)?.iter().map(|c| c.to_ascii_lowercase() as char));
            accumulator.push(separator);
            accumulator.extend(crate_name.as_bytes().get(2..4)?.iter().map(|c| c.to_ascii_lowercase() as char));
        }
    };
    Some(())
}

fn crate_name_to_relative_path(crate_name: &str) -> Option<String> {
    let mut rel_path = String::with_capacity(crate_name.len() + 6);
    crate_prefix(&mut rel_path, crate_name, std::path::MAIN_SEPARATOR)?;
    rel_path.push(std::path::MAIN_SEPARATOR);
    rel_path.extend(crate_name.as_bytes().iter().map(|c| c.to_ascii_lowercase() as char));

    Some(rel_path)
}

/// A whole crate with all its versions
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Crate {
    versions: Box<[Version]>,
}

impl Crate {
    /// Parse crate file from in-memory JSON data
    #[inline(never)]
    pub(crate) fn from_slice_with_context(mut bytes: &[u8], dedupe: &mut DedupeContext) -> io::Result<Crate> {
        // Trim last newline
        while bytes.last() == Some(&b'\n') {
            bytes = &bytes[..bytes.len() - 1];
        }

        #[inline(always)]
        fn is_newline(&c: &u8) -> bool {
            c == b'\n'
        }
        let num_versions = bytes.split(is_newline).count();
        let mut versions = Vec::with_capacity(num_versions);
        for line in bytes.split(is_newline) {
            let mut version: Version = serde_json::from_slice(line)
                .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;

            if let Some(features2) = version.features2.take() {
                if let Some(f1) = Arc::get_mut(&mut version.features) {
                    for (key, mut val) in features2.into_iter() {
                        f1.entry(key).or_insert_with(Vec::new).append(&mut val);
                    }
                }
            }

            // Many versions have identical dependencies and features
            dedupe.deps(&mut version.deps);
            dedupe.features(&mut version.features);

            versions.push(version);
        }
        if versions.is_empty() {
            return Err(io::ErrorKind::UnexpectedEof.into());
        }
        debug_assert_eq!(versions.len(), versions.capacity());
        Ok(Crate {
            versions: versions.into_boxed_slice(),
        })
    }

    /// Parse crate index entry from a .cache file, this can fail for a number of reasons
    ///
    /// 1. There is no entry for this crate
    /// 2. The entry was created with an older commit and might be outdated
    /// 3. The entry is a newer version than what can be read, would only
    /// happen if a future version of cargo changed the format of the cache entries
    /// 4. The cache entry is malformed somehow
    #[inline(never)]
    pub(crate) fn from_cache_slice(bytes: &[u8], index_version: &str) -> io::Result<Crate> {
        const CURRENT_CACHE_VERSION: u8 = 1;

        // See src/cargo/sources/registry/index.rs
        let (&first_byte, rest) = bytes.split_first().ok_or(io::ErrorKind::UnexpectedEof)?;

        if first_byte != CURRENT_CACHE_VERSION {
            return Err(io::Error::new(io::ErrorKind::Unsupported, "looks like a different Cargo's cache"));
        }

        let mut iter = split(rest, 0);
        let update = iter.next().ok_or(io::ErrorKind::UnexpectedEof)?;
        if update != index_version.as_bytes() {
            return Err(io::Error::new(io::ErrorKind::Other,
                format!("cache out of date: current index ({index_version}) != cache ({})", String::from_utf8_lossy(update))));
        }
        Self::from_version_entries_iter(iter)
    }

    #[inline(never)]
    pub(crate) fn from_sparse_cache_slice(bytes: &[u8]) -> io::Result<Crate> {
        const CURRENT_CACHE_VERSION: u8 = 3;
        const CURRENT_INDEX_FORMAT_VERSION: u32 = 2;

        let (&first_byte, rest) = bytes.split_first().ok_or(io::ErrorKind::UnexpectedEof)?;

        if first_byte != CURRENT_CACHE_VERSION {
            return Err(io::Error::new(io::ErrorKind::Unsupported, "looks like a different Cargo's cache"));
        }

        let index_v_bytes = rest.get(..4).ok_or(io::ErrorKind::UnexpectedEof)?;
        let index_v = u32::from_le_bytes(index_v_bytes.try_into().unwrap());
        if index_v != CURRENT_INDEX_FORMAT_VERSION {
            return Err(io::Error::new(io::ErrorKind::Unsupported,
                format!("wrong index format version: {index_v} (expected {CURRENT_INDEX_FORMAT_VERSION}))")));
        }
        let rest = &rest[4..];

        let mut iter = split(rest, 0);
        if iter.next().is_none() {
            return Err(io::Error::new(io::ErrorKind::InvalidData, "malformed cache (missing git sha)"));
        }

        Self::from_version_entries_iter(iter)
    }

    pub(crate) fn from_version_entries_iter<'a, I: Iterator<Item = &'a [u8]> + 'a>(
        mut iter: I,
    ) -> io::Result<Crate> {
        let mut versions = Vec::new();

        // Each entry is a tuple of (semver, version_json)
        while let Some(_version) = iter.next() {
            let version_slice = iter.next().ok_or(io::ErrorKind::UnexpectedEof)?;
            let version: Version = serde_json::from_slice(version_slice)
                .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
            versions.push(version);
        }

        Ok(Self {
            versions: versions.into_boxed_slice(),
        })
    }

    /// All versions of this crate sorted chronologically by date originally published
    ///
    /// Warning: may be yanked or duplicate
    #[inline]
    #[must_use]
    pub fn versions(&self) -> &[Version] {
        &self.versions
    }

    /// The highest version as per semantic versioning specification
    ///
    /// Warning: may be pre-release or yanked
    #[must_use]
    pub fn highest_version(&self) -> &Version {
        self.versions
            .iter()
            .max_by_key(|v| SemverVersion::parse(&v.vers).ok())
            // Safety: Versions inside the index will always adhere to
            // semantic versioning. If a crate is inside the index, at
            // least one version is available.
            .unwrap()
    }

    /// Returns crate version with the highest version number according to semver,
    /// but excludes pre-release and yanked versions.
    ///
    /// 0.x.y versions are included.
    ///
    /// May return `None` if the crate has only pre-release or yanked versions.
    #[must_use]
    pub fn highest_normal_version(&self) -> Option<&Version> {
        self.versions
            .iter()
            .filter(|v| !v.is_yanked())
            .filter_map(|v| Some((v, SemverVersion::parse(&v.vers).ok()?)))
            .filter(|(_, sem)| sem.pre.is_empty())
            .max_by(|a, b| a.1.cmp(&b.1))
            .map(|(v, _)| v)
    }

    /// Crate's unique registry name. Case-sensitive, mostly.
    #[inline]
    #[must_use]
    pub fn name(&self) -> &str {
        self.versions[0].name()
    }

    /// The last release by date, even if it's yanked or less than highest version.
    ///
    /// See [`Crate::highest_normal_version`]
    #[inline]
    #[must_use]
    pub fn most_recent_version(&self) -> &Version {
        &self.versions[self.versions.len() - 1]
    }

    /// First version ever published. May be yanked.
    ///
    /// It is not guaranteed to be the lowest version number.
    #[inline]
    #[must_use]
    pub fn earliest_version(&self) -> &Version {
        &self.versions[0]
    }

    /// Unconstrained Latest version
    ///
    /// Warning: may not be the highest version and may be yanked
    #[cold]
    #[doc(hidden)]
    #[deprecated(note = "use most_recent_version")]
    #[must_use]
    pub fn latest_version(&self) -> &Version {
        self.most_recent_version()
    }

    /// Returns the highest version as per semantic versioning specification,
    /// filtering out versions with pre-release identifiers.
    ///
    /// Warning: may be yanked
    #[cold]
    #[doc(hidden)]
    #[deprecated(note = "use highest_normal_version")]
    #[must_use]
    pub fn highest_stable_version(&self) -> Option<&Version> {
        self.versions
            .iter()
            .filter_map(|v| Some((v, SemverVersion::parse(&v.vers).ok()?)))
            .filter(|(_, sem)| sem.pre.is_empty())
            .max_by(|a, b| a.1.cmp(&b.1))
            .map(|(v, _)| v)
    }

    /// Parse an index file with all of crate's versions.
    ///
    /// The file must contain at least one version.
    #[inline]
    pub fn new<P: AsRef<Path>>(index_path: P) -> io::Result<Crate> {
        let lines = std::fs::read(index_path)?;
        Self::from_slice(&lines)
    }

    /// Parse crate file from in-memory JSON-lines data
    #[inline]
    pub fn from_slice(bytes: &[u8]) -> io::Result<Crate> {
        let mut dedupe = DedupeContext::new();
        Self::from_slice_with_context(bytes, &mut dedupe)
    }
}

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

/// Global configuration of an index, reflecting the [contents of config.json](https://doc.rust-lang.org/cargo/reference/registries.html#index-format).
#[derive(Clone, Debug, Deserialize)]
pub struct IndexConfig {
    /// Pattern for creating download URLs. Use [`IndexConfig::download_url`] instead.
    pub dl: String,
    /// Base URL for publishing, etc.
    pub api: Option<String>,
}

impl IndexConfig {
    /// Get the URL from where the specified package can be downloaded.
    /// This method assumes the particular version is present in the registry,
    /// and does not verify that it is.
    #[must_use]
    pub fn download_url(&self, name: &str, version: &str) -> Option<String> {
        if !self.dl.contains("{crate}")
            && !self.dl.contains("{version}")
            && !self.dl.contains("{prefix}")
            && !self.dl.contains("{lowerprefix}")
        {
            let mut new = String::with_capacity(self.dl.len() + name.len() + version.len() + 10);
            new.push_str(&self.dl);
            new.push('/');
            new.push_str(name);
            new.push('/');
            new.push_str(version);
            new.push_str("/download");
            Some(new)
        } else {
            let mut prefix = String::with_capacity(5);
            crate_prefix(&mut prefix, name, '/')?;
            Some(
                self.dl
                    .replace("{crate}", name)
                    .replace("{version}", version)
                    .replace("{prefix}", &prefix)
                    .replace("{lowerprefix}", &prefix.to_ascii_lowercase()),
            )
        }
    }
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

#[cfg(test)]
mod test {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn sizes() {
        assert!(std::mem::size_of::<Version>() <= 152);
        assert!(std::mem::size_of::<Crate>() <= 16);
        assert!(std::mem::size_of::<Dependency>() <= 80);
    }

    #[test]
    fn semver() {
        let c = Crate::from_slice(r#"{"vers":"1.0.0", "name":"test", "deps":[], "features":{}, "cksum":"1234567890123456789012345678901234567890123456789012345678901234", "yanked":false}
            {"vers":"1.2.0-alpha.1", "name":"test", "deps":[], "features":{}, "cksum":"1234567890123456789012345678901234567890123456789012345678901234", "yanked":false}
            {"vers":"1.0.1", "name":"test", "deps":[], "features":{}, "cksum":"1234567890123456789012345678901234567890123456789012345678901234", "yanked":false}"#.as_bytes()).unwrap();
        assert_eq!(c.most_recent_version().version(), "1.0.1");
        assert_eq!(c.highest_version().version(), "1.2.0-alpha.1");
        assert_eq!(c.highest_normal_version().unwrap().version(), "1.0.1");
    }

    #[test]
    fn test_dependencies() {
        let index = Index::new_cargo_default().unwrap();

        let crate_ = index
            .crate_("sval")
            .expect("Could not find the crate libnotify in the index");
        let _ = format!("supports debug {crate_:?}");

        let version = crate_
            .versions()
            .iter()
            .find(|v| v.version() == "0.0.1")
            .expect("Version 0.0.1 of sval does not exist?");
        let dep_with_package_name = version
            .dependencies()
            .iter()
            .find(|d| d.name() == "serde_lib")
            .expect("sval does not have expected dependency?");
        assert_ne!(
            dep_with_package_name.name(),
            dep_with_package_name.package().unwrap()
        );
        assert_eq!(
            dep_with_package_name.crate_name(),
            dep_with_package_name.package().unwrap()
        );
    }

    #[test]
    fn test_cargo_default_updates() {
        let mut index = Index::new_cargo_default().unwrap();
        index
            .update()
            .map_err(|e| {
                format!(
                    "could not fetch cargo's index in {}: {}",
                    index.path().display(),
                    e
                )
            })
            .unwrap();
        assert!(index.crate_("crates-index").is_some());
        assert!(index.crate_("toml").is_some());
        assert!(index.crate_("gcc").is_some());
        assert!(index.crate_("cc").is_some());
        assert!(index.crate_("CC").is_some());
        assert!(index.crate_("無").is_none());
    }

    #[test]
    fn test_can_parse_all() {
        let tmp_dir = TempDir::new().unwrap();
        let mut found_gcc_crate = false;

        let index = Index::with_path(tmp_dir.path(), crate::INDEX_GIT_URL).unwrap();
        let mut ctx = DedupeContext::new();

        for c in index.crates_refs().unwrap() {
            if c.as_slice().map_or(false, |blob| blob.is_empty()) {
                continue; // https://github.com/rust-lang/crates.io/issues/6159
            }
            match c.parse(&mut ctx) {
                Ok(c) => {
                    if c.name() == "gcc" {
                        found_gcc_crate = true;
                    }
                }
                Err(e) => panic!("can't parse :( {c:?}: {e}"),
            }
        }

        assert!(found_gcc_crate);
    }

    #[test]
    fn features2() {
        let c = Crate::from_slice(br#"{"vers":"1.0.0", "name":"test", "deps":[], "features":{"a":["one"], "b":["x"]},"features2":{"a":["two"], "c":["y"]}, "cksum":"1234567890123456789012345678901234567890123456789012345678901234"}"#).unwrap();
        let f2 = c.most_recent_version().features();

        assert_eq!(3, f2.len());
        assert_eq!(["one", "two"], &f2["a"][..]);
        assert_eq!(["x"], &f2["b"][..]);
        assert_eq!(["y"], &f2["c"][..]);
    }

    #[test]
    fn rust_version() {
        let c = Crate::from_slice(br#"{"vers":"1.0.0", "name":"test", "deps":[], "features":{},"features2":{}, "cksum":"1234567890123456789012345678901234567890123456789012345678901234", "rust_version":"1.64.0"}"#).unwrap();
        assert_eq!(c.most_recent_version().rust_version(), Some("1.64.0"));
    }
}
