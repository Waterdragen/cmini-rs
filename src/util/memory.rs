use fxhash::FxHashMap;
use std::sync::{Arc, RwLock};

use once_cell::sync::Lazy;
use strsim::jaro_winkler;
use thiserror::Error;
use crate::util::authors::AUTHORS;
use crate::util::core::{LayoutConfig, ServerLayouts};
use crate::util::corpora::CORPORA_PREFS;
use crate::util::jsons::{read_json, write_json};
use crate::util::links::LINKS;

pub static LAYOUTS: Lazy<ServerLayouts> = Lazy::new(|| read_json("./layouts.json"));
pub static LIKES: Lazy<Arc<RwLock<FxHashMap<String, Vec<u64>>>>> = Lazy::new(|| read_json("./likes.json"));

#[derive(Debug, Error)]
pub enum RemoveError<'a> {
    #[error("Error: `{0}` does not exist")]
    NotFound(&'a str),
    #[error("Error: you don't own `{0}`")]
    NotOwner(&'a str),
}

pub fn add(ll: Arc<LayoutConfig>) -> bool {
    if has_layout(&ll.name) {
        return false;
    }
    add_layout(ll);
    true
}

pub fn get(name: &str) -> Option<Arc<LayoutConfig>> {
    match has_layout(name) {
        true => Some(get_layout(name)),
        false => None,
    }
}

pub fn find(name: &str) -> Arc<LayoutConfig> {
    let closest = best_match(name);
    get_layout(&closest)
}

pub fn remove(name: &str, id: u64) -> Result<(), RemoveError> {
    remove_layout(name, id, false)
}

pub fn remove_as_admin(name: &str, id: u64) -> Result<(), RemoveError> {
    remove_layout(name, id, true)
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

fn add_layout(ll: Arc<LayoutConfig>) {
    let mut layouts_mut = LAYOUTS.write().unwrap();
    let name = ll.name.clone();
    layouts_mut.insert(name, ll);
}

fn get_layout(name: &str) -> Arc<LayoutConfig> {
    let layouts = LAYOUTS.read().unwrap();
    let layout = layouts.get(name).unwrap_or_else(|| panic!("Cannot get {name}"));
    Arc::clone(layout)
}

fn has_layout(name: &str) -> bool {
    let layouts = LAYOUTS.read().unwrap();
    layouts.contains_key(name)
}

fn remove_layout(name: &str, id: u64, admin: bool) -> Result<(), RemoveError> {
    if !has_layout(name) {
        return Err(RemoveError::NotFound(name));
    }
    let ll = get_layout(name);
    if ll.user == id || admin {
        let mut layouts_mut = LAYOUTS.write().unwrap();
        layouts_mut.shift_remove(name);
        Ok(())
    } else {
        Err(RemoveError::NotOwner(&name))
    }
}

fn best_match(base_name: &str) -> String {
    let layouts = LAYOUTS.read().unwrap();
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


