pub use serde_json::Error as SerdeJsonError;
use std::{io};
use std::path::PathBuf;
pub use toml::de::Error as TomlDeError;

/// The catch-all error for the entire crate.
#[derive(Debug, thiserror::Error)]
#[allow(missing_docs)]
pub enum Error {
    #[error("\"gix\" crate failed. If problems persist, consider deleting `~/.cargo/registry/index/github.com-1ecc6299db9ec823/`")]
    #[cfg(feature = "git-index")]
    Git(#[from] GixError),
    #[error("{0}")]
    Url(String),
    #[error("Could not obtain the most recent head commit in repo at {}. Tried {}, had {} available", repo_path.display(), refs_tried.join(", "), refs_available.join(", "))] 
    MissingHead {
        /// The references we tried to get commits for.
        refs_tried: &'static [&'static str],
        /// The references that were actually present in the repository.
        refs_available: Vec<String>,
        /// The path of the repository we tried
        repo_path: PathBuf,
    },
    #[error(transparent)]
    Io(#[from] io::Error),
    #[error("If this happens, the registry is seriously corrupted. Consider deleting `~/.cargo/registry/index/`")]
    Json(#[from] SerdeJsonError),
    #[error(transparent)]
    Toml(#[from] TomlDeError),
}

/// Any error produced by `gix` or the `gix-*` family of crates.
#[derive(Debug, thiserror::Error)]
#[allow(missing_docs)]
#[cfg(feature = "git-index")]
pub enum GixError {
    #[error(transparent)]
    CreateInMemoryRemote(#[from] gix::remote::init::Error),
    #[error(transparent)]
    HeadCommit(#[from] gix::reference::head_commit::Error),
    #[error(transparent)]
    TreeOfCommit(#[from] gix::object::commit::Error),
    #[error(transparent)]
    DecodeObject(#[from] gix::objs::decode::Error),
    #[error(transparent)]
    FindExistingObject(#[from] gix::object::find::existing::Error),
    #[error(transparent)]
    FindObject(#[from] gix::object::find::Error),
    #[error(transparent)]
    IntoObjectKind(#[from] gix::object::try_into::Error),
    #[error("The '{}' file is missing at the root of the tree of the crates index", path.display())]
    PathMissing {
        path: std::path::PathBuf
    },
    #[error(transparent)]
    LockAcquire(#[from] gix::lock::acquire::Error),
    #[error(transparent)]
    ParseRefSpec(#[from] gix::refspec::parse::Error),
    #[error(transparent)]
    RemoteConnect(#[from] gix::remote::connect::Error),
    #[error(transparent)]
    PrepareFetch(#[from] gix::remote::fetch::prepare::Error),
    #[error(transparent)]
    Fetch(#[from] gix::remote::fetch::Error),
    #[error(transparent)]
    PrepareClone(#[from] gix::clone::Error),
    #[error(transparent)]
    RemoteName(#[from] gix::remote::name::Error),
    #[error(transparent)]
    FetchDuringClone(#[from] gix::clone::fetch::Error),
    #[error(transparent)]
    PeelToKind(#[from] gix::object::peel::to_kind::Error),
}

/// Unknown error from [`crate::GitIndex::crates_parallel`]
#[cfg(feature = "parallel")]
#[derive(Debug, thiserror::Error)]
#[error("error while iterating git repository")]
pub struct CratesIterError;
