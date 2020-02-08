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
//! extern crate crates_index;
//!
//! let index = crates_index::Index::new("/tmp/_index");
//! if !index.exists() {
//!    index.retrieve().expect("Could not fetch crates.io index");
//! }
//! for crate_ in index.crates() {
//!    let latest_version = crate_.latest_version();
//!    println!("crate name: {}", latest_version.name());
//!    println!("crate version: {}", latest_version.version());
//! }
//! ```

use std::collections::HashMap;
use std::fs;
use std::io::{self, BufRead, BufReader};
use std::iter;
use std::path::{Path, PathBuf};

#[macro_use]
extern crate error_chain;
extern crate git2;
extern crate glob;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate serde;
extern crate home;

error_chain! {
    foreign_links {
        Git(git2::Error);
    }
}

static INDEX_GIT_URL: &'static str = "https://github.com/rust-lang/crates.io-index";


/// A single version of a crate published to the index
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Version {
    name: String,
    vers: String,
    deps: Vec<Dependency>,
    cksum: String,
    features: HashMap<String, Vec<String>>,
    yanked: bool,
}

impl Version {
    /// Name of the crate
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Name of this version
    pub fn version(&self) -> &str {
        &self.vers
    }

    /// Dependencies for this version
    pub fn dependencies(&self) -> &[Dependency] {
        &self.deps
    }

    /// Checksum of the package for this version
    pub fn checksum(&self) -> &str {
        &self.cksum
    }

    pub fn features(&self) -> &HashMap<String, Vec<String>> {
        &self.features
    }

    /// Whether this version was [yanked](http://doc.crates.io/crates-io.html#cargo-yank) from the
    /// index
    pub fn is_yanked(&self) -> bool {
        self.yanked
    }
}

/// A single dependency of a specific crate version
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Dependency {
    name: String,
    req: String,
    features: Vec<String>,
    optional: bool,
    default_features: bool,
    target: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    kind: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    package: Option<String>,
}

impl Dependency {
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn requirement(&self) -> &str {
        &self.req
    }

    pub fn features(&self) -> &[String] {
        &self.features
    }

    pub fn is_optional(&self) -> bool {
        self.optional
    }

    pub fn has_default_features(&self) -> bool {
        self.default_features
    }

    pub fn target(&self) -> Option<&str> {
        match self.target {
            Some(ref s) => Some(s),
            None => None,
        }
    }

    pub fn kind(&self) -> Option<&str> {
        match self.kind {
            Some(ref s) => Some(s),
            None => None,
        }
    }

    pub fn package(&self) -> Option<&str> {
        match self.package {
            Some(ref s) => Some(s),
            None => None,
        }
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
    pub fn crate_name(&self) -> &str {
        match self.package {
            Some(ref s) => s,
            None => self.name(),
        }
    }
}


/// Constructed from `Index::crates`
///
/// Silently ignores crates that can't be loaded/parsed
pub struct Crates(CrateIndexPaths);

impl Iterator for Crates {
    type Item = Crate;
    fn next(&mut self) -> Option<Self::Item> {
        while let Some(p) = self.0.next() {
            if let Ok(c) = Crate::new_checked(&p) {
                return Some(c)
            }
        }
        None
    }
}


/// Constructed from `Index::crate_index_paths`
pub struct CrateIndexPaths(iter::Chain<glob::Paths, glob::Paths>);

impl CrateIndexPaths {
    fn new<P: AsRef<Path>>(path: P) -> CrateIndexPaths {
        let mut match_options = glob::MatchOptions::new();
        match_options.require_literal_leading_dot = true;
        let path = path.as_ref();

        let glob_pattern = format!("{}/*/*/*", path.to_str().unwrap());
        let index_paths1 = glob::glob_with(&glob_pattern, match_options).unwrap();

        let glob_pattern = format!("{}/[12]/*", path.to_str().unwrap());
        let index_paths2 = glob::glob_with(&glob_pattern, match_options).unwrap();

        CrateIndexPaths(index_paths1.chain(index_paths2))
    }
}

impl Iterator for CrateIndexPaths {
    type Item = PathBuf;
    fn next(&mut self) -> Option<Self::Item> {
        self.0.next().map(|glob_result| glob_result.unwrap())
    }
}

/// Wrapper around managing the crates.io-index git repository
#[derive(Debug, Clone, PartialEq)]
pub struct Index {
    path: PathBuf,
}

impl Index {
    /// Construct a new Index supplying a path where the index lives or should live
    pub fn new<P: Into<PathBuf>>(path: P) -> Index {
        Index { path: path.into() }
    }

    /// Use Cargo's own index in `CARGO_HOME` (`~/.cargo/registry/index`)
    pub fn new_cargo_default() -> Index {
        let cargo_home = home::cargo_home().unwrap_or_default();
        Self::new(cargo_home.join("registry").join("index").join("github.com-1ecc6299db9ec823"))
    }

    /// Determines if a crates.io repository exists at `self.path`
    pub fn exists(&self) -> bool {
        git2::Repository::discover(&self.path)
            .map(|repository| {
                repository
                    .find_remote("origin").ok()
                    // Cargo creates a checkout without an origin set,
                    // so default to true in case of missing origin
                    .map_or(true, |remote| remote.url().map_or(true, |url| url == INDEX_GIT_URL))
            })
            .unwrap_or(false)
    }

    /// Downloads the index to the path specified from the constructor
    pub fn retrieve(&self) -> Result<()> {
        let _ = git2::Repository::clone(INDEX_GIT_URL, &self.path)?;
        Ok(())
    }

