use crate::core::get::{Get, GetMut};
use crate::core::{conv, FxIndexMap, Key, Position};
use crate::prelude::RwLock;
use crate::util::memory::RemoveError;
use fxhash::FxHashMap;
use serde::ser::SerializeMap;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::borrow::Cow;
use std::ops::Deref;
use std::sync::Arc;
use strsim::jaro_winkler;

pub type Layout = FxHashMap<Key, Position>;

#[derive(Serialize)]
#[serde(transparent)]
pub struct ServerLayouts(Arc<RwLock<FxIndexMap<String, LayoutConfig>>>);

impl Deref for ServerLayouts {
    type Target = Arc<RwLock<FxIndexMap<String, LayoutConfig>>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl ServerLayouts {
    pub fn add(&self, ll: LayoutConfig) -> bool {
        if self.contains(&ll.name) {
            return false;
        }
        let mut layouts_mut = self.write();
        layouts_mut.insert(ll.name.clone(), ll);
        true
    }
    /// # Panic on deref
    /// - the layout name does not exist
    #[track_caller]
    pub fn raw_get<'a>(&'a self, name: &'a str) -> Get<'a, LayoutConfig> {
        Get(self.read(), Cow::Borrowed(name))
    }
    /// # Panic on deref
    /// - the layout name does not exist
    #[track_caller]
    pub fn raw_get_mut<'a>(&'a self, name: &'a str) -> GetMut<'a, LayoutConfig> {
        GetMut(self.write(), Cow::Borrowed(name))
    }
    /// # Panic on deref
    /// - the layout name is empty
    #[track_caller]
    pub fn find(&self, name: &str) -> Get<LayoutConfig> {
        let closest = self.best_match(name);
        Get(self.read(), Cow::Owned(closest))
    }
    /// # Panic on deref
    /// - the layout name is empty
    #[track_caller]
    pub fn find_mut(&self, name: &str) -> GetMut<LayoutConfig> {
        let closest = self.best_match(name);
        GetMut(self.write(), Cow::Owned(closest))
    }
    pub fn contains(&self, name: &str) -> bool {
        let layouts = self.read();
        layouts.contains_key(name)
    }
    pub fn remove<'a>(&self, name: &'a str, id: u64) -> Result<LayoutConfig, RemoveError<'a>> {
        self.remove_impl(name, id, None)
    }
    pub fn remove_as_admin<'a>(&self, name: &'a str, id: u64, in_public_channel: bool) -> Result<LayoutConfig, RemoveError<'a>> {
        self.remove_impl(name, id, Some(in_public_channel))
    }
    #[track_caller]
    fn remove_impl<'a>(&self, name: &'a str, id: u64, admin: Option<bool>) -> Result<LayoutConfig, RemoveError<'a>> {
        let user = {
            // Must drop or else deadlock
            match self.raw_get(name).checked() {
                None => return Err(RemoveError::NotFound(name)),
                Some(ll) => ll.user,
            }
        };
        match (user == id, admin) {
            (true, _) | (false, Some(true)) => {
                let mut layouts_mut = self.write();
                // Removal always succeed
                Ok(layouts_mut.shift_remove(name).unwrap())
            }
            (false, None) => Err(RemoveError::NotOwner(name)),
            (false, Some(false)) => Err(RemoveError::SudoInPrivateChannel),
        }
    }
    fn best_match(&self, base_name: &str) -> String {
        let layouts = self.read();
        let mut max_score = 0.0;
        let mut closest = String::new();

        for name in layouts.keys() {
            let score = jaro_winkler(name, base_name);

            if score > max_score {
                max_score = score;
                closest = name.to_string();
            }
        }
        closest
    }
    pub fn arc_clone(&self) -> Self {
        Self(Arc::clone(&self.0))
    }

}

impl<'de> Deserialize<'de> for ServerLayouts {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>
    {
        let map = FxHashMap::<String, JsonLayoutConfig>::deserialize(deserializer)?;
        let layouts_inner = map.into_iter()
            .map(|(name, JsonLayoutConfig { user, board, keys })| {
                let sum = conv::hash_keys(&keys);
                let keys = conv::layout::unpack(&keys);
                (name.clone(), LayoutConfig {
                    name,
                    user,
                    board,
                    keys,
                    sum,
                })
            })
            .collect::<FxIndexMap<_, _>>();

        Ok(Self(Arc::new(RwLock::new(layouts_inner))))
    }
}

#[derive(Deserialize)]
pub struct JsonLayoutConfig {
    pub user: u64,
    pub board: String,
    pub keys: String,
}

#[derive(Clone)]
pub struct LayoutConfig {
    pub name: String,  // This field is impossible to impl Deserialize here
    pub user: u64,
    pub board: String,
    pub keys: Layout,
    pub sum: u64,
}

impl LayoutConfig {
    pub fn new(name: String, user: u64, board: String, keys: Layout) -> Self {
        let packed = conv::layout::pack(&keys);
        let sum = conv::hash_keys(&packed);
        LayoutConfig {
            name,
            user,
            board,
            keys,
            sum,
        }
    }
    // FIXME: this method should be impl for Layout, but for now Layout is a type alias for FxHashMap
    // FIXME: it might change to FxIndexMap in the future
    pub fn sorted_layout(&self) -> Vec<(Key, Position)> {
        let mut sorted = self.keys.iter()
            .map(|(key, pos)| (*key, *pos))
            .collect::<Vec<_>>();
        sorted.sort_unstable_by_key(|(_, pos)| (pos.row, pos.col));
        sorted
    }
}

impl Serialize for LayoutConfig {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer {
        let mut map = serializer.serialize_map(Some(3))?;
        map.serialize_entry("user", &self.user)?;
        map.serialize_entry("board", &self.board)?;
        map.serialize_entry("keys", &conv::layout::pack(&self.keys))?;
        map.end()
    }
}