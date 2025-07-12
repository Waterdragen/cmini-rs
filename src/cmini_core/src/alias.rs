use std::sync::Arc;
use crate::lock::RwLock;
use fxhash::FxBuildHasher;
use indexmap::IndexMap;

pub type FxIndexMap<K, V> = IndexMap<K, V, FxBuildHasher>;
pub type SyncIndexMap<K, V> = Arc<RwLock<FxIndexMap<K, Arc<V>>>>;