    /// Assumes the index already exists at `self.path`, and updates it
    pub fn update(&self) -> Result<()> {
        debug_assert!(self.exists());
        let repo = git2::Repository::discover(&self.path)?;
        let mut origin_remote = repo.find_remote("origin")
            .or_else(|_| repo.remote_anonymous(INDEX_GIT_URL))?;
        origin_remote.fetch(&["master"], None, None)?;
        let oid = repo.refname_to_id("FETCH_HEAD")?;
        let object = repo.find_object(oid, None).unwrap();
        repo.reset(&object, git2::ResetType::Hard, None)?;
        Ok(())
    }

    /// Downloads the index to the path specified from the constructor
    pub fn retrieve_or_update(&self) -> Result<()> {
        if self.exists() {
            self.update()
        } else {
            self.retrieve()
        }
    }

    /// Retrieve a single crate by name (case insensitive) from the index
    pub fn crate_(&self, crate_name: &str) -> Option<Crate> {
        let name_lower = crate_name.to_ascii_lowercase();
        if !name_lower.is_ascii() {
            return None;
        }
        let path = match name_lower.len() {
            0 => return None,
            1 => self.path.join("1"),
            2 => self.path.join("2"),
            3 => self.path.join("3").join(&name_lower[0..1]),
            _ => self.path.join(&name_lower[0..2]).join(&name_lower[2..4]),
        }.join(name_lower);
        if path.exists() {
            Crate::new_checked(path.as_path()).ok()
        } else {
            None
        }
    }

    /// Retrieve an iterator over all the crates in the index
    pub fn crates(&self) -> Crates {
        Crates(self.crate_index_paths())
    }

    /// Returns all the crate index file paths in the index
    pub fn crate_index_paths(&self) -> CrateIndexPaths {
        CrateIndexPaths::new(self.path.clone()) // TODO: remove this clone
    }

    /// Get the index directory.
    pub fn path(&self) -> &Path {
        &self.path
    }
}

/// A single crate that contains many published versions
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Crate {
    versions: Vec<Version>,
}

impl Crate {
    #[doc(hidden)]
    #[deprecated(note = "this may panic. use new_checked() instead")]
    pub fn new<P: AsRef<Path>>(index_path: P) -> Crate {
        Self::new_checked(index_path).unwrap()
    }

    /// Parse the file with crate versions.
    ///
    /// The file must contain at least one version.
    pub fn new_checked<P: AsRef<Path>>(index_path: P) -> io::Result<Crate> {
        let index_path = index_path.as_ref();
        let mut versions = vec![];
        let file = fs::File::open(&index_path)?;
        for line in BufReader::new(file).lines() {
            let version: Version = serde_json::from_str(&line?).map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
            versions.push(version);
        }
        if versions.is_empty() {
            return Err(io::Error::new(io::ErrorKind::Other, "crate must have versions"));
        }
        Ok(Crate { versions })
    }

    /// Published versions of this crate sorted chronologically by date published
    pub fn versions(&self) -> &[Version] {
        &self.versions
    }

    /// Oldest version.
    ///
    /// Warning: may not be the lowest version number.
    pub fn earliest_version(&self) -> &Version {
        &self.versions[0]
    }

    /// Most recently published version. Warning: may not be the highest version.
    pub fn latest_version(&self) -> &Version {
        &self.versions[self.versions.len() - 1]
    }

    pub fn name(&self) -> &str {
        self.latest_version().name()
    }
}

#[cfg(test)]
mod test {
    use super::Index;
    extern crate tempdir;
    use self::tempdir::TempDir;

    #[test]
    fn test_dependencies() {
        let tmp_dir = TempDir::new("test1").unwrap();

        let index = Index::new(tmp_dir.path());
        index.retrieve_or_update().expect("could not fetch crates io index");
        // let crate_ = index.crates().nth(0).expect("could not find a crate in the index");
        let crate_ = index.crate_("sval").expect("Could not find the crate libnotify in the index");
        let _ = format!("supports debug {:?}", crate_);

        let version = crate_.versions().iter().find(|v| v.version() == "0.0.1")
            .expect("Version 0.0.1 of sval does not exist?");
        let dep_with_package_name = version.dependencies().iter().find(|d| d.name() == "serde_lib")
            .expect("sval does not have expected dependency?");
        assert_ne!(dep_with_package_name.name(), dep_with_package_name.package().unwrap());
        assert_eq!(dep_with_package_name.crate_name(), dep_with_package_name.package().unwrap());
    }

    #[test]
    fn test_retrieve_or_update() {
        let tmp_dir = TempDir::new("test2").unwrap();

        let index = Index::new(tmp_dir.path());
        index.retrieve_or_update().expect("could not fetch crates io index");
        assert!(index.exists());
        index.retrieve_or_update().expect("could not fetch crates io index");
        assert!(index.exists());
    }

    #[test]
    fn test_cargo_default_updates() {
        let index = Index::new_cargo_default();
        index.update().map_err(|e| format!("could not fetch cargo's index in {}: {}", index.path().display(), e)).unwrap();
        assert!(index.crate_("crates-index").is_some());
        assert!(index.crate_("toml").is_some());
        assert!(index.crate_("gcc").is_some());
        assert!(index.crate_("cc").is_some());
        assert!(index.crate_("CC").is_some());
        assert!(index.crate_("無").is_none());
    }

    #[test]
    fn test_exists() {
        let tmp_dir = TempDir::new("test3").unwrap();

        let index = Index::new(tmp_dir.path());
        assert!(!index.exists());
        index.retrieve().unwrap();
        assert!(index.exists());
    }
}
