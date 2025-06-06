use fxhash::FxHashMap;
use std::sync::{Arc, RwLock};
use once_cell::sync::Lazy;
use crate::util::jsons::get_map_str_str;

static LINKS: Lazy<Arc<RwLock<FxHashMap<String, String>>>> = Lazy::new(||
    Arc::new(RwLock::new(get_map_str_str("./links.json")))
);

pub fn get_link(layout_name: &str) -> String {
    let links = LINKS.read().unwrap();
    let exteral_link = links.get(layout_name);
    match exteral_link {
        Some(link) => format!("<{}>", link),
        None => String::new()
    }
}