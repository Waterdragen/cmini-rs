use std::sync::Arc;
use fxhash::FxHashMap;

use once_cell::sync::Lazy;
use strsim::jaro_winkler;

use crate::util::jsons::{get_server_layouts, write_layouts, get_map_str_vec_u64};
use crate::util::core::{LayoutConfig, ServerLayouts};

pub static LAYOUTS: Lazy<ServerLayouts> = Lazy::new(|| get_server_layouts("./layouts.json"));
pub static LIKES: Lazy<FxHashMap<String, Vec<u64>>> = Lazy::new(|| get_map_str_vec_u64("./likes.json"));

pub fn add(ll: LayoutConfig) -> bool {
    if has_layout(&ll.name) {
        return false;
    }
    add_layout(ll);
    true
}

pub fn get(name: &str) -> Option<LayoutConfig> {
    match has_layout(name) {
        true => Some(get_layout(name)),
        false => None,
    }
}

pub fn find(name: &str) -> LayoutConfig {
    let closest = best_match(name);
    get_layout(&closest)
}

pub fn remove(name: &str, id: u64) -> bool {
    remove_layout(name, id, false)
}

pub fn remove_as_admin(name: &str, id: u64) -> bool {
    remove_layout(name, id, true)
}

pub fn get_like_count(name: &str) -> usize {
    match LIKES.get(name) {
        Some(liked_users) => liked_users.len(),
        None => 0,
    }
}

pub fn sync_layouts() {
    write_layouts("./layouts.json", &LAYOUTS);
}

fn add_layout(ll: LayoutConfig) {
    let mut layouts_mut = LAYOUTS.write().unwrap();
    let name = ll.name.clone();
    layouts_mut.insert(name, ll);
}

fn get_layout(name: &str) -> LayoutConfig {
    let layouts = LAYOUTS.read().unwrap();
    let layout = layouts.get(name).unwrap_or_else(|| panic!("Cannot get {name}"));
    Arc::clone(layout)
}

fn has_layout(name: &str) -> bool {
    let layouts = LAYOUTS.read().unwrap();
    layouts.contains_key(name)
}

fn remove_layout(name: &str, id: u64, admin: bool) -> bool {
    if !has_layout(name) {
        return false;
    }
    let ll = get_layout(name);
    let check = ll.user == id || admin;
    if check {
        let mut layouts_mut = LAYOUTS.write().unwrap();
        layouts_mut.swap_remove(name);
    }
    check
}

fn best_match(base_name: &str) -> String {
    let layouts = LAYOUTS.read().unwrap();
    let mut max_score = 0.0;
    let mut closest = String::new();

    for name in layouts.keys() {
        let score = jaro_winkler(name, base_name);

        if score > max_score {
            max_score = score;
            closest = base_name.to_string();
        }
    }
    closest
}


