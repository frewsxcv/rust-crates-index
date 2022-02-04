use crate::Dependency;
use rustc_hash::FxHashSet;
use std::collections::HashMap;
use std::hash::Hash;
use std::hash::Hasher;
use std::sync::Arc;

/// Many crates (their versions) have the same features and dependencies
pub(crate) struct DedupeContext {
    features: FxHashSet<HashableHashMap<String, Vec<String>>>,
    deps: FxHashSet<Arc<[Dependency]>>,
}

impl DedupeContext {
    #[inline]
    pub(crate) fn new() -> Self {
        Self {
           deps: FxHashSet::default(),
           features: FxHashSet::default(),
        }
    }

    pub(crate) fn features(&mut self, features: &mut Arc<HashMap<String, Vec<String>>>) {
        let features_to_dedupe = HashableHashMap::new(Arc::clone(&features));
        if let Some(has_feats) = self.features.get(&features_to_dedupe) {
            *features = Arc::clone(&has_feats.map);
        } else {
            if self.features.len() > 16384 { // keeps peek memory low (must clear, remove is leaving tombstones)
                self.features.clear();
            }
            self.features.insert(features_to_dedupe);
        }
    }

    pub(crate) fn deps(&mut self, deps: &mut Arc<[Dependency]>) {
        if let Some(has_deps) = self.deps.get(&*deps) {
            *deps = Arc::clone(has_deps);
        } else {
            if self.deps.len() > 16384 { // keeps peek memory low (must clear, remove is leaving tombstones)
                self.deps.clear();
            }
            self.deps.insert(Arc::clone(&deps));
        }
    }
}

/// Newtype that caches hash of the hashmap (the default hashmap has a random order of the keys, so it's not cheap to hash)
#[derive(PartialEq, Eq)]
pub struct HashableHashMap<K: PartialEq + Hash + Eq, V: PartialEq + Hash + Eq> {
    pub map: Arc<HashMap<K, V>>,
    hash: u64,
}

impl<K: PartialEq + Hash + Eq, V: PartialEq + Hash + Eq> Hash for HashableHashMap<K, V> {
    fn hash<H>(&self, hasher: &mut H) where H: Hasher {
        hasher.write_u64(self.hash);
    }
}

impl<K: PartialEq + Hash + Eq, V: PartialEq + Hash + Eq> HashableHashMap<K, V> {
    pub(crate) fn new(map: Arc<HashMap<K, V>>) -> Self {
        let mut hash = 0;
        for (k, v) in map.iter() {
            let mut hasher = rustc_hash::FxHasher::default();
            k.hash(&mut hasher);
            v.hash(&mut hasher);
            hash ^= hasher.finish(); // XOR makes it order-independent
        }
        Self {
            hash, map
        }
    }
}
