use crate::bare_index::fetch_remote;
use crate::error::GixError;
use crate::Error;
use crate::Index;
use gix::bstr::ByteSlice;
use gix::prelude::TreeEntryRefExt;
use std::collections::{HashSet, VecDeque};
use std::convert::TryInto;
use std::time::{Duration, SystemTime};

const INDEX_GIT_ARCHIVE_URL: &str = "https://github.com/rust-lang/crates.io-index-archive";

/// An individual change to a crate in the crates.io index, returned by [the changes iterator](Index::changes).
#[derive(Debug, Clone)]
pub struct Change {
    /// Name of a crate, can be used in [`Index::crate_`]
    crate_name: Box<str>,
    /// Timestamp in the crates.io index repository
    time: SystemTime,
    commit: gix::ObjectId,
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
    repo: &'repo gix::Repository,
    current: gix::Commit<'repo>,
    current_tree: gix::Tree<'repo>,
    out: VecDeque<Change>,
}

impl<'repo> Iterator for ChangesIter<'repo> {
    type Item = Result<Change, Error>;

    fn next(&mut self) -> Option<Self::Item> {
        while self.out.is_empty() {
            let parent = match self.get_parent() {
                Ok(Some(parent)) => parent,
                Ok(None) => return None,
                Err(e) => return Some(Err(e.into())),
            };
            let parent_tree = parent.tree().ok()?;
            let time = SystemTime::UNIX_EPOCH + Duration::from_secs(self.current.time().ok()?.seconds.max(0) as _);
            Self::tree_additions(&self.repo, &mut self.out, time, &self.current.id(), &self.current_tree, &parent_tree).ok()?;
            self.current_tree = parent_tree;
            self.current = parent;
        }
        self.out.pop_front().map(Ok)
    }
}

impl<'repo> ChangesIter<'repo> {
    pub(crate) fn new(index: &'repo Index) -> Result<Self, GixError> {
        let current = index.repo.find_object(index.head_commit)?.peel_to_kind(gix::object::Kind::Commit)?.into_commit();
        let current_tree = current.tree()?;

        Ok(Self {
            repo: &index.repo,
            current,
            current_tree,
            out: VecDeque::new(),
        })
    }

    fn get_parent(&self) -> Result<Option<gix::Commit<'repo>>, GixError> {
        match self.current.parent_ids().next().map(|id| id.try_object()).transpose()?.flatten()
        {
            Some(obj) => Ok(Some(obj.try_into_commit()?)),
            None => {
                let msg = self.current.message_raw_sloppy().to_str_lossy();
                let (oid, branch) = match oid_and_branch_from_commit_message(msg.as_ref()) {
                    Some(res) => res,
                    None => return Ok(None),
                };
                match self.repo.try_find_object(oid)? {
                    Some(obj) => Ok(Some(obj.try_into_commit()?)),
                    None => {
                        let mut remote = self.repo.remote_at(INDEX_GIT_ARCHIVE_URL)?;
                        fetch_remote(&mut remote, &[&format!("+refs/heads/{}", branch)])?;
                        Ok(Some(self.repo.find_object(oid)?.try_into_commit()?))
                    }
                }
            }
        }
    }

    fn tree_additions(
        repo: &gix::Repository,
        out: &mut VecDeque<Change>,
        change_time: SystemTime,
        commit: &gix::hash::oid,
        new: &gix::Tree<'_>,
        old: &gix::Tree<'_>,
    ) -> Result<(), GixError> {
        let old_oids = old
            .iter()
            .map(|old| old.map(|e| e.object_id()))
            .collect::<Result<HashSet<_>, _>>()?;
        let old = old.decode()?;
        for new_entry in new.iter().filter_map(Result::ok) {
            if old_oids.contains(new_entry.oid()) {
                continue;
            }
            if new_entry.mode().is_tree() {
                let new_tree = new_entry.object()?.into_tree();
                let name = new_entry.filename();
                // Recurse only into crate subdirs, and they all happen to be 1 or 2 letters long
                let is_crates_subdir = name.len() <= 2 && name.iter().copied().all(valid_crate_name_char);
                let old_obj = if is_crates_subdir {
                    old.bisect_entry(name, true)
                        .map(|entry| entry.attach(repo))
                } else {
                    None
                }
                    .map(|o| o.object())
                    .transpose()?;
                let old_tree = match old_obj.and_then(|o| o.try_into_tree().ok()) {
                    Some(t) => t,
                    None => repo.empty_tree(),
                };
                Self::tree_additions(repo, out, change_time, commit, &new_tree, &old_tree)?;
            } else {
                let name = new_entry.filename();
                // filter out config.json
                if name.iter().copied().all(valid_crate_name_char) {
                    out.push_back(Change {
                        time: change_time,
                        crate_name: name.to_string().into(),
                        commit: commit.into(),
                    });
                }
            }
        }
        Ok(())
    }
}

#[inline]
fn valid_crate_name_char(c: u8) -> bool {
    c.is_ascii_alphanumeric() || c == b'-' || c == b'_'
}

fn oid_and_branch_from_commit_message(msg: &str) -> Option<(gix::ObjectId, &str)> {
    let hash_start = msg.split_once("Previous HEAD was ")?.1.trim_start_matches(|c: char| !c.is_ascii_hexdigit());
    let (hash_str, rest) = hash_start.split_once(|c: char| !c.is_ascii_hexdigit())?;
    let hash = gix::ObjectId::from_hex(hash_str.as_bytes()).ok()?;
    let snapshot_start = rest.find("snapshot-")?;
    let branch = rest.get(snapshot_start..snapshot_start + "snapshot-xxxx-xx-xx".len())?;

    Some((hash, branch))
}

#[cfg(test)]
pub(crate) mod test {
    use crate::changes::{oid_and_branch_from_commit_message, };

    #[test]
    fn changes_parse_split_message() {
        let (id, branch) = oid_and_branch_from_commit_message(
            "Previous HEAD was 4181c62812c70fafb2b56cbbd66c31056671b445, now on the `snapshot-2021-07-02` branch

More information about this change can be found [online] and on [this issue].

[online]: https://internals.rust-lang.org/t/cargos-crate-index-upcoming-squash-into-one-commit/8440
[this issue]: https://github.com/rust-lang/crates-io-cargo-teams/issues/47",
        )
            .unwrap();
        assert_eq!("4181c62812c70fafb2b56cbbd66c31056671b445", id.to_string());
        assert_eq!("snapshot-2021-07-02", branch);
    }
}
