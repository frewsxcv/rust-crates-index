use crate::dedupe::DedupeContext;
use crate::dirs::url_to_local_dir;
use crate::{error::CratesIterError, path_max_byte_len, Crate, Error, IndexConfig};
use git2::Repository;
use std::fmt;
use std::path::{Path, PathBuf};
use std::{fs, io};

/// https://doc.rust-lang.org/cargo/reference/config.html#hierarchical-structure
fn find_cargo_config() -> Option<PathBuf> {
    if let Ok(current) = std::env::current_dir() {
        let mut base = current;
        loop {
            let mut path = base.join(".cargo");
            path.push("config.toml");
            if path.exists() {
                return Some(path);
            }
            if !base.pop() {
                break;
            }
        }
    }
    if let Ok(home) = home::cargo_home() {
        let path = home.join("config.toml");
        if path.exists() {
            return Some(path);
        }
    }
    None
}

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
        let config: toml::Value;
        let url = if let Some(path) = find_cargo_config() {
            config = toml::from_str(&fs::read_to_string(path)?)
                .map_err(Error::Toml)?;
            config.get("source").and_then(|sources|
                sources.get("crates-io")
                    .and_then(|v| v.get("replace-with"))
                    .and_then(|v| v.as_str())
                    .and_then(|v| sources.get(v))
                    .and_then(|v| v.get("registry"))
                    .and_then(|v| v.as_str()))
        } else {
            None
        };
        Self::from_url(url.unwrap_or(crate::INDEX_GIT_URL))
    }

    /// Creates a bare index from a provided URL, opening the same location on
    /// disk that Cargo uses for that registry index.
    ///
    /// It can be used to access custom registries.
    pub fn from_url(url: &str) -> Result<Self, Error> {
        let (dir_name, canonical_url) = url_to_local_dir(url)?;
        let mut path = home::cargo_home()?;

        path.push("registry");
        path.push("index");
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
    #[must_use]
    pub fn path(&self) -> &Path {
        &self.path
    }

    /// Get the index url.
    #[inline]
    #[must_use]
    pub fn url(&self) -> &str {
        &self.url
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
            if let Some(parent) = path.parent() {
                let _ = std::fs::create_dir_all(parent);
            }
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

        let head = Self::find_valid_repo_head(&repo, &path)?;

        Ok(Self {
            path,
            url,
            repo,
            head_str: head.to_string(),
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
    #[must_use]
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

        let head = Self::find_valid_repo_head(&self.repo, &self.path)?;

        self.head = head;
        self.head_str = self.head.to_string();

        Ok(())
    }

    /// Reads a crate from the index, it will attempt to use a cached entry if
    /// one is available, otherwise it will fallback to reading the crate
    /// directly from the git blob containing the crate information.
    ///
    /// Use this only if you need to get very few crates. If you're going
    /// to read majority of crates, prefer the [`Index::crates()`] iterator.
    #[must_use]
    pub fn crate_(&self, name: &str) -> Option<Crate> {
        let rel_path = crate::crate_name_to_relative_path(name)?;

        // Attempt to load the .cache/ entry first, this is purely an acceleration
        // mechanism and can fail for a few reasons that are non-fatal
        {
            // avoid realloc on each push
            let mut cache_path = PathBuf::with_capacity(path_max_byte_len(&self.path) + 8 + rel_path.len());
            cache_path.push(&self.path);
            cache_path.push(".cache");
            cache_path.push(&rel_path);
            if let Ok(cache_bytes) = std::fs::read(&cache_path) {
                if let Ok(krate) = Crate::from_cache_slice(&cache_bytes, Some(&self.head_str)) {
                    return Some(krate);
                }
            }
        }

        // Fallback to reading the blob directly via git if we don't have a
        // valid cache entry
        self.crate_from_rel_path(&rel_path).ok()
    }

    fn crate_from_rel_path(&self, path: &str) -> Result<Crate, Error> {
        let entry = self.tree()?.get_path(Path::new(path))?;
        let object = entry.to_object(&self.repo)?;
        let blob = object
            .as_blob()
            .ok_or_else(|| Error::Io(io::Error::new(io::ErrorKind::NotFound, path.to_owned())))?;

        Crate::from_slice(blob.content()).map_err(Error::Io)
    }

    /// Single-threaded iterator over all the crates in the index.
    ///
    /// [`Index::crates_parallel`] is typically 3 times faster.
    ///
    /// Skips crates that can not be parsed (but there shouldn't be any such crates in the crates-io index).
    #[inline]
    #[must_use]
    pub fn crates(&self) -> Crates<'_> {
        Crates {
            blobs: self.crates_refs().expect("HEAD commit disappeared"),
            dedupe: MaybeOwned::Owned(DedupeContext::new()),
        }
    }

    /// Iterate over all crates using rayon.
    ///
    /// This method is available only if the "parallel" feature is enabled.
    #[cfg(feature = "parallel")]
    #[must_use] pub fn crates_parallel(&self) -> impl rayon::iter::ParallelIterator<Item=Result<Crate, CratesIterError>> + '_ {
        use rayon::iter::{IntoParallelIterator, ParallelIterator, IndexedParallelIterator};

        let tree_oids = match self.crates_top_level_refs() {
            Ok(objs) => objs.into_iter().map(|obj| obj.id()).collect::<Vec<_>>(),
            Err(_) => vec![git2::Oid::zero()], // intentionally broken oid to return error from the iterator
        };

        let path = self.repo.path();

        tree_oids.into_par_iter()
            .with_min_len(64)
            .map_init(
                move || (Repository::open_bare(path), DedupeContext::new()),
                |(repo, ctx), oid| {
                    let repo = match repo.as_ref() {
                        Ok(repo) => repo,
                        Err(_) => return vec![Err(CratesIterError)],
                    };
                    let mut stack = Vec::with_capacity(64);
                    match repo.find_object(oid, None) {
                        Ok(obj) => stack.push(obj),
                        Err(_) => return vec![Err(CratesIterError)],
                    };
                    let blobs = CratesRefs { stack, repo };
                    Crates {
                        blobs,
                        dedupe: MaybeOwned::Borrowed(ctx),
                    }
                    .map(Ok)
                    .collect::<Vec<_>>()
                },
            )
            .flat_map_iter(|chunk| chunk.into_iter())
    }

    /// update an iterator over all the crates in the index.
    /// Returns opaque reference for each crate in the index, which can be used with [`CrateRef::parse`]
    pub(crate) fn crates_refs(&self) -> Result<CratesRefs<'_>, git2::Error> {
        Ok(CratesRefs {
            stack: self.crates_top_level_refs()?,
            repo: &self.repo,
        })
    }

    pub(crate) fn crates_top_level_refs(&self) -> Result<Vec<git2::Object<'_>>, git2::Error> {
        let mut stack = Vec::with_capacity(800);
        for entry in self.tree()?.iter() {
            // crates are in short dirs, skip .git/.cache
            if entry.name_bytes().len() <= 2 {
                let entry = entry.to_object(&self.repo)?;
                // Scan only directories at top level
                if entry.as_tree().is_some() {
                    stack.push(entry);
                }
            }
        }
        Ok(stack)
    }

    /// Get the global configuration of the index.
    pub fn index_config(&self) -> Result<IndexConfig, Error> {
        let entry = self.tree()?.get_path(Path::new("config.json"))?;
        let object = entry.to_object(&self.repo)?;
        let blob = object
            .as_blob()
            .ok_or_else(|| Error::Io(io::Error::new(io::ErrorKind::NotFound, "config.json")))?;
        serde_json::from_slice(blob.content()).map_err(Error::Json)
    }

    fn find_valid_repo_head(repo: &Repository, path: &Path) -> Result<git2::Oid, Error> {
        repo.refname_to_id("FETCH_HEAD")
            .or_else(|_| repo.refname_to_id("HEAD"))
            .and_then(|head| {
                // Users of sparse registry reported git failures due to missing oids,
                // which isn't supposed to happen.
                let _ = repo.find_commit(head)?;
                Ok(head)
            })
            .map_err(|e| {
                // TODO: The Error enum lacks a proper variant for this case
                Error::Url(format!("The repo at path {} is unusable due to having an invalid HEAD reference: {e}", path.display()))
            })
    }
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
    pub fn parse(&self, ctx: &mut DedupeContext) -> io::Result<Crate> {
        let blob = self.as_slice().ok_or(io::ErrorKind::InvalidData)?;
        Crate::from_slice_with_context(blob, ctx)
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
                        self.stack.push(entry.to_object(self.repo).unwrap());
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

enum MaybeOwned<'a, T> {
    Owned(T),
    Borrowed(&'a mut T),
}

/// Iterator over all crates in the index. Skips crates that failed to parse.
pub struct Crates<'a> {
    blobs: CratesRefs<'a>,
    dedupe: MaybeOwned<'a, DedupeContext>,
}

