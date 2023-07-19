#![allow(clippy::result_large_err)]
#[cfg(feature = "changes")]
use crate::changes::ChangesIter;
use crate::dedupe::DedupeContext;
use crate::dirs::get_index_details;
use crate::error::GixError;
use crate::{path_max_byte_len, Crate, Error, IndexConfig};
use gix::config::tree::Key;
use std::io;
use std::path::{Path, PathBuf};

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
    head_commit: gix::ObjectId,
    head_commit_hex: String,
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
        let mut mapping = gix::sec::trust::Mapping::default();
        let open_with_complete_config =
            gix::open::Options::default().permissions(gix::open::Permissions {
                config: gix::open::permissions::Config {
                    // Be sure to get all configuration, some of which is only known by the git binary.
                    // That way we are sure to see all the systems credential helpers
                    git_binary: true,
                    ..Default::default()
                },
                ..Default::default()
            });
        mapping.reduced = open_with_complete_config.clone();
        mapping.full = open_with_complete_config.clone();
        let repo = gix::ThreadSafeRepository::discover_opts(
            &path,
            gix::discover::upwards::Options::default().apply_environment(),
            mapping,
        )
        .ok()
        .map(|repo| repo.to_thread_local())
        .filter(|repo| {
            // The `cargo` standard registry clone has no configured origin (when created with `git2`).
            repo.find_remote("origin").map_or(true, |remote| {
                remote
                    .url(gix::remote::Direction::Fetch)
                    .map_or(false, |remote_url| remote_url.to_bstring() == url)
            })
        });

        let repo = match repo {
            Some(repo) => repo,
            None => {
                let _lock = gix::lock::Marker::acquire_to_hold_resource(
                    path.with_extension("crates-index"),
                    gix::lock::acquire::Fail::AfterDurationWithBackoff(
                        std::time::Duration::from_secs(60 * 10),
                    ),
                    Some(PathBuf::from_iter(Some(std::path::Component::RootDir))),
                )
                .map_err(GixError::from)?;

                match gix::open_opts(&path, open_with_complete_config).ok() {
                    None => clone_url(&url, &path)?,
                    Some(repo) => repo,
                }
            }
        };

        let head_commit = Self::find_repo_head(&repo, &path)?;
        let git2_repo = git2::Repository::open(repo.path()).expect("valid repo opens fine");
        Ok(Self {
            path,
            git2_repo,
            git2_head: git2::Oid::from_bytes(head_commit.as_slice()).expect("valid head id"),
            url,
            repo,
            head_commit_hex: head_commit.to_hex().to_string(),
            head_commit,
        })
    }

    fn tree(&self) -> Result<gix::Tree<'_>, GixError> {
        Ok(self
            .repo
            .find_object(self.head_commit)?
            .try_into_commit()?
            .tree()?)
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
        (|| -> Result<(), GixError> {
            let mut remote = self.repo.find_remote("origin").ok().unwrap_or_else(|| {
                self.repo
                    .remote_at(self.url.as_str())
                    .expect("own URL is always valid")
            });
            remote.replace_refspecs(
                [
                    "HEAD:refs/remotes/origin/HEAD",
                    "master:refs/remotes/origin/master",
                ],
                gix::remote::Direction::Fetch,
            )?;

            remote
                .connect(gix::remote::Direction::Fetch)?
                .prepare_fetch(gix::progress::Discard, Default::default())?
                .receive(gix::progress::Discard, &gix::interrupt::IS_INTERRUPTED)?;
            Ok(())
        })()?;

        let head_commit = Self::find_repo_head(&self.repo, &self.path)?;
        self.head_commit = head_commit;
        self.head_commit_hex = head_commit.to_hex().to_string();

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
            let mut cache_path =
                PathBuf::with_capacity(path_max_byte_len(&self.path) + 8 + rel_path.len());
            cache_path.push(&self.path);
            cache_path.push(".cache");
            cache_path.push(&rel_path);
            if let Ok(cache_bytes) = std::fs::read(&cache_path) {
                if let Ok(krate) =
                    Crate::from_cache_slice(&cache_bytes, Some(&self.head_commit_hex))
                {
                    return Some(krate);
                }
            }
        }

        // Fallback to reading the blob directly via git if we don't have a
        // valid cache entry
        self.crate_from_rel_path(rel_path).ok()
    }

    fn crate_from_rel_path(&self, rel_path: String) -> Result<Crate, Error> {
        let object = self.object_at_path(rel_path.into())?;
        Crate::from_slice(&object.data).map_err(Error::Io)
    }

    /// Single-threaded iterator over all the crates in the index.
    ///
    /// [`Index::crates_parallel`] is typically 4 times faster.
    ///
    /// Skips crates that can not be parsed (but there shouldn't be any such crates in the crates-io index).
    /// Also consider to enable `git-index-performance` feature toggle for better performance.
    #[inline]
    #[must_use]
    pub fn crates(&self) -> Crates<'_> {
        Crates {
            blobs: self.crates_blobs().expect("HEAD commit disappeared"),
            dedupe: MaybeOwned::Owned(DedupeContext::new()),
        }
    }

    /// Iterate over all crates using rayon.
    ///
    /// This method is available only if the "parallel" feature is enabled.
    /// Also consider to enable `git-index-performance` feature toggle for better performance.
    #[cfg(feature = "parallel")]
    #[must_use]
    pub fn crates_parallel(
        &self,
    ) -> impl rayon::iter::ParallelIterator<Item = Result<Crate, crate::error::CratesIterError>> + '_
    {
        use rayon::iter::{IntoParallelIterator, ParallelIterator};
        let tree_oids = match self.crates_top_level_ids() {
            Ok(objs) => objs,
            Err(_) => vec![self.repo.object_hash().null()], // intentionally broken oid to return error from the iterator
        };

        tree_oids
            .into_par_iter()
            .map_init(
                {
                    let repo = self.repo.clone().into_sync();
                    move || {
                        (
                            {
                                let mut repo = repo.to_thread_local();
                                repo.objects.unset_pack_cache();
                                repo
                            },
                            DedupeContext::new(),
                        )
                    }
                },
                |(repo, ctx), oid| {
                    let mut stack = Vec::with_capacity(64);
                    match repo.find_object(oid) {
                        Ok(obj) => stack.push(obj.detach()),
                        Err(_) => return vec![Err(crate::error::CratesIterError)],
                    };
                    let blobs = CratesTreesToBlobs {
                        stack,
                        repo: repo.clone(),
                    };
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

    fn crates_blobs(&self) -> Result<CratesTreesToBlobs, GixError> {
        let repo = with_delta_cache(self.repo.clone());
        Ok(CratesTreesToBlobs {
            stack: self
                .crates_top_level_ids()?
                .into_iter()
                .map(|id| self.repo.find_object(id).map(|tree| tree.detach()))
                .collect::<Result<_, _>>()?,
            repo,
        })
    }

    fn crates_top_level_ids(&self) -> Result<Vec<gix::ObjectId>, GixError> {
        let mut stack = Vec::with_capacity(800);
        for entry in self.tree()?.iter() {
            let entry = entry?;
            // crates are in directories no longer than 2 letters.
            if !is_top_level_dir(&entry) {
                continue;
            };
            stack.push(entry.oid());
        }
        Ok(stack)
    }

    /// Get the global configuration of the index.
    pub fn index_config(&self) -> Result<IndexConfig, Error> {
        let blob = self.object_at_path("config.json".into())?;
        serde_json::from_slice(&blob.data).map_err(Error::Json)
    }

    fn object_at_path(&self, path: PathBuf) -> Result<gix::Object<'_>, GixError> {
        let entry = self
            .tree()?
            .lookup_entry_by_path(&path)?
            .ok_or(GixError::PathMissing { path })?;
        Ok(entry.object()?)
    }

    fn find_repo_head(repo: &gix::Repository, path: &Path) -> Result<gix::ObjectId, Error> {
        repo.head_id().ok()
            .filter(|id| id.header().map_or(false, |h| h.kind().is_commit()))
            .or_else(|| repo.find_reference("origin/master").ok().and_then(|r| r.try_id()))
            .map(|id| id.detach())
            .ok_or_else(|| {
                // TODO: The Error enum lacks a proper variant for this case
                Error::Url(format!("The repo at path {} is unusable due to having an invalid HEAD reference nor origin/master", path.display()))
            })
    }
}

fn is_top_level_dir(entry: &gix::object::tree::EntryRef<'_, '_>) -> bool {
    entry.mode().is_tree() && entry.filename().len() <= 2
}

fn with_delta_cache(mut repo: gix::Repository) -> gix::Repository {
    if repo
        .config_snapshot()
        .integer_by_key(
            gix::config::tree::Core::DELTA_BASE_CACHE_LIMIT
                .logical_name()
                .as_str(),
        )
        .is_none()
    {
        let mut config = repo.config_snapshot_mut();
        // Set a memory-backed delta-cache to the same size as git for ~40% more speed in this workload.
        config
            .set_value(&gix::config::tree::Core::DELTA_BASE_CACHE_LIMIT, "96m")
            .expect("in memory always works");
    }
    repo
}

fn clone_url(url: &str, destination: &Path) -> Result<gix::Repository, GixError> {
    // Clones and fetches already know they need `bin_config` to work, so nothing to do here.
    let (repo, _outcome) = gix::prepare_clone_bare(url, destination)?
        .with_remote_name("origin")?
        .configure_remote(|remote| {
            Ok(remote.with_refspecs(
                [
                    "HEAD:refs/remotes/origin/HEAD",
                    "master:refs/remotes/origin/master",
                ],
                gix::remote::Direction::Fetch,
            )?)
        })
        .fetch_only(gix::progress::Discard, &gix::interrupt::IS_INTERRUPTED)?;
    Ok(repo)
}

/// Iterator over all crates in the index, but returns opaque objects that can be parsed separately.
struct CratesTreesToBlobs {
    stack: Vec<gix::ObjectDetached>,
    repo: gix::Repository,
}

/// Opaque representation of a crate in the index. See [`CrateUnparsed::parse`].
struct CrateUnparsed(Vec<u8>);

impl CrateUnparsed {
    #[inline]
    fn parse(&self, ctx: &mut DedupeContext) -> io::Result<Crate> {
        Crate::from_slice_with_context(self.0.as_slice(), ctx)
    }
}

impl Iterator for CratesTreesToBlobs {
    type Item = CrateUnparsed;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(obj) = self.stack.pop() {
            if obj.kind.is_tree() {
                let tree = gix::objs::TreeRef::from_bytes(&obj.data).unwrap();
                for entry in tree.entries.into_iter().rev() {
                    self.stack
                        .push(self.repo.find_object(entry.oid).unwrap().detach());
                }
                continue;
            } else {
                return Some(CrateUnparsed(obj.data));
            }
        }
        None
    }
}

