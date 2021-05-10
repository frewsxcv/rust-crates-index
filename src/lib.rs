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

//! Library for retrieving and interacting with the [crates.io index].
//!
//! [crates.io index]: https://github.com/rust-lang/crates.io-index
//!
//! ## Examples
//!
//! ```rust
//! let index = crates_index::Index::new_cargo_default();
//! if !index.exists() {
//!    index.retrieve().expect("Could not fetch crates.io index");
//! }
//! for crate_ in index.crates() {
//!    let latest_version = crate_.latest_version();
//!    println!("crate name: {}", latest_version.name());
//!    println!("crate version: {}", latest_version.version());
//! }
//! ```

use semver::Version as SemverVersion;
use serde_derive::{Deserialize, Serialize};
use smartstring::alias::String as SmolStr;
use std::collections::HashSet;
use std::sync::Arc;
use std::{
    collections::HashMap,
    io, iter,
    path::{Path, PathBuf},
};

mod bare_index;
mod error;

pub use bare_index::{BareIndex, BareIndexRepo};
pub use error::Error;

static INDEX_GIT_URL: &str = "https://github.com/rust-lang/crates.io-index";

/// A single version of a crate published to the index
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Version {
    name: SmolStr,
    vers: SmolStr,
    deps: Arc<[Dependency]>,
    features: Arc<HashMap<String, Vec<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    links: Option<Box<SmolStr>>,
    #[serde(with = "hex")]
    cksum: [u8; 32],
    yanked: bool,
}

impl Version {
    /// Name of the crate
    #[inline]
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Name of this version
    #[inline]
    pub fn version(&self) -> &str {
        &self.vers
    }

    /// Dependencies for this version
    #[inline]
    pub fn dependencies(&self) -> &[Dependency] {
        &self.deps
    }

    /// Checksum of the package for this version
    ///
    /// SHA256 of the .crate file
    #[inline]
    pub fn checksum(&self) -> &[u8; 32] {
        &self.cksum
    }

    #[inline]
    pub fn features(&self) -> &HashMap<String, Vec<String>> {
        &self.features
    }

    #[inline]
    pub fn links(&self) -> Option<&str> {
        self.links.as_ref().map(|s| s.as_str())
    }

    /// Whether this version was [yanked](http://doc.crates.io/crates-io.html#cargo-yank) from the
    /// index
    #[inline]
    pub fn is_yanked(&self) -> bool {
        self.yanked
    }

    pub fn download_url(&self, index: &IndexConfig) -> Option<String> {
        index.download_url(&self.name, &self.vers)
    }
}

/// A single dependency of a specific crate version
#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq, Hash)]
pub struct Dependency {
    name: SmolStr,
    req: SmolStr,
    features: Box<[String]>,
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
    #[inline]
    pub fn name(&self) -> &str {
        &self.name
    }

    #[inline]
    pub fn requirement(&self) -> &str {
        &self.req
    }

    #[inline]
    pub fn features(&self) -> &[String] {
        &self.features
    }

    #[inline]
    pub fn is_optional(&self) -> bool {
        self.optional
    }

    #[inline]
    pub fn has_default_features(&self) -> bool {
        self.default_features
    }

    #[inline]
    pub fn target(&self) -> Option<&str> {
        self.target.as_ref().map(|s| s.as_str())
    }

    #[inline]
    pub fn kind(&self) -> DependencyKind {
        self.kind.unwrap_or_default()
    }

    #[inline]
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
    pub fn crate_name(&self) -> &str {
        match self.package {
            Some(ref s) => s,
            None => self.name(),
        }
    }
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize, Eq, PartialEq, Hash)]
#[serde(rename_all = "lowercase")]
pub enum DependencyKind {
    Normal,
    Dev,
    Build,
}

impl Default for DependencyKind {
    fn default() -> Self {
        Self::Normal
    }
}

/// Constructed from [`Index::crates`]
///
/// Silently ignores crates that can't be loaded/parsed
pub struct Crates(CrateIndexPaths);

impl Iterator for Crates {
    type Item = Crate;
    fn next(&mut self) -> Option<Self::Item> {
        while let Some(p) = self.0.next() {
            if let Ok(c) = Crate::new(&p) {
                return Some(c);
            }
        }
        None
    }
}

/// Constructed from [`Index::crate_index_paths`]
pub struct CrateIndexPaths(iter::Chain<iter::Chain<glob::Paths, glob::Paths>, glob::Paths>);

