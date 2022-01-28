use crate::{Crate, Error, IndexConfig};
use std::fmt;

use std::{
    io,
    path::{Path, PathBuf},
};

/// Wrapper around managing the crates.io-index git repository
///
/// Uses a "bare" git index that fetches files directly from the repo instead of local checkout.
/// Uses Cargo's cache.
pub struct Index {
    path: PathBuf,
    url: String,

    repo: git2::Repository,
    head: git2::Oid,
    head_str: String,
}

impl Index {
    #[doc(hidden)]
    #[deprecated(note = "use new_cargo_default()")]
    pub fn new<P: Into<PathBuf>>(path: P) -> Self {
        Self::from_path_and_url(path.into(), crate::INDEX_GIT_URL.into()).unwrap()
    }

    /// Creates an index for the default crates.io registry, using the same
    /// disk location as Cargo itself.
    ///
    /// This is the recommended way to access Cargo's index.
    #[inline]
    pub fn new_cargo_default() -> Result<Self, Error> {
        Self::from_url(crate::INDEX_GIT_URL)
    }

    /// Creates a bare index from a provided URL, opening the same location on
    /// disk that Cargo uses for that registry index.
    ///
    /// It can be used to access custom registries.
    pub fn from_url(url: &str) -> Result<Self, Error> {
        let (dir_name, canonical_url) = url_to_local_dir(url)?;
        let mut path = home::cargo_home().unwrap_or_default();

        path.push("registry/index");
        path.push(dir_name);

        Self::from_path_and_url(path, canonical_url)
    }

    /// Creates a bare index at the provided path with the specified repository URL.
    #[inline]
    pub fn with_path<P: Into<PathBuf>, S: Into<String>>(path: P, url: S) -> Result<Self, Error> {
        Self::from_path_and_url(path.into(), url.into())
    }

    /// Get the index directory.
    #[inline]
    pub fn path(&self) -> &Path {
        &self.path
    }
}

impl Index {
    fn from_path_and_url(path: PathBuf, url: String) -> Result<Self, Error> {
        let exists = git2::Repository::discover(&path)
            .map(|repository| {
                repository
                    .find_remote("origin")
                    .ok()
                    // Cargo creates a checkout without an origin set,
                    // so default to true in case of missing origin
                    .map_or(true, |remote| remote.url().map_or(true, |u| u == url))
            })
            .unwrap_or(false);

        let repo = if !exists {
            let mut opts = git2::RepositoryInitOptions::new();
            opts.external_template(false);
            let repo = git2::Repository::init_opts(&path, &opts)?;
            {
                let mut origin_remote = repo
                    .find_remote("origin")
                    .or_else(|_| repo.remote_anonymous(&url))?;

                origin_remote.fetch(
                    &[
                        "HEAD:refs/remotes/origin/HEAD",
                        "master:refs/remotes/origin/master",
                    ],
                    Some(&mut crate::fetch_opts()),
                    None,
                )?;
            }
            repo
        } else {
            git2::Repository::open(&path)?
        };

        let head = repo
            // Fallback to HEAD, as a fresh clone won't have a FETCH_HEAD
            .refname_to_id("FETCH_HEAD")
            .or_else(|_| repo.refname_to_id("HEAD"))?;
        let head_str = head.to_string();

        Ok(Self {
            path,
            url,
            head_str,
            repo,
            head,
        })
    }