enum MaybeOwned<'a, T> {
    Owned(T),
    Borrowed(&'a mut T),
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

#[cfg(test)]
#[cfg(feature = "https")]
mod test {
    use super::*;
    use gix::bstr::ByteSlice;

    #[test]
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
                break;
            }
        }
        assert!(found_first_crate);
        assert!(found_second_crate);
    }

    #[test]
    fn clones_bare_index() {
        let tmp_dir = tempfile::TempDir::new().unwrap();
        let path = tmp_dir.path().join("some/sub/dir/testing/abc");

        let mut repo =
            Index::with_path(path, crate::INDEX_GIT_URL).expect("Failed to clone crates.io index");

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
        let _config = index
            .index_config()
            .expect("we are able to obtain and parse the configuration of the default registry");
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
    #[cfg_attr(debug_assertions, ignore = "too slow in debug mode")]
    fn test_can_parse_all() {
        let index = shared_index();

        let mut ctx = DedupeContext::new();

        let mut found_gcc_crate = false;
        for c in index.crates_blobs().unwrap() {
            match c.parse(&mut ctx) {
                Ok(c) => {
                    if c.name() == "gcc" {
                        found_gcc_crate = true;
                    }
                }
                Err(e) => panic!("can't parse :( {:?}: {e}", c.0.as_bstr()),
            }
        }

        assert!(found_gcc_crate);
    }

    fn shared_index() -> Index {
        let index_path = "tests/testdata/git-registry";
        if is_ci::cached() {
            Index::new_cargo_default()
                .expect("CI has just cloned this index and its ours and valid")
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