impl<'a> Iterator for Crates<'a> {
    type Item = Crate;

    fn next(&mut self) -> Option<Self::Item> {
        for next in self.blobs.by_ref() {
            let dedupe = match &mut self.dedupe {
                MaybeOwned::Owned(d) => d,
                MaybeOwned::Borrowed(d) => d,
            };
            if let Ok(k) = CrateRef::parse(&next, dedupe) {
                return Some(k);
            }
        }
        None
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn bare_iterator() {
        use super::Index;

        let tmp_dir = tempfile::TempDir::new().unwrap();

        let repo = Index::with_path(tmp_dir.path().to_owned(), crate::INDEX_GIT_URL)
            .expect("Failed to clone crates.io index");
        assert_eq!("time", repo.crate_("time").unwrap().name());

        let mut found_gcc_crate = false;
        let mut found_time_crate = false;

        for c in repo.crates() {
            if c.name() == "gcc" {
                found_gcc_crate = true;
            }
            if c.name() == "time" {
                found_time_crate = true;
            }
        }

        assert!(found_gcc_crate);
        assert!(found_time_crate);
    }

    #[test]
    fn clones_bare_index() {
        use super::Index;

        let tmp_dir = tempfile::TempDir::new().unwrap();
        let path = tmp_dir.path().join("some/sub/dir/testing/abc");

        let mut repo = Index::with_path(path, crate::INDEX_GIT_URL)
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

        let tmp_dir = tempfile::TempDir::new().unwrap();

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

    #[test]
    fn reads_replaced_source() {
        use super::Index;

        let index = Index::new_cargo_default();
        assert!(index.is_ok());
        assert!(index.unwrap().index_config().is_ok());
    }
}
