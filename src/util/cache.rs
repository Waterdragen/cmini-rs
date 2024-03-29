use std::collections::HashMap;
use std::path::Path;
use crate::util::jsons::{
    get_map_str_map_str_f64,
    get_layout,
    write_map_str_map_str_f64,
};
use crate::util::core::Layout;
use crate::util::corpora::load_corpus;

pub type CachedStat = HashMap<String, HashMap<String, f64>>;

fn cache_get(name: &str) -> Option<CachedStat> {
    let name = name.to_lowercase();
    let path = format!("./cache/{}.json", name);
    if !Path::new(&path).exists() {
        return None;
    }
    Some(get_map_str_map_str_f64(&path))
}

fn layout_get(name: &str) -> Layout {
    get_layout(&format!("./layouts/{}.json", name))
}

fn cache_fill(ll: Layout, data: Option<CachedStat>, corpus: &str) -> CachedStat {
    let trigrams = load_corpus(&format!("./corpora/{}/trigrams.json", corpus));
    todo!();
}

fn update(name: &str, data: CachedStat) -> CachedStat {
    write_map_str_map_str_f64(&format!("./cache/{}.json", name), &data);
    data
}

fn get(name: &str, corpus: &str) -> Option<CachedStat> {
    todo!()
}