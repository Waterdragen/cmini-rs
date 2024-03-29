use std::path::Path;
use strsim::jaro_winkler;

use crate::util::core::Layout;
use crate::util::jsons::{get_layout, write_layout};

pub fn add(ll: &Layout) -> bool {
    let path = format!("./layouts/{}.json", ll.name.to_lowercase());

    if Path::new(&path).exists() {
        return false;
    }
    write_layout(&path, ll);
    true
}

pub fn remove(name: &str, id: u64) -> bool {
    remove_layout(name, id, false)
}

pub fn remove_as_admin(name: &str, id: u64) -> bool {
    remove_layout(name, id, true)
}

fn remove_layout(name: &str, id: u64, admin: bool) -> bool {
    let path = format!("./layouts/{}.json", name);

    if !Path::new(&path).exists() {
        return false;
    }
    let ll = get_layout(&path);
    let check = ll.user == id || admin;
    if check {
        std::fs::remove_file(path).unwrap();
    }
    check
}

pub fn get(name: &str) -> Option<Layout> {
    let path = format!("./layouts/{}.json", name);
    if !Path::new(&path).exists() {
        return None;
    }
    Some(get_layout(&path))
}

pub fn find(name: &str) -> Layout {
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