impl CrateIndexPaths {
    fn new<P: AsRef<Path>>(path: P) -> CrateIndexPaths {
        let mut match_options = glob::MatchOptions::new();
        match_options.require_literal_leading_dot = true;
        let path = path.as_ref();

        // > 3 characters
        let index_paths1 =
            glob::glob_with(&format!("{}/*/*/*", path.to_str().unwrap()), match_options).unwrap();

        // 1 or 2
        let index_paths2 =
            glob::glob_with(&format!("{}/[12]/*", path.to_str().unwrap()), match_options).unwrap();

        // 3
        let index_paths3 =
            glob::glob_with(&format!("{}/3/*/*", path.to_str().unwrap()), match_options).unwrap();

        CrateIndexPaths(index_paths1.chain(index_paths2).chain(index_paths3))
    }
}

impl Iterator for CrateIndexPaths {
    type Item = PathBuf;
    fn next(&mut self) -> Option<Self::Item> {
        self.0.next().map(|glob_result| glob_result.unwrap())
    }
}

fn fetch_opts<'cb>() -> git2::FetchOptions<'cb> {
    let mut proxy_opts = git2::ProxyOptions::new();
    proxy_opts.auto();
    let mut fetch_opts = git2::FetchOptions::new();
    fetch_opts.proxy_options(proxy_opts);
    fetch_opts
}

/// Wrapper around managing the crates.io-index git repository
#[derive(Debug, Clone, PartialEq)]
pub struct Index {
    path: PathBuf,
}

impl Index {
    /// Construct a new Index supplying a path where the index lives or should live
    #[inline]
    pub fn new<P: Into<PathBuf>>(path: P) -> Index {
        Index { path: path.into() }
    }

    /// Use Cargo's own index in `CARGO_HOME` (`~/.cargo/registry/index`)
    pub fn new_cargo_default() -> Index {
        let cargo_home = home::cargo_home().unwrap_or_default();
        Self::new(
            cargo_home
                .join("registry")
                .join("index")
                .join("github.com-1ecc6299db9ec823"),
        )
    }

    /// Determines if a crates.io repository exists at `self.path`
    pub fn exists(&self) -> bool {
        git2::Repository::discover(&self.path)
            .map(|repository| {
                repository
                    .find_remote("origin")
                    .ok()
                    // Cargo creates a checkout without an origin set,
                    // so default to true in case of missing origin
                    .map_or(true, |remote| {
                        remote.url().map_or(true, |url| url == INDEX_GIT_URL)
                    })
            })
            .unwrap_or(false)
    }

    /// Downloads the index to the path specified from the constructor
    pub fn retrieve(&self) -> Result<(), Error> {
        let mut opts = git2::RepositoryInitOptions::new();
        opts.external_template(false);
        git2::Repository::init_opts(&self.path, &opts)?;
        self.update()?;
        Ok(())
    }

    /// Assumes the index already exists at `self.path`, and updates it
    pub fn update(&self) -> Result<(), Error> {
        debug_assert!(self.exists());
        let repo = git2::Repository::discover(&self.path)?;
        let mut origin_remote = repo
            .find_remote("origin")
            .or_else(|_| repo.remote_anonymous(INDEX_GIT_URL))?;
        origin_remote.fetch(&["HEAD"], Some(&mut fetch_opts()), None)?;
        let oid = repo.refname_to_id("FETCH_HEAD")?;
        let object = repo.find_object(oid, None).unwrap();
        repo.reset(&object, git2::ResetType::Hard, None)?;
        Ok(())
    }

    /// Downloads the index to the path specified from the constructor
    pub fn retrieve_or_update(&self) -> Result<(), Error> {
        if self.exists() {
            self.update()
        } else {
            self.retrieve()
        }
    }

    /// Retrieve a single crate by name (case insensitive) from the index
    pub fn crate_(&self, crate_name: &str) -> Option<Crate> {
        match crate_name_to_relative_path(crate_name) {
            Some(rel_path) => {
                let path = self.path.join(rel_path);
                if path.exists() {
                    Crate::new(path.as_path()).ok()
                } else {
                    None
                }
            }
            None => None,
        }
    }

    /// Retrieve an iterator over all the crates in the index
    #[inline]
    pub fn crates(&self) -> Crates {
        Crates(CrateIndexPaths::new(&self.path))
    }

    /// Returns all the crate index file paths in the index
    #[deprecated(note = "This method won't work with BareIndex")]
    pub fn crate_index_paths(&self) -> CrateIndexPaths {
        CrateIndexPaths::new(&self.path)
    }

