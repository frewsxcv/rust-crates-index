#[cfg(feature = "changes")]
use crate::changes::ChangesIter;
use crate::dedupe::DedupeContext;
use crate::dirs::get_index_details;
use crate::{path_max_byte_len, Crate, Error, IndexConfig};
use git2::Repository;
use std::fmt;
use std::path::{Path, PathBuf};
use std::io;
use crate::error::GixError;

/// The default URL of the crates.io index for use with git, see [`Index::with_path`]
pub const INDEX_GIT_URL: &str = "https://github.com/rust-lang/crates.io-index";

pub(crate) fn fetch_opts<'cb>() -> git2::FetchOptions<'cb> {
    let mut proxy_opts = git2::ProxyOptions::new();
    proxy_opts.auto();
    let mut fetch_opts = git2::FetchOptions::new();
    fetch_opts.proxy_options(proxy_opts);

    let mut remote_callbacks = git2::RemoteCallbacks::new();
    remote_callbacks.credentials(|url, username_from_url, allowed_types| {
        let config = git2::Config::open_default()?;

        if allowed_types.contains(git2::CredentialType::USER_PASS_PLAINTEXT) {
            if let Some((username, password)) = git2::CredentialHelper::new(url)
                .config(&config)
                .username(username_from_url)
                .execute()
            {
                let cred = git2::Cred::userpass_plaintext(&username, &password)?;
                return Ok(cred);
            }
        }

        #[cfg(feature = "ssh")]
        if allowed_types.contains(git2::CredentialType::SSH_KEY) {
            if let Some(username) = username_from_url {
                if let Ok(cred) = git2::Cred::ssh_key_from_agent(username) {
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

/// Wrapper around managing the crates.io-index git repository
///
/// Uses a "bare" git index that fetches files directly from the repo instead of local checkout.
/// Uses Cargo's cache.
pub struct Index {
    path: PathBuf,
    url: String,

    pub(crate) git2_repo: git2::Repository,
    repo: gix::Repository,
    pub(crate) git2_head: git2::Oid,
    head: gix::ObjectId,
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
    ///
    /// Note this function takes the `CARGO_HOME` environment variable into account
    #[inline]
    pub fn new_cargo_default() -> Result<Self, Error> {
        let url = crate::config::get_crates_io_replacement(None, None)?;
        Self::from_url(url.as_deref().unwrap_or(crate::INDEX_GIT_URL))
    }

    /// Creates a bare index from a provided URL, opening the same location on
    /// disk that Cargo uses for that registry index.
    ///
    /// It can be used to access custom registries.
    pub fn from_url(url: &str) -> Result<Self, Error> {
        let (path, canonical_url) = get_index_details(url, None)?;
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

    /// List crates that have changed (published or yanked), in reverse chronological order.
    ///
    /// This iterator is aware of periodic index squashing crates.io performs,
    /// and will perform (slow and blocking) network requests to fetch the additional history from <https://github.com/rust-lang/crates.io-index-archive> if needed.
    ///
    /// If you want to track newly added/changed crates over time, make a note of the last [`commit`](crate::changes::Change::commit) or [`timestamp`](crate::changes::Change) you've processed,
    /// and stop iteration on it next time.
    ///
    /// Crates will be reported multiple times, once for each publish/yank/unyank event that happened.
    ///
    /// If you like to know publication dates of all crates, consider <https://crates.io/data-access> instead.
    #[cfg(feature = "changes")]
    pub fn changes(&self) -> Result<ChangesIter<'_>, Error> {
        Ok(ChangesIter::new(self)?)
    }

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
            let _lock = gix::lock::Marker::acquire_to_hold_resource(path.with_extension("crates-index"), 
                                                        gix::lock::acquire::Fail::AfterDurationWithBackoff(std::time::Duration::from_secs(60 * 10)), 
                                                        None).unwrap(); // TODO(git): normal error handling once `gix` is available.
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
                    Some(&mut fetch_opts()),
                    None,
                )?;
            }
            repo
        } else {
            git2::Repository::open(&path)?
        };

        let head = Self::git2_find_valid_repo_head(&repo, &path)?;

        Ok(Self {
            path,
            url,
            repo: gix::open_opts(repo.path(), gix::open::Options::isolated()).expect("loads fine as `git2` can also do it"),
            git2_repo: repo,
            head_str: head.to_string(),
            git2_head: head,
            head: gix::ObjectId::from(head.as_bytes())
        })
    }

    fn git2_tree(&self) -> Result<git2::Tree<'_>, git2::Error> {
        let commit = self.git2_repo.find_commit(self.git2_head)?;
        commit.tree()
    }
    
    fn tree(&self) -> Result<gix::Tree<'_>, GixError> {
        Ok(self.repo.find_object(self.head)?.try_into_commit()?.tree()?)
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
                .git2_repo
                .find_remote("origin")
                .or_else(|_| self.git2_repo.remote_anonymous(&self.url))?;

            origin_remote.fetch(
                &[
                    "HEAD:refs/remotes/origin/HEAD",
                    "master:refs/remotes/origin/master",
                ],
                Some(&mut fetch_opts()),
                None,
            )?;
        }

        let head = Self::git2_find_valid_repo_head(&self.git2_repo, &self.path)?;

        self.git2_head = head;
        self.head_str = self.git2_head.to_string();

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
        let rel_path = crate::crate_name_to_relative_path(name, None)?;

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
        let entry = self.git2_tree()?.get_path(Path::new(path))?;
        let object = entry.to_object(&self.git2_repo)?;
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
    pub fn crates(&self) -> Git2Crates<'_> {
        Git2Crates {
            blobs: self.crates_refs().expect("HEAD commit disappeared"),
            dedupe: MaybeOwned::Owned(DedupeContext::new()),
        }
    }

    /// Iterate over all crates using rayon.
    ///
    /// This method is available only if the "parallel" feature is enabled. 
    /// Also consider to enable `git-index-performance` feature toggle for better performance.
    #[cfg(feature = "parallel")]
    #[must_use] pub fn crates_parallel(&self) -> impl rayon::iter::ParallelIterator<Item=Result<Crate, crate::error::CratesIterError>> + '_ {
        use rayon::iter::{IntoParallelIterator, ParallelIterator};
        let tree_oids = match self.crates_top_level_ids() {
            Ok(objs) => objs,
            Err(_) => vec![self.repo.object_hash().null()], // intentionally broken oid to return error from the iterator
        };

        tree_oids.into_par_iter()
            .map_init(
                {
                    let repo = self.repo.clone().into_sync();
                    move || ({
                                 let mut repo = repo.to_thread_local(); 
                                 repo.objects.unset_pack_cache();
                                 repo
                             }, DedupeContext::new())
                },
                |(repo, ctx), oid| {
                    let mut stack = Vec::with_capacity(64);
                    match repo.find_object(oid) {
                        Ok(obj) => stack.push(obj.detach()),
                        Err(_) => return vec![Err(crate::error::CratesIterError)],
                    };
                    let blobs = CratesTreesToBlobs { stack, repo: repo.clone() };
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
    /// Returns opaque reference for each crate in the index, which can be used with [`Git2CrateRef::parse`]
    fn crates_refs(&self) -> Result<Git2CratesRefs<'_>, git2::Error> {
        Ok(Git2CratesRefs {
            stack: self.git2_crates_top_level_refs()?,
            repo: &self.git2_repo,
        })
    }

    fn git2_crates_top_level_refs(&self) -> Result<Vec<git2::Object<'_>>, git2::Error> {
        let mut stack = Vec::with_capacity(800);
        for entry in self.git2_tree()?.iter() {
            // crates are in short dirs, skip .git/.cache
            if entry.name_bytes().len() <= 2 {
                let entry = entry.to_object(&self.git2_repo)?;
                // Scan only directories at top level
                if entry.as_tree().is_some() {
                    stack.push(entry);
                }
            }
        }
        Ok(stack)
    }
    
    fn crates_top_level_ids(&self) -> Result<Vec<gix::ObjectId>, GixError> {
        let mut stack = Vec::with_capacity(800);
        for entry in self.tree()?.iter() {
            let entry = entry?;
            // crates are in directories no longer than 2 letters. 
            if !is_top_level_dir(&entry) {
                continue
            };
            stack.push(entry.oid());
        }
        Ok(stack)
    }

    /// Get the global configuration of the index.
    pub fn index_config(&self) -> Result<IndexConfig, Error> {
        let entry = self.git2_tree()?.get_path(Path::new("config.json"))?;
        let object = entry.to_object(&self.git2_repo)?;
        let blob = object
            .as_blob()
            .ok_or_else(|| Error::Io(io::Error::new(io::ErrorKind::NotFound, "config.json")))?;
        serde_json::from_slice(blob.content()).map_err(Error::Json)
    }

    fn git2_find_valid_repo_head(repo: &Repository, path: &Path) -> Result<git2::Oid, Error> {
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

fn is_top_level_dir(entry: &gix::object::tree::EntryRef<'_, '_>) -> bool {
    // TODO: probably `EntryMode` should be part of `gix::object::tree`
    entry.mode() == gix::objs::tree::EntryMode::Tree && entry.filename().len() <= 2
}

/// Iterator over all crates in the index, but returns opaque objects that can be parsed separately.
///
/// See [`Git2CrateRef::parse`].
struct Git2CratesRefs<'a> {
    stack: Vec<git2::Object<'a>>,
    repo: &'a git2::Repository,
}

