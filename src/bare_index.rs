use crate::{Crate, Error, IndexConfig};
use std::marker::PhantomPinned;
use std::{
    io,
    path::{Path, PathBuf},
};

/// Access to a "bare" git index that fetches files directly from the repo instead of local checkout
///
/// Uses Cargo's cache
pub struct BareIndex {
    path: PathBuf,
    pub url: String,
}

impl BareIndex {
    /// Creates a bare index from a provided URL, opening the same location on
    /// disk that cargo uses for that registry index.
    pub fn from_url(url: &str) -> Result<Self, Error> {
        let (dir_name, canonical_url) = url_to_local_dir(url)?;
        let mut path = home::cargo_home().unwrap_or_default();

        path.push("registry/index");
        path.push(dir_name);

        Ok(Self {
            path,
            url: canonical_url,
        })
    }

    /// Creates a bare index at the provided path with the specified repository URL.
    #[inline]
    pub fn with_path(path: PathBuf, url: &str) -> Self {
        Self {
            path,
            url: url.to_owned(),
        }
    }

    /// Creates an index for the default crates.io registry, using the same
    /// disk location as cargo itself.
    #[inline]
    pub fn new_cargo_default() -> Self {
        // UNWRAP: The default index git URL is known to safely convert to a path.
        Self::from_url(crate::INDEX_GIT_URL).unwrap()
    }

    /// Opens the local index, which acts as a kind of lock for source control
    /// operations
    #[inline]
    pub fn open_or_clone(&self) -> Result<BareIndexRepo<'_>, Error> {
        BareIndexRepo::new(self)
    }

    /// Get the index directory.
    #[inline]
    pub fn path(&self) -> &Path {
        &self.path
    }
}

/// Self-referential struct where `Tree` borrows from `Repository`
struct UnsafeRepoTree {
    /// Warning: order of the fields is necessary for safety. `tree` must Drop before `repo`.
    tree: git2::Tree<'static>,
    repo: Box<git2::Repository>,
    // Currently !Unpin is Rust's heuristic for self-referential structs
    _self_referential: PhantomPinned,
}

/// Opened instance of [`BareIndex`]
pub struct BareIndexRepo<'a> {
    inner: &'a BareIndex,
    head_str: String,
    rt: UnsafeRepoTree,
}

impl<'a> BareIndexRepo<'a> {
    fn new(index: &'a BareIndex) -> Result<Self, Error> {
        let exists = git2::Repository::discover(&index.path)
            .map(|repository| {
                repository
                    .find_remote("origin")
                    .ok()
                    // Cargo creates a checkout without an origin set,
                    // so default to true in case of missing origin
                    .map_or(true, |remote| {
                        remote.url().map_or(true, |url| url == index.url)
                    })
            })
            .unwrap_or(false);

        let repo = if !exists {
            let mut opts = git2::RepositoryInitOptions::new();
            opts.external_template(false);
            let repo = git2::Repository::init_opts(&index.path, &opts)?;
            {
                let mut origin_remote = repo
                    .find_remote("origin")
                    .or_else(|_| repo.remote_anonymous(&index.url))?;

                origin_remote.fetch(
                    &["HEAD:refs/remotes/origin/HEAD"],
                    Some(&mut crate::fetch_opts()),
                    None,
                )?;
            }
            repo
        } else {
            git2::Repository::open(&index.path)?
        };

        // It's going to be used in a self-referential type. Boxing prevents it from being moved
        // and adds a layer of indirection that will hopefully not upset noalias analysis.
        let repo = Box::new(repo);

        let head = repo
            // Fallback to HEAD, as a fresh clone won't have a FETCH_HEAD
            .refname_to_id("FETCH_HEAD")
            .or_else(|_| repo.refname_to_id("HEAD"))?;
        let head_str = head.to_string();

        let tree = {
            let commit = repo.find_commit(head)?;
            let tree = commit.tree()?;

            // See `UnsafeRepoTree`
            unsafe { std::mem::transmute::<git2::Tree<'_>, git2::Tree<'static>>(tree) }
        };

        Ok(Self {
            inner: index,
            head_str,
            rt: UnsafeRepoTree {
                repo,
                tree,
                _self_referential: PhantomPinned,
            },
        })
    }

    /// Fetches latest from the remote index repository. Note that using this
    /// method will mean no cache entries will be used, if a new commit is fetched
    /// from the repository, as their commit version will no longer match.
    pub fn retrieve(&mut self) -> Result<(), Error> {
        {
            let mut origin_remote = self
                .rt
                .repo
                .find_remote("origin")
                .or_else(|_| self.rt.repo.remote_anonymous(&self.inner.url))?;

            origin_remote.fetch(
                &["+HEAD:refs/remotes/origin/HEAD"],
                Some(&mut crate::fetch_opts()),
                None,
            )?;
        }

        let head = self
            .rt
            .repo
            .refname_to_id("FETCH_HEAD")
            .or_else(|_| self.rt.repo.refname_to_id("HEAD"))?;
        let head_str = head.to_string();

        let commit = self.rt.repo.find_commit(head)?;
        let tree = commit.tree()?;

        // See `UnsafeRepoTree`
        let tree = unsafe { std::mem::transmute::<git2::Tree<'_>, git2::Tree<'static>>(tree) };

        self.head_str = head_str;
        self.rt.tree = tree;

        Ok(())
    }

