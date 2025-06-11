use crate::util::jsons::read_json;
use fxhash::FxHashMap;
use once_cell::sync::Lazy;
use std::sync::{Arc, RwLock};

pub(super) static LINKS: Lazy<Arc<RwLock<FxHashMap<String, String>>>> = Lazy::new(||
    Arc::new(RwLock::new(read_json("./links.json")))
);

pub fn get_link(layout_name: &str) -> String {
    let links = LINKS.read().unwrap();
    let exteral_link = links.get(layout_name);
    match exteral_link {
        Some(link) => format!("<{}>", link),
        None => String::new()
    }
}