    /// Get the index directory.
    #[inline]
    pub fn path(&self) -> &Path {
        &self.path
    }

    /// Get the global configuration of the index.
    pub fn index_config(&self) -> Result<IndexConfig, Error> {
        let content = std::fs::read(self.path().join("config.json")).map_err(Error::Io)?;
        serde_json::from_slice(&content).map_err(Error::Json)
    }
}

fn crate_prefix(crate_name: &str, separator: char) -> Option<SmolStr> {
    if !crate_name.is_ascii() {
        return None;
    }

    let mut accumulator = SmolStr::new();

    match crate_name.len() {
        0 => return None,
        1 => accumulator.push('1'),
        2 => accumulator.push('2'),
        3 => {
            accumulator.push('3');
            accumulator.push(separator);
            accumulator.push_str(&crate_name[0..1]);
        }
        _ => {
            accumulator.push_str(&crate_name[0..2]);
            accumulator.push(separator);
            accumulator.push_str(&crate_name[2..4]);
        }
    };
    Some(accumulator)
}

fn crate_name_to_relative_path(crate_name: &str) -> Option<String> {
    if !crate_name.is_ascii() {
        return None;
    }

    let name_lower = crate_name.to_ascii_lowercase();
    let mut rel_path = String::with_capacity(crate_name.len() + 6);
    rel_path.push_str(&crate_prefix(&name_lower, std::path::MAIN_SEPARATOR)?);
    rel_path.push(std::path::MAIN_SEPARATOR);
    rel_path.push_str(&name_lower);

    Some(rel_path)
}

/// A single crate that contains many published versions
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Crate {
    versions: Box<[Version]>,
}

impl Crate {
    /// Parse the file with crate versions.
    ///
    /// The file must contain at least one version.
    #[inline]
    pub fn new<P: AsRef<Path>>(index_path: P) -> io::Result<Crate> {
        let lines = std::fs::read(index_path)?;
        Self::from_slice(&lines)
    }

    #[doc(hidden)]
    #[deprecated(note = "new_checked() is no longer needed, you can use new() now")]
    pub fn new_checked<P: AsRef<Path>>(index_path: P) -> io::Result<Crate> {
        Self::new(index_path)
    }

