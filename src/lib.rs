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

use std::ascii::AsciiExt;
use std::collections::HashMap;
use std::fs;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;

extern crate git2;
extern crate glob;
extern crate rustc_serialize;


static INDEX_GIT_URL: &'static str = "https://github.com/rust-lang/crates.io-index";


/// A single version of crate published to the index
#[derive(RustcDecodable)]
pub struct Version {
    pub name: String,
    pub vers: String,
    pub deps: Vec<Dependency>,
    pub cksum: String,
    pub features: HashMap<String, Vec<String>>,
    pub yanked: bool,
}

/// A single dependency of a specific crate version
#[derive(RustcDecodable)]
pub struct Dependency {
    pub name: String,
    pub req: String,
    pub features: Vec<String>,
    pub optional: bool,
    pub default_features: bool,
    pub target: Option<String>,
    pub kind: Option<String>
}


/// Constructed from `Index::crates`
pub struct Crates(CrateIndexPaths);

impl Iterator for Crates {
    type Item = Crate;
    fn next(&mut self) -> Option<Self::Item> {
        self.0.next().map(|p| Crate::new(&p))
    }
}


/// Constructed from `Index::crate_index_paths`
pub struct CrateIndexPaths(std::iter::Chain<glob::Paths, glob::Paths>);

impl CrateIndexPaths {
    fn new(path: PathBuf) -> CrateIndexPaths {
        let mut match_options = glob::MatchOptions::new();
        match_options.require_literal_leading_dot = true;

        let glob_pattern = format!("{}/*/*/*", path.to_str().unwrap());
        let index_paths1 = glob::glob_with(&glob_pattern, &match_options).unwrap();

        let glob_pattern = format!("{}/[12]/*", path.to_str().unwrap());
        let index_paths2 = glob::glob_with(&glob_pattern, &match_options).unwrap();

        CrateIndexPaths(index_paths1.chain(index_paths2))
    }
}

impl Iterator for CrateIndexPaths {
    type Item = PathBuf;
    fn next(&mut self)  -> Option<Self::Item> {
        self.0.next().map(|glob_result| glob_result.unwrap())
    }
}


/// Wrapper around managing the crates.io-index git repository
pub struct Index {
    path: PathBuf,
}

impl Index {
    /// Construct a new Index supplying a path where the index lives or should live
    pub fn new(path: PathBuf) -> Index {
        Index{path: path}
    }

    /// Determines if *anything* exists at the path specified from the constructor
    pub fn exists(&self) -> bool {
        fs::metadata(&self.path).is_ok()
    }

    /// Clones the index to the path specified from the constructor
    pub fn clone(&self) -> Result<(), git2::Error> {
        let _ = try!(git2::Repository::clone(INDEX_GIT_URL, &self.path));
        Ok(())
    }

    /// Retrieve a single crate by name (case insensitive) from the index
    pub fn crate_(&self, crate_name: &str) -> Option<Crate> {
        self.crate_index_paths()
            .find(|path| path.file_name().unwrap().to_str().unwrap().eq_ignore_ascii_case(crate_name))
            .map(|p| Crate::new(&p))
    }

    /// Retrieve an iterator over all the crates in the index
    pub fn crates(&self) -> Crates {
        Crates(self.crate_index_paths())
    }

    /// Returns all the crate index file paths in the index
    pub fn crate_index_paths(&self) -> CrateIndexPaths {
        CrateIndexPaths::new(self.path.clone())  // TODO: remove this clone
    }
}


/// A single crate that contains many published versions
pub struct Crate {
    versions: Vec<Version>,
}

impl Crate {
    pub fn new(index_path: &PathBuf) -> Crate {
        let mut versions = vec![];
        let file = fs::File::open(&index_path).unwrap();
        for line in BufReader::new(file).lines() {
            let version: Version = rustc_serialize::json::decode(&line.unwrap()).unwrap();
            versions.push(version);
        }
        Crate {versions: versions}
    }

    pub fn latest_version(&self) -> &Version {
        &self.versions[self.versions.len() - 1]
    }
}


#[test]
fn test_dependencies() {
    let index = Index::new("_test".into());
    if !index.exists() {
        index.clone().unwrap();
    }
    let crate_ = index.crates().nth(0).unwrap();
    let version = crate_.latest_version();
    let _ = version.deps;
}
