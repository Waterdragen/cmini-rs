use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use crate::util::jsons::get_map_str_str;
use lazy_static::lazy_static;

lazy_static!(
    static ref __LINKS: Arc<RwLock<HashMap<String, String>>>
        = Arc::new(RwLock::new(get_map_str_str("./links.json")));
);

pub fn get_link(layout_name: &str) -> String {
    let links = __LINKS.read().unwrap();
    let exteral_link = links.get(layout_name);
    match exteral_link {
        Some(link) => format!("<{}>", link),
        None => String::new()
    }
}