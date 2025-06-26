use fxhash::{FxBuildHasher, FxHashMap};
use indexmap::IndexMap;
use crate::prelude::{Arc, RwLock};

pub type FxIndexMap<K, V> = IndexMap<K, V, FxBuildHasher>;
pub type SyncFxIndexMap<K, V> = Arc<RwLock<FxHashMap<K, Arc<V>>>>;
pub type SyncIndexMap<K, V> = Arc<RwLock<FxIndexMap<K, Arc<V>>>>;