/// Iterator over all crates in the index, but returns opaque objects that can be parsed separately.
///
/// See [`Git2CrateRef::parse`].
struct CratesTreesToBlobs {
    stack: Vec<gix::ObjectDetached>,
    repo: gix::Repository,
}

/// Opaque representation of a crate in the index. See [`Git2CrateRef::parse`].
struct Git2CrateRef<'a>(git2::Object<'a>);

/// Opaque representation of a crate in the index. See [`Git2CrateRef::parse`].
struct CrateUnparsed(Vec<u8>);

impl Git2CrateRef<'_> {
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

impl CrateUnparsed{
    #[inline]
    /// Parse a crate from [`Index::crates_blobs`] iterator
    fn parse(&self, ctx: &mut DedupeContext) -> io::Result<Crate> {
        Crate::from_slice_with_context(self.0.as_slice(), ctx)
    }
}

impl<'a> Iterator for Git2CratesRefs<'a> {
    type Item = Git2CrateRef<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(last) = self.stack.pop() {
            match last.as_tree() {
                None => return Some(Git2CrateRef(last)),
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

impl Iterator for CratesTreesToBlobs {
    type Item = CrateUnparsed;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(obj) = self.stack.pop() {
            if obj.kind == gix::object::Kind::Tree {
                let tree = gix::objs::TreeRef::from_bytes(&obj.data).unwrap();
                for entry in tree.entries.into_iter().rev() {
                    self.stack.push(self.repo.find_object(entry.oid).unwrap().detach());
                }
                continue;
            } else {
                return Some(CrateUnparsed(obj.data))
            }
        }
        None
    }
}

impl fmt::Debug for Git2CrateRef<'_> {
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
pub struct Git2Crates<'a> {
    blobs: Git2CratesRefs<'a>,
    dedupe: MaybeOwned<'a, DedupeContext>,
}

/// Iterator over all crates in the index. Skips crates that failed to parse.
pub struct Crates<'a> {
    blobs: CratesTreesToBlobs,
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
            if let Ok(k) = CrateUnparsed::parse(&next, dedupe) {
                return Some(k);
            }
        }
        None
    }
}