    fn tree(&self) -> Result<git2::Tree<'_>, git2::Error> {
        let commit = self.repo.find_commit(self.head)?;
        commit.tree()
    }

    #[doc(hidden)]
    #[deprecated(note = "use update()")]
    pub fn retrieve_or_update(&mut self) -> Result<(), Error> {
        self.update()
    }

    #[doc(hidden)]
    #[deprecated(note = "it's always retrieved. there's no need to call it any more")]
    pub fn retrieve(&self) -> Result<(), Error> {
        Ok(())
    }

    #[doc(hidden)]
    #[deprecated(note = "it's always retrieved, so it's assumed to always exist")]
    pub fn exists(&self) -> bool {
        true
    }

    /// Fetches latest from the remote index repository. Note that using this
    /// method will mean no cache entries will be used, if a new commit is fetched
    /// from the repository, as their commit version will no longer match.
    pub fn update(&mut self) -> Result<(), Error> {
        {
            let mut origin_remote = self
                .repo
                .find_remote("origin")
                .or_else(|_| self.repo.remote_anonymous(&self.url))?;

            origin_remote.fetch(
                &[
                    "HEAD:refs/remotes/origin/HEAD",
                    "master:refs/remotes/origin/master",
                ],
                Some(&mut crate::fetch_opts()),
                None,
            )?;
        }

        let head = self
            .repo
            .refname_to_id("FETCH_HEAD")
            .or_else(|_| self.repo.refname_to_id("HEAD"))?;

        self.head = head;
        self.head_str = self.head.to_string();

        Ok(())
    }

    /// Reads a crate from the index, it will attempt to use a cached entry if
    /// one is available, otherwise it will fallback to reading the crate
    /// directly from the git blob containing the crate information.
    ///
    /// Use this only if you need to get very few crates. If you're going
    /// to read majority of crates, prefer the [`crates()`] iterator.
    pub fn crate_(&self, name: &str) -> Option<Crate> {
        let rel_path = match crate::crate_name_to_relative_path(name) {
            Some(rp) => rp,
            None => return None,
        };

        // Attempt to load the .cache/ entry first, this is purely an acceleration
        // mechanism and can fail for a few reasons that are non-fatal
        {
            // avoid realloc on each push
            let mut cache_path =
                PathBuf::with_capacity(path_min_byte_len(&self.path) + 8 + rel_path.len());
            cache_path.push(&self.path);
            cache_path.push(".cache");
            cache_path.push(&rel_path);
            if let Ok(cache_bytes) = std::fs::read(&cache_path) {
                if let Ok(krate) = Crate::from_cache_slice(&cache_bytes, &self.head_str) {
                    return Some(krate);
                }
            }
        }

        // Fallback to reading the blob directly via git if we don't have a
        // valid cache entry
        self.crate_from_rel_path(&rel_path).ok()
    }

    fn crate_from_rel_path(&self, path: &str) -> Result<Crate, Error> {
        let entry = self.tree()?.get_path(&Path::new(path))?;
        let object = entry.to_object(&self.repo)?;
        let blob = object
            .as_blob()
            .ok_or_else(|| Error::Io(io::Error::new(io::ErrorKind::NotFound, path.to_owned())))?;

        Crate::from_slice(blob.content()).map_err(Error::Io)
    }

    /// update an iterator over all the crates in the index.
    /// Skips crates that can not be parsed.
    #[inline]
    pub fn crates(&self) -> Crates<'_> {
        Crates {
            blobs: self.crates_refs().expect("HEAD commit disappeared"),
        }
    }

    /// update an iterator over all the crates in the index.
    /// Returns opaque reference for each crate in the index, which can be used with [`CrateRef::parse`]
    pub(crate) fn crates_refs(&self) -> Result<CratesRefs<'_>, git2::Error> {
        let mut stack = Vec::with_capacity(800);
        for entry in self.tree()?.iter() {
            // crates are in short dirs, skip .git/.cache
            if entry.name_bytes().len() <= 2 {
                let entry = entry.to_object(&self.repo).expect("repo integrity");
                // Scan only directories at top level
                if entry.as_tree().is_some() {
                    stack.push(entry);
                }
            }
        }
        Ok(CratesRefs {
            stack,
            repo: &self.repo,
        })
    }

    /// Get the global configuration of the index.
    pub fn index_config(&self) -> Result<IndexConfig, Error> {
        let entry = self.tree()?.get_path(&Path::new("config.json"))?;
        let object = entry.to_object(&self.repo)?;
        let blob = object
            .as_blob()
            .ok_or_else(|| Error::Io(io::Error::new(io::ErrorKind::NotFound, "config.json")))?;
        serde_json::from_slice(blob.content()).map_err(Error::Json)
    }
}