    /// Reads a crate from the index, it will attempt to use a cached entry if
    /// one is available, otherwise it will fallback to reading the crate
    /// directly from the git blob containing the crate information.
    pub fn crate_(&self, name: &str) -> Option<Crate> {
        let rel_path = match crate::crate_name_to_relative_path(name) {
            Some(rp) => rp,
            None => return None,
        };

        // Attempt to load the .cache/ entry first, this is purely an acceleration
        // mechanism and can fail for a few reasons that are non-fatal
        {
            let mut cache_path = self.inner.path.join(".cache");
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
        let entry = self.rt.tree.get_path(&Path::new(path))?;
        let object = entry.to_object(&self.rt.repo)?;
        let blob = object
            .as_blob()
            .ok_or_else(|| Error::Io(io::Error::new(io::ErrorKind::NotFound, path.to_owned())))?;

        Crate::from_slice(blob.content()).map_err(Error::Io)
    }

    /// Retrieve an iterator over all the crates in the index.
    /// skips crates that can not be parsed.
    #[inline]
    pub fn crates(&self) -> Crates<'_> {
        Crates {
            blobs: self.crates_refs(),
        }
    }

    /// Retrieve an iterator over all the crates in the index.
    /// Returns opaque reference for each crate in the index, which can be used with [`CrateRef::parse`]
    fn crates_refs(&self) -> CrateRefs<'_> {
        let mut stack = Vec::with_capacity(800);
        // Scan only directories at top level (skip config.json, etc.)
        for entry in self.rt.tree.iter() {
            let entry = entry.to_object(&self.rt.repo).unwrap();
            if entry.as_tree().is_some() {
                stack.push(entry);
            }
        }
        CrateRefs {
            stack,
            rt: &self.rt,
        }
    }

    /// Get the global configuration of the index.
    pub fn index_config(&self) -> Result<IndexConfig, Error> {
        let entry = self.rt.tree.get_path(&Path::new("config.json"))?;
        let object = entry.to_object(&self.rt.repo)?;
        let blob = object
            .as_blob()
            .ok_or_else(|| Error::Io(io::Error::new(io::ErrorKind::NotFound, "config.json")))?;
        serde_json::from_slice(blob.content()).map_err(Error::Json)
    }
}

/// Iterator over all crates in the index, but returns opaque objects that can be parsed separately.
///
/// See [`CrateRef::parse`].
struct CrateRefs<'a> {
    stack: Vec<git2::Object<'a>>,
    rt: &'a UnsafeRepoTree,
}

/// Opaque representation of a crate in the index. See [`CrateRef::parse`].
pub(crate) struct CrateRef<'a>(pub(crate) git2::Object<'a>);

impl CrateRef<'_> {
    #[inline]
    /// Parse a crate from [`BareIndex::crates_blobs`] iterator
    pub fn parse(&self) -> Option<Crate> {
        Crate::from_slice(self.as_slice()?).ok()
    }

    /// Raw crate data that can be parsed with [`Crate::from_slice`]
    pub fn as_slice(&self) -> Option<&[u8]> {
        Some(self.0.as_blob()?.content())
    }
}

impl<'a> Iterator for CrateRefs<'a> {
    type Item = CrateRef<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(last) = self.stack.pop() {
            match last.as_tree() {
                None => return Some(CrateRef(last)),
                Some(tree) => {
                    for entry in tree.iter().rev() {
                        self.stack.push(entry.to_object(&self.rt.repo).unwrap());
                    }
                    continue;
                }
            }
        }
        None
    }
}

pub struct Crates<'a> {
    blobs: CrateRefs<'a>,
}

impl<'a> Iterator for Crates<'a> {
    type Item = Crate;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(next) = self.blobs.next() {
            if let Some(k) = CrateRef::parse(&next) {
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

    if canonical.ends_with(".git") {
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
                "https://dl.cloudsmith.io/aBcW1234aBcW1234/embark/rust/cargo/index".to_owned()
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
        use super::BareIndex;

        let tmp_dir = tempdir::TempDir::new("bare_iterator").unwrap();

        let index = BareIndex::with_path(tmp_dir.path().to_owned(), crate::INDEX_GIT_URL);

        let repo = index
            .open_or_clone()
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
        use super::BareIndex;

        let tmp_dir = tempdir::TempDir::new("clones_bare_index").unwrap();

        let index = BareIndex::with_path(tmp_dir.path().to_owned(), crate::INDEX_GIT_URL);

        let mut repo = index
            .open_or_clone()
            .expect("Failed to clone crates.io index");

        fn test_sval(repo: &super::BareIndexRepo<'_>) {
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

        repo.retrieve().expect("Failed to fetch crates.io index");

        test_sval(&repo);
    }

    #[test]
    fn opens_bare_index() {
        use super::BareIndex;

        let tmp_dir = tempdir::TempDir::new("opens_bare_index").unwrap();

        let index = BareIndex::with_path(tmp_dir.path().to_owned(), crate::INDEX_GIT_URL);

        {
            let _ = index
                .open_or_clone()
                .expect("Failed to clone crates.io index");
        }

        let mut repo = index
            .open_or_clone()
            .expect("Failed to open crates.io index");

        fn test_sval(repo: &super::BareIndexRepo<'_>) {
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

        repo.retrieve().expect("Failed to fetch crates.io index");

        test_sval(&repo);
    }
}
