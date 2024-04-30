use std::path::Path;
use std::sync::{Arc, RwLock};

use lazy_static::lazy_static;
use strsim::jaro_winkler;

use crate::util::jsons::get_raw_layouts;
use crate::util::core::{LayoutConfig, ServerLayouts};

lazy_static!(
    pub static ref LAYOUTS: ServerLayouts = get_raw_layouts("./layouts.json");
);

pub fn add(ll: LayoutConfig) -> bool {
    if has_layout(&ll.name) {
        return false;
    }
    add_layout(ll);
    true
}

pub fn get<'a>(name: &str) -> Option<LayoutConfig> {
    let path = format!("./layouts/{}.json", name);
    if !Path::new(&path).exists() {
        return None;
    }
    Some(get_layout(&path))
}

pub fn find(name: &str) -> LayoutConfig {
    let layout_files = std::fs::read_dir("./layouts").unwrap();
    let mut max_score = 0.0;
    let mut closest = String::new();

    for file in layout_files {
        if let Ok(entry) = file {
            if let Some(file_name) = entry.file_name().to_str() {
                let base_name = file_name.trim_end_matches(".json");
                let score = jaro_winkler(name, base_name);

                if score > max_score {
                    max_score = score;
                    closest = base_name.to_string();
                }
            }
        }
    }
    get_layout(&format!("./layouts/{}.json", closest))
}

pub fn remove(name: &str, id: u64) -> bool {
    remove_layout(name, id, false)
}

pub fn remove_as_admin(name: &str, id: u64) -> bool {
    remove_layout(name, id, true)
}

fn add_layout(ll: LayoutConfig) {
    let mut layouts_mut = LAYOUTS.write().unwrap();
    let name = ll.name.clone();
    layouts_mut.insert(name, ll);
}

fn get_layout(name: &str) -> LayoutConfig {
    let layouts = LAYOUTS.read().unwrap();
    let layout = layouts.get(name).unwrap();
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