#[cfg(unix)]
fn path_min_byte_len(path: &Path) -> usize {
    use std::os::unix::prelude::OsStrExt;
    path.as_os_str().as_bytes().len()
}

#[cfg(not(unix))]
fn path_min_byte_len(path: &Path) -> usize {
    path.to_str().map_or(0, |p| p.len())
}

/// Iterator over all crates in the index, but returns opaque objects that can be parsed separately.
///
/// See [`CrateRef::parse`].
pub(crate) struct CratesRefs<'a> {
    stack: Vec<git2::Object<'a>>,
    repo: &'a git2::Repository,
}

/// Opaque representation of a crate in the index. See [`CrateRef::parse`].
pub(crate) struct CrateRef<'a>(git2::Object<'a>);

impl CrateRef<'_> {
    #[inline]
    /// Parse a crate from [`Index::crates_blobs`] iterator
    pub fn parse(&self) -> io::Result<Crate> {
        let blob = self.as_slice().ok_or(io::ErrorKind::InvalidData)?;
        Crate::from_slice(blob)
    }

    /// Raw crate data that can be parsed with [`Crate::from_slice`]
    pub fn as_slice(&self) -> Option<&[u8]> {
        Some(self.0.as_blob()?.content())
    }
}

impl<'a> Iterator for CratesRefs<'a> {
    type Item = CrateRef<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(last) = self.stack.pop() {
            match last.as_tree() {
                None => return Some(CrateRef(last)),
                Some(tree) => {
                    for entry in tree.iter().rev() {
                        self.stack.push(entry.to_object(&self.repo).unwrap());
                    }
                    continue;
                }
            }
        }
        None
    }
}

impl fmt::Debug for CrateRef<'_> {
    #[cold]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("CrateRef")
            .field("oid", &self.0.id())
            .finish()
    }
}

/// Iterator over all crates in the index. Skips crates that failed to parse.
pub struct Crates<'a> {
    blobs: CratesRefs<'a>,
}

impl<'a> Iterator for Crates<'a> {
    type Item = Crate;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(next) = self.blobs.next() {
            if let Ok(k) = CrateRef::parse(&next) {
                return Some(k);
            }
        }
        None
    }
}

/// Converts a full url, eg https://github.com/rust-lang/crates.io-index, into
/// the root directory name where cargo itself will fetch it on disk
fn url_to_local_dir(url: &str) -> Result<(String, String), Error> {
    fn to_hex(num: u64) -> String {
        const CHARS: &[u8] = b"0123456789abcdef";

        let bytes = &[
            num as u8,
            (num >> 8) as u8,
            (num >> 16) as u8,
            (num >> 24) as u8,
            (num >> 32) as u8,
            (num >> 40) as u8,
            (num >> 48) as u8,
            (num >> 56) as u8,
        ];

        let mut output = vec![0u8; 16];

        let mut ind = 0;

        for &byte in bytes {
            output[ind] = CHARS[(byte >> 4) as usize];
            output[ind + 1] = CHARS[(byte & 0xf) as usize];

            ind += 2;
        }

        String::from_utf8(output).expect("valid utf-8 hex string")
    }

    #[allow(deprecated)]
    fn hash_u64(url: &str) -> u64 {
        use std::hash::{Hash, Hasher, SipHasher};

        let mut hasher = SipHasher::new_with_keys(0, 0);
        // Registry
        2usize.hash(&mut hasher);
        // Url
        url.hash(&mut hasher);
        hasher.finish()
    }

    // Ensure we have a registry or bare url
    let (url, scheme_ind) = {
        let scheme_ind = url
            .find("://")
            .ok_or_else(|| Error::Url(format!("'{}' is not a valid url", url)))?;

        let scheme_str = &url[..scheme_ind];
        if let Some(ind) = scheme_str.find('+') {
            if &scheme_str[..ind] != "registry" {
                return Err(Error::Url(format!("'{}' is not a valid registry url", url)));
            }

            (&url[ind + 1..], scheme_ind - ind - 1)
        } else {
            (url, scheme_ind)
        }
    };

    // Could use the Url crate for this, but it's simple enough and we don't
    // need to deal with every possible url (I hope...)
    let host = match url[scheme_ind + 3..].find('/') {
        Some(end) => &url[scheme_ind + 3..scheme_ind + 3 + end],
        None => &url[scheme_ind + 3..],
    };

    // cargo special cases github.com for reasons, so do the same
    let mut canonical = if host == "github.com" {
        url.to_lowercase()
    } else {
        url.to_owned()
    };

    // Chop off any query params/fragments
    if let Some(hash) = canonical.rfind('#') {
        canonical.truncate(hash);
    }

    if let Some(query) = canonical.rfind('?') {
        canonical.truncate(query);
    }

    let ident = to_hex(hash_u64(&canonical));

    if canonical.ends_with('/') {
        canonical.pop();
    }

    if canonical.contains("github.com/") && canonical.ends_with(".git") {
        // Only GitHub (crates.io) repositories have their .git suffix truncated
        canonical.truncate(canonical.len() - 4);
    }

    Ok((format!("{}-{}", host, ident), canonical))
}

