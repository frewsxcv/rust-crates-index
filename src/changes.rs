use crate::bare_index::fetch_opts;
use crate::Error;
use crate::Index;
use git2::{Commit, Tree, Oid};
use std::collections::{VecDeque, HashSet};
use std::convert::TryInto;
use std::time::{SystemTime, Duration};

const INDEX_GIT_ARCHIVE_URL: &str = "https://github.com/rust-lang/crates.io-index-archive";

/// An individual change to a crate in the crates.io index, returned by [the changes iterator](Index::changes).
pub struct Change {
    /// Name of a crate, can be used in [`Index::crate_`]
    crate_name: Box<str>,
    /// Timestamp in the crates.io index repository
    time: SystemTime,
    commit: Oid,
}

impl Change {
    /// Name of a crate, can be used in [`Index::crate_`]
    #[inline]
    #[must_use]
    pub fn crate_name(&self) -> &str {
        &*self.crate_name
    }

    /// Timestamp in the crates.io index repository, which may be publication or modification date
    #[inline]
    #[must_use]
    pub fn time(&self) -> SystemTime {
        self.time
    }

    /// git hash of a commit in the crates.io repository
    #[must_use]
    pub fn commit(&self) -> &[u8; 20] {
        self.commit.as_bytes().try_into().unwrap()
    }

    /// git hash of a commit in the crates.io repository
    #[must_use]
    pub fn commit_hex(&self) -> String {
        self.commit.to_string()
    }
}

/// See [`Index::changes`]
pub struct ChangesIter<'repo> {
    repo: &'repo git2::Repository,
    current: Commit<'repo>,
    current_tree: Tree<'repo>,
    out: VecDeque<Change>,
}

impl<'repo> Iterator for ChangesIter<'repo> {
    type Item = Result<Change, Error>;

    fn next(&mut self) -> Option<Self::Item> {
        while self.out.is_empty() {
            let parent = match self.get_parent() {
                Ok(Some(parent)) => parent,
                Ok(None) => return None,
                Err(e) => return Some(Err(e)),
            };
            let parent_tree = parent.tree().ok()?;
            let time = SystemTime::UNIX_EPOCH + Duration::from_secs(self.current.time().seconds().max(0) as _);
            Self::tree_additions(&self.repo, &mut self.out, time, &self.current.id(), &self.current_tree, &parent_tree).ok()?;
            self.current_tree = parent_tree;
            self.current = parent;
        }
        self.out.pop_front().map(Ok)
    }
}

impl<'repo> ChangesIter<'repo> {
    pub(crate) fn new(index: &'repo Index) -> Result<Self, git2::Error> {
        let current = index.repo.find_object(index.head, None)?.peel_to_commit()?;
        let current_tree = current.tree()?;

        Ok(Self {
            repo: &index.repo,
            current,
            current_tree,
            out: VecDeque::new(),
        })
    }

    fn get_parent(&self) -> Result<Option<Commit<'repo>>, Error> {
        match self.current.parents().next() {
            Some(ok) => Ok(Some(ok)),
            None => {
                let (oid, branch) = match oid_and_branch_from_commit_message(self.current.body().unwrap_or_default()) {
                    Some(res) => res,
                    None => return Ok(None),
                };
                match self.repo.find_commit(oid) {
                    Ok(ok) => Ok(Some(ok)),
                    Err(_) => {
                        let mut archive_origin = self.repo.remote_anonymous(INDEX_GIT_ARCHIVE_URL)?;
                        archive_origin.fetch(
                            &[format!("refs/heads/{}", branch)],
                            Some(&mut fetch_opts()),
                            None,
                        )?;
                        Ok(Some(self.repo.find_commit(oid)?))
                    },
                }
            }
        }
    }

    fn tree_additions(repo: &git2::Repository, out: &mut VecDeque<Change>, change_time: SystemTime, commit: &Oid, new: &Tree, old: &Tree) -> Result<(), git2::Error> {
        let old_oids = old.iter().map(|old| old.id()).collect::<HashSet<_>>();
        for new_entry in new.iter() {
            let new_id = new_entry.id();
            if !old_oids.contains(&new_id) {
                let new_obj = new_entry.to_object(repo)?;
                if let Some(new_tree) = new_obj.as_tree() {
                    let name_bytes = new_entry.name_bytes();
                    // Recurse only into crate subdirs, and they all happen to be 1 or 2 letters long
                    let old_obj = if name_bytes.len() <= 2 && name_bytes.iter().copied().all(valid_crate_name_char)
                        { old.get_name_bytes(name_bytes) } else { None }
                        .map(|o| o.to_object(repo))
                        .transpose()?;
                    let empty;
                    let old_tree = match old_obj.as_ref().and_then(|o| o.as_tree()) {
                        Some(t) => t,
                        None => { empty = Self::empty_tree(repo); &empty }
                    };
                    Self::tree_additions(repo, out, change_time, commit, new_tree, old_tree)?;
                } else {
                    if let Some(name) = new_entry.name() {
                        // filter out config.json
                        if name.bytes().all(valid_crate_name_char) {
                            out.push_back(Change {
                                time: change_time,
                                crate_name: name.into(),
                                commit: commit.clone(),
                            });
                        }
                    }
                }
            }
        }
        Ok(())
    }

    fn empty_tree(repo: &git2::Repository) -> Tree<'_> {
        let magic_empty = Oid::from_str("4b825dc642cb6eb9a060e54bf8d69288fbee4904").unwrap();
        repo.find_tree(magic_empty).expect("oops, git changed its implementation detail")
    }
}

#[inline]
fn valid_crate_name_char(c: u8) -> bool {
    c.is_ascii_alphanumeric() || c == b'-' || c == b'_'
}

fn oid_and_branch_from_commit_message(msg: &str) -> Option<(Oid, &str)> {
    let hash_start = msg.split_once("Previous HEAD was ")?.1.trim_start_matches(|c: char| !c.is_ascii_hexdigit());
    let (hash_str, rest) = hash_start.split_once(|c:char| !c.is_ascii_hexdigit())?;
    let hash = Oid::from_str(hash_str).ok()?;
    let snapshot_start = rest.find("snapshot-")?;
    let branch = rest.get(snapshot_start..snapshot_start+"snapshot-xxxx-xx-xx".len())?;

    Some((hash, branch))
}

#[test]
fn changes() {
    let index = Index::new_cargo_default().unwrap();
    let ch = ChangesIter::new(&index).unwrap();
    let mut last_time = SystemTime::now();
    for c in ch.take(20) {
        let c = c.unwrap();
        index.crate_(&c.crate_name).unwrap();
        assert!(last_time >= c.time);
        last_time = c.time;
    }
}

#[test]
fn changes_parse_split_message() {
    let (id, branch) = oid_and_branch_from_commit_message("Previous HEAD was 4181c62812c70fafb2b56cbbd66c31056671b445, now on the `snapshot-2021-07-02` branch

More information about this change can be found [online] and on [this issue].

[online]: https://internals.rust-lang.org/t/cargos-crate-index-upcoming-squash-into-one-commit/8440
[this issue]: https://github.com/rust-lang/crates-io-cargo-teams/issues/47").unwrap();
    assert_eq!("4181c62812c70fafb2b56cbbd66c31056671b445", id.to_string());
    assert_eq!("snapshot-2021-07-02", branch);
}
