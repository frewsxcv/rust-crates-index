use crate::dedupe::DedupeContext;

use crate::IndexConfig;
use semver::Version as SemverVersion;
use serde_derive::{Deserialize, Serialize};
use smol_str::SmolStr;
use std::collections::HashMap;
use std::io;
use std::path::Path;
use std::sync::Arc;

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

    /// combines features and features2
    ///
    /// dedupes dependencies and features
    fn build_data(&mut self, dedupe: &mut DedupeContext) {
        if let Some(features2) = self.features2.take() {
            if let Some(f1) = Arc::get_mut(&mut self.features) {
                for (key, mut val) in features2.into_iter() {
                    f1.entry(key).or_insert_with(Vec::new).append(&mut val);
                }
            }
        }

        // Many versions have identical dependencies and features
        dedupe.deps(&mut self.deps);
        dedupe.features(&mut self.features);
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
    registry: Option<SmolStr>,
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

    /// The registry URL, if available.
    ///
    /// Example: `https://github.com/rust-lang/crates.io-index.git`
    #[inline]
    #[must_use]
    pub fn registry(&self) -> Option<&str> {
        self.registry.as_deref()
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
#[derive(Debug, Copy, Clone, Serialize, Deserialize, Eq, PartialEq, Hash, Default)]
#[serde(rename_all = "lowercase")]
pub enum DependencyKind {
    /// Used at run time
    #[default]
    Normal,
    /// Not fetched and not used, except for when used direclty in a workspace
    Dev,
    /// Used at build time, not available at run time
    Build,
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
            let mut version: Version = serde_json::from_slice(line).map_err(io::Error::other)?;

            version.build_data(dedupe);

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
    /// 2. The entry was created with an older version than the one specified
    /// 3. The entry is a newer version than what can be read, would only
    ///    happen if a future version of cargo changed the format of the cache entries
    /// 4. The cache entry is malformed somehow
    #[inline(never)]
    pub(crate) fn from_cache_slice(bytes: &[u8], index_version: Option<&str>) -> io::Result<Self> {
        const CURRENT_CACHE_VERSION: u8 = 3;
        const CURRENT_INDEX_FORMAT_VERSION: u32 = 2;

        // See src/cargo/sources/registry/index.rs
        let (first_byte, mut rest) = bytes.split_first().ok_or(io::ErrorKind::UnexpectedEof)?;

        match *first_byte {
            // This is the current 1.54.0 - 1.70.0+ version of cache entries
            CURRENT_CACHE_VERSION => {
                let index_v_bytes = rest.get(..4).ok_or(io::ErrorKind::UnexpectedEof)?;
                let index_v = u32::from_le_bytes(index_v_bytes.try_into().unwrap());
                if index_v != CURRENT_INDEX_FORMAT_VERSION {
                    return Err(io::Error::new(
                        io::ErrorKind::Unsupported,
                        format!("wrong index format version: {index_v} (expected {CURRENT_INDEX_FORMAT_VERSION}))"),
                    ));
                }
                rest = &rest[4..];
            }
            // This is only to support ancient <1.52.0 versions of cargo https://github.com/rust-lang/cargo/pull/9161
            1 => {}
            // Note that the change from 2 -> 3 was only to invalidate cache
            // entries https://github.com/rust-lang/cargo/pull/9476 and
            // version 2 entries should only be emitted by cargo 1.52.0 and 1.53.0,
            // but rather than _potentially_ parse bad cache entries as noted in
            // the PR we explicitly tell the user their version of cargo is suspect
            // these versions are so old (and specific) it shouldn't affect really anyone
            2 => {
                return Err(io::Error::other("potentially invalid version 2 cache entry found"));
            }
            version => {
                return Err(io::Error::new(
                    io::ErrorKind::Unsupported,
                    format!("cache version '{version}' not currently supported"),
                ));
            }
        }

        let mut iter = crate::split(rest, 0);
        let update = iter.next().ok_or(io::ErrorKind::UnexpectedEof)?;
        if let Some(index_version) = index_version {
            if update != index_version.as_bytes() {
                return Err(io::Error::other(format!(
                    "cache out of date: current index ({index_version}) != cache ({})",
                    String::from_utf8_lossy(update)
                )));
            }
        }

        Self::from_version_entries_iter(iter)
    }

    pub(crate) fn from_version_entries_iter<'a, I: Iterator<Item = &'a [u8]> + 'a>(mut iter: I) -> io::Result<Crate> {
        let mut versions = Vec::new();

        let mut dedupe = DedupeContext::new();

        // Each entry is a tuple of (semver, version_json)
        while let Some(_version) = iter.next() {
            let version_slice = iter.next().ok_or(io::ErrorKind::UnexpectedEof)?;
            let mut version: Version =
                serde_json::from_slice(version_slice).map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;

            version.build_data(&mut dedupe);

            versions.push(version);
        }

        Ok(Self {
            versions: versions.into_boxed_slice(),
        })
    }

    /// Writes a cache entry to disk in the same format as cargo
    #[cfg(feature = "sparse")]
    pub(crate) fn write_cache_entry(&self, path: &Path, version: &str) -> io::Result<()> {
        const CURRENT_CACHE_VERSION: u8 = 3;
        const CURRENT_INDEX_FORMAT_VERSION: u32 = 2;

        let mut v = Vec::new();
        v.push(CURRENT_CACHE_VERSION);
        v.extend_from_slice(&CURRENT_INDEX_FORMAT_VERSION.to_le_bytes());
        v.extend_from_slice(version.as_bytes());
        v.push(0);

        for version in self.versions() {
            v.extend_from_slice(version.version().as_bytes());
            v.push(0);
            v.append(&mut serde_json::to_vec(version).unwrap());
            v.push(0);
        }

        std::fs::write(path, v)
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