#[cfg(test)]
mod test {
    #[test]
    fn matches_cargo() {
        assert_eq!(
            super::url_to_local_dir(crate::INDEX_GIT_URL).unwrap(),
            (
                "github.com-1ecc6299db9ec823".to_owned(),
                crate::INDEX_GIT_URL.to_owned()
            )
        );

        // I've confirmed this also works with a custom registry, unfortunately
        // that one includes a secret key as part of the url which would allow
        // anyone to publish to the registry, so uhh...here's a fake one instead
        assert_eq!(
            super::url_to_local_dir(
                "https://dl.cloudsmith.io/aBcW1234aBcW1234/embark/rust/cargo/index.git"
            )
            .unwrap(),
            (
                "dl.cloudsmith.io-ff79e51ddd2b38fd".to_owned(),
                "https://dl.cloudsmith.io/aBcW1234aBcW1234/embark/rust/cargo/index.git".to_owned()
            )
        );

        // Ensure we actually strip off the irrelevant parts of a url, note that
        // the .git suffix is not part of the canonical url, but *is* used when hashing
        assert_eq!(
            super::url_to_local_dir(&format!(
                "registry+{}.git?one=1&two=2#fragment",
                crate::INDEX_GIT_URL
            ))
            .unwrap(),
            (
                "github.com-c786010fb7ef2e6e".to_owned(),
                crate::INDEX_GIT_URL.to_owned()
            )
        );
    }

    #[test]
    fn bare_iterator() {
        use super::Index;

        let tmp_dir = tempdir::TempDir::new("bare_iterator").unwrap();

        let repo = Index::with_path(tmp_dir.path().to_owned(), crate::INDEX_GIT_URL)
            .expect("Failed to clone crates.io index");

        let mut found_gcc_crate = false;

        for c in repo.crates() {
            if c.name() == "gcc" {
                found_gcc_crate = true;
            }
        }

        assert!(found_gcc_crate);
    }

    #[test]
    fn clones_bare_index() {
        use super::Index;

        let tmp_dir = tempdir::TempDir::new("clones_bare_index").unwrap();

        let mut repo = Index::with_path(tmp_dir.path().to_owned(), crate::INDEX_GIT_URL)
            .expect("Failed to clone crates.io index");

        fn test_sval(repo: &Index) {
            let krate = repo
                .crate_("sval")
                .expect("Could not find the crate sval in the index");

            let version = krate
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

        test_sval(&repo);

        repo.update().expect("Failed to fetch crates.io index");

        test_sval(&repo);
    }

    #[test]
    fn opens_bare_index() {
        use super::Index;

        let tmp_dir = tempdir::TempDir::new("opens_bare_index").unwrap();

        let mut repo = Index::with_path(tmp_dir.path().to_owned(), crate::INDEX_GIT_URL)
            .expect("Failed to open crates.io index");

        fn test_sval(repo: &Index) {
            let krate = repo
                .crate_("sval")
                .expect("Could not find the crate sval in the index");

            let version = krate
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

        test_sval(&repo);

        repo.update().expect("Failed to fetch crates.io index");

        test_sval(&repo);
    }
}
