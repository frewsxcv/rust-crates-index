use crate::{Crate, Error};
use crate::dirs::{url_to_local_dir, RegistryKind};
use crate::{sparse_index, bare_index};

/// A crates index (either a bare git checkout or a sparse HTTP cache)
pub enum Index {
    /// A sparse HTTP index
    Sparse(sparse_index::Index),
    /// A bare Git index
    Bare(bare_index::Index),
}

impl Index {
    /// Creates a view of the provided index URL, auto-detecting the underlying type of the
    /// registry.
    pub fn from_url(url: &str) -> Result<Self, Error> {
        let (dir, url, kind) = url_to_local_dir(url)?;
        let mut path = home::cargo_home()?;
        path.push("registry");
        path.push("index");
        path.push(dir);
        Ok(match kind {
            RegistryKind::Git => Self::Bare(bare_index::Index::from_path_and_url(path, url)?),
            RegistryKind::SparseHttp => Self::Sparse(sparse_index::Index::from_path_and_url(path, url)),
        })
    }

    /// Read a single crate definition from the index, this does not update the index first so
    /// results may be stale.  (Use `Index::update` if you want to force a global update).
    pub fn crate_(&self, name: &str) -> Option<Crate> {
        match self {
            Self::Sparse(s) => s.crate_from_cache(name),
            Self::Bare(b) => b.crate_(name),
        }
    }

    /// Force a refresh of the crates index from the upstream repository.  Note that this may
    /// invalidate cache entries as their commit IDs will no longer match.
    ///
    /// # TODO
    ///
    /// This is a no-op on sparse registries.
    pub fn update(&mut self) -> Result<(), Error> {
        match self {
            Self::Sparse(_s) => Ok(()),
            Self::Bare(b) => b.update(),
        }
    }
}