impl<'a> Iterator for Git2Crates<'a> {
    type Item = Crate;

    fn next(&mut self) -> Option<Self::Item> {
        for next in self.blobs.by_ref() {
            let dedupe = match &mut self.dedupe {
                MaybeOwned::Owned(d) => d,
                MaybeOwned::Borrowed(d) => d,
            };
            if let Ok(k) = Git2CrateRef::parse(&next, dedupe) {
                return Some(k);
            }
        }
        None
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    #[cfg(feature = "https")]
    fn bare_iterator() {
        let repo = shared_index();
        assert_eq!("time", repo.crate_("time").unwrap().name());

        let mut found_first_crate = false;
        let mut found_second_crate = false;

        // Note that crates are roughly ordered in reverse.
        for c in repo.crates() {
            if c.name() == "zzzz" {
                found_first_crate = true;
            } else if c.name() == "zulip" {
                found_second_crate = true;
            }
            if found_first_crate && found_second_crate {
                break
            }
        }
        assert!(found_first_crate);
        assert!(found_second_crate);
    }

    #[test]
    #[cfg(feature = "https")]
    fn clones_bare_index() {
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
    #[cfg(feature = "https")]
    fn opens_bare_index() {
        let mut repo = shared_index();
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
        let index = shared_index();
        let _config = index.index_config().expect("we are able to obtain and parse the configuration of the default registry");
    }

    #[test]
    fn test_dependencies() {
        let index = shared_index();

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
    #[cfg(feature = "https")]
    fn test_cargo_default_updates() {
        let mut index = shared_index();
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
    #[cfg(feature = "https")]
    fn test_can_parse_all() {
        let index = shared_index();
        
        let mut ctx = DedupeContext::new();
        let mut found_gcc_crate = false;
        for c in index.crates_refs().unwrap() {
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
    
    fn shared_index() -> Index {
        let index_path = "tests/testdata/git-registry";
        if is_ci::cached() {
            Index::new_cargo_default() .expect("CI has just cloned this index and its ours and valid")
        } else {
            Index::with_path(index_path, INDEX_GIT_URL).expect("clone works and there is no racing")
        }
    }

    #[test]
    fn matches_cargo() {
        assert_eq!(
            crate::dirs::url_to_local_dir(crate::INDEX_GIT_URL).unwrap(),
            (
                "github.com-1ecc6299db9ec823".to_owned(),
                crate::INDEX_GIT_URL.to_owned()
            )
        );

        // Ensure we actually strip off the irrelevant parts of a url, note that
        // the .git suffix is not part of the canonical url, but *is* used when hashing
        assert_eq!(
            crate::dirs::url_to_local_dir(&format!(
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
}