    /// Parse crate file from in-memory JSON data
    pub fn from_slice(mut bytes: &[u8]) -> io::Result<Crate> {
        // Trim last newline
        while bytes.last() == Some(&b'\n') {
            bytes = &bytes[..bytes.len() - 1];
        }

        #[inline(always)]
        fn is_newline(&c: &u8) -> bool {
            c == b'\n'
        }
        let num_versions = bytes.split(is_newline).count();
        let mut deps_dedupe = HashSet::with_capacity(num_versions);
        let mut features_dedupe = Vec::new();
        let mut versions = Vec::with_capacity(num_versions);
        for line in bytes.split(is_newline) {
            let mut version: Version = serde_json::from_slice(line)
                .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;

            // Many versions have identical dependencies and features
            if let Some(has_deps) = deps_dedupe.get(&version.deps) {
                version.deps = Arc::clone(has_deps);
            } else {
                deps_dedupe.insert(Arc::clone(&version.deps));
            }
            if let Some(has_feats) = features_dedupe.iter().find(|v| *v == &version.features) {
                version.features = Arc::clone(has_feats);
            } else {
                features_dedupe.push(Arc::clone(&version.features));
            }
            versions.push(version);
        }
        if versions.is_empty() {
            return Err(io::Error::new(
                io::ErrorKind::Other,
                "crate must have versions",
            ));
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
    pub fn from_cache_slice(bytes: &[u8], index_version: &str) -> io::Result<Crate> {
        const CURRENT_CACHE_VERSION: u8 = 1;

        // See src/cargo/sources/registry/index.rs
        let (first_byte, rest) = bytes
            .split_first()
            .ok_or_else(|| io::Error::new(io::ErrorKind::Other, "malformed cache"))?;

        if *first_byte != CURRENT_CACHE_VERSION {
            return Err(io::Error::new(
                io::ErrorKind::Other,
                "looks like a different Cargo's cache, bailing out",
            ));
        }

        fn split<'a>(haystack: &'a [u8], needle: u8) -> impl Iterator<Item = &'a [u8]> + 'a {
            struct Split<'a> {
                haystack: &'a [u8],
                needle: u8,
            }

            impl<'a> Iterator for Split<'a> {
                type Item = &'a [u8];

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

        let mut iter = split(rest, 0);
        if let Some(update) = iter.next() {
            if update != index_version.as_bytes() {
                return Err(io::Error::new(
                    io::ErrorKind::Other,
                    format!(
                        "cache out of date: current index ({}) != cache ({})",
                        index_version,
                        std::str::from_utf8(update)
                            .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?,
                    ),
                ));
            }
        } else {
            return Err(io::Error::new(io::ErrorKind::Other, "malformed file"));
        }

        let mut versions = Vec::new();

        // Each entry is a tuple of (semver, version_json)
        while let Some(_version) = iter.next() {
            let version_slice = iter
                .next()
                .ok_or_else(|| io::Error::new(io::ErrorKind::Other, "malformed file"))?;
            let version: Version = serde_json::from_slice(version_slice)
                .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
            versions.push(version);
        }

        Ok(Self {
            versions: versions.into_boxed_slice(),
        })
    }

    /// Published versions of this crate sorted chronologically by date published
    #[inline]
    pub fn versions(&self) -> &[Version] {
        &self.versions
    }

    /// Oldest version.
    ///
    /// Warning: may not be the lowest version number.
    #[inline]
    pub fn earliest_version(&self) -> &Version {
        &self.versions[0]
    }

    /// Most recently published version. Warning: may not be the highest version.
    #[inline]
    pub fn latest_version(&self) -> &Version {
        &self.versions[self.versions.len() - 1]
    }

    /// Returns the highest version as per semantic versioning specification.
    pub fn highest_version(&self) -> SemverVersion {
        self.versions
            .iter()
            .map(|v| SemverVersion::parse(&v.vers).ok())
            .flatten()
            .max()
            // Safety: Versions inside the index will always adhere to
            // semantic versioning. If a crate is inside the index, at
            // least one version is available.
            .unwrap()
    }

    /// Returns the highest version as per semantic versioning specification,
    /// filtering out versions with pre-release identifiers.
    pub fn highest_stable_version(&self) -> Option<SemverVersion> {
        self.versions
            .iter()
            .map(|v| SemverVersion::parse(&v.vers).ok())
            .flatten()
            .filter(|v| !v.is_prerelease())
            .max()
    }

    #[inline]
    pub fn name(&self) -> &str {
        self.latest_version().name()
    }
}

/// Global configuration of an index, reflecting the contents of config.json as specified at
/// https://doc.rust-lang.org/cargo/reference/registries.html#index-format
#[derive(Clone, Debug, Deserialize)]
pub struct IndexConfig {
    pub dl: String,
    pub api: Option<String>,
}

impl IndexConfig {
    /// Get the URL from where the specified package can be downloaded.
    /// This method assumes the particular version is present in the registry,
    /// and does not verify that it is.
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
            return Some(new);
        } else {
            let prefix = crate_prefix(name, '/')?;
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

#[cfg(test)]
mod test {
    use super::{Crate, Index};
    use tempdir::TempDir;

    #[test]
    fn test_dependencies() {
        let tmp_dir = TempDir::new("test1").unwrap();

        let index = Index::new(tmp_dir.path());
        index
            .retrieve_or_update()
            .expect("could not fetch crates io index");
        // let crate_ = index.crates().nth(0).expect("could not find a crate in the index");
        let crate_ = index
            .crate_("sval")
            .expect("Could not find the crate libnotify in the index");
        let _ = format!("supports debug {:?}", crate_);

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
    fn test_retrieve_or_update() {
        let tmp_dir = TempDir::new("test2").unwrap();

        let index = Index::new(tmp_dir.path());
        index
            .retrieve_or_update()
            .expect("could not fetch crates io index");
        assert!(index.exists());
        index
            .retrieve_or_update()
            .expect("could not fetch crates io index");
        assert!(index.exists());
    }

    #[test]
    fn test_cargo_default_updates() {
        let index = Index::new_cargo_default();
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
    #[allow(deprecated)]
    fn test_can_parse_all() {
        let tmp_dir = TempDir::new("test3").unwrap();
        let mut found_gcc_crate = false;

        let index = Index::new(tmp_dir.path());
        assert!(!index.exists());
        index.retrieve().unwrap();
        assert!(index.exists());

        for path in index.crate_index_paths() {
            match Crate::new(&path) {
                Ok(c) => {
                    if c.name() == "gcc" {
                        found_gcc_crate = true;
                    }
                }
                Err(e) => {
                    let _ = tmp_dir.into_path();
                    panic!("{} {}", e, path.display());
                }
            }
        }

        assert!(found_gcc_crate);
    }
}
