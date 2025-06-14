use crate::util::authors::AUTHORS;
use crate::util::conv;
use crate::util::core::{FxIndexMap, JsonLayoutConfig, LayoutConfig};
use crate::util::corpora::CORPORA_PREFS;
use crate::util::jsons::{read_json, write_json};
use crate::util::links::LINKS;
use fxhash::{FxBuildHasher, FxHashMap};
use once_cell::sync::Lazy;
use serde::de::{MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use std::borrow::Cow;
use std::fmt::{Debug, Formatter};
use std::ops::Deref;
use std::sync::{Arc, RwLock};
use strsim::jaro_winkler;
use thiserror::Error;
use crate::util::get::{Get, GetMut};

pub static LAYOUTS: Lazy<ServerLayouts> = Lazy::new(|| read_json("./layouts.json"));
pub static LIKES: Lazy<Arc<RwLock<FxHashMap<String, Vec<u64>>>>> = Lazy::new(|| read_json("./likes.json"));

#[derive(Debug, Error)]
pub enum RemoveError<'a> {
    #[error("Error: `{0}` does not exist")]
    NotFound(&'a str),
    #[error("Error: you don't own `{0}`")]
    NotOwner(&'a str),
}

pub fn get_like_count(name: &str) -> usize {
    let likes = LIKES.read().unwrap();
    match likes.get(name) {
        Some(liked_users) => liked_users.len(),
        None => 0,
    }
}

pub fn sync_data() {
    write_json("./authors.json", &*AUTHORS);
    write_json("./corpora.json", &*CORPORA_PREFS);
    write_json("./layouts.json", &*LAYOUTS);
    write_json("./likes.json", &*LIKES);
    write_json("./links.json", &*LINKS);
}

#[repr(transparent)]
#[derive(Serialize)]
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
        let mut layouts_mut = self.write().unwrap();
        layouts_mut.insert(ll.name.clone(), ll);
        true
    }
    pub fn get<'a>(&'a self, name: &'a str) -> Get<'a, LayoutConfig> {
        Get(self.read().unwrap(), Cow::Borrowed(name))
    }
    pub fn get_mut<'a>(&'a self, name: &'a str) -> GetMut<'a, LayoutConfig> {
        GetMut(self.write().unwrap(), Cow::Borrowed(name))
    }
    pub fn find(&self, name: &str) -> Get<LayoutConfig> {
        let closest = self.best_match(name);
        Get(self.read().unwrap(), Cow::Owned(closest))
    }
    pub fn contains(&self, name: &str) -> bool {
        let layouts = self.read().unwrap();
        layouts.contains_key(name)
    }
    pub fn remove<'a>(&self, name: &'a str, id: u64) -> Result<LayoutConfig, RemoveError<'a>> {
        self.remove_impl(name, id, false)
    }
    pub fn remove_as_admin<'a>(&self, name: &'a str, id: u64) -> Result<LayoutConfig, RemoveError<'a>> {
        self.remove_impl(name, id, true)
    }
    fn remove_impl<'a>(&self, name: &'a str, id: u64, admin: bool) -> Result<LayoutConfig, RemoveError<'a>> {
        let user = {
            // Must drop or else deadlock
            let ll = self.get(name);
            match ll.checked() {
                None => return Err(RemoveError::NotFound(name)),
                Some(_) => ll.user,
            }
        };
        if user == id || admin {
            let mut layouts_mut = self.write().unwrap();
            // Removal always succeed
            Ok(layouts_mut.shift_remove(name).unwrap())
        } else {
            Err(RemoveError::NotOwner(name))
        }
    }
    pub fn best_match(&self, base_name: &str) -> String {
        let layouts = self.read().unwrap();
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

}

impl<'de> Deserialize<'de> for ServerLayouts {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>
    {
        deserializer.deserialize_map(ServerLayoutsVisitor)
    }
}

struct ServerLayoutsVisitor;

impl<'de> Visitor<'de> for ServerLayoutsVisitor {
    type Value = ServerLayouts;

    fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
        write!(formatter, "struct ServerLayouts")
    }

    fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
    where
        A: MapAccess<'de>,
    {
        let mut map_inner = FxIndexMap::with_capacity_and_hasher(
            map.size_hint().unwrap_or(0),
            FxBuildHasher::default()
        );
        while let Some((key, ll)) = map.next_entry::<String, JsonLayoutConfig>()? {
            let name = key.clone();
            let layout_config = LayoutConfig {
                name: key,
                user: ll.user,
                board: ll.board.clone(),
                keys: conv::layout::unpack(&ll.keys),
                sum: conv::hash_keys(&ll.keys),
            };
            map_inner.insert(name, layout_config);
        }
        Ok(ServerLayouts(Arc::new(RwLock::new(map_inner))))
    }
}