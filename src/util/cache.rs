use std::collections::HashMap;
use std::path::Path;
use std::ptr::write;
use rayon::prelude::*;
use std::time::Instant;
use crate::util::jsons::{
    get_map_str_map_str_f64,
    get_layout,
    write_map_str_map_str_f64,
};
use crate::util::core::{Layout, Corpus};
use crate::util::{analyzer, corpora, memory};

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

fn cache_fill<'a>(ll: &Layout, data: Option<&'a mut CachedStat>, corpus: &str) -> &'a mut CachedStat {
    let path = format!("./corpora/{}/trigrams.json", corpus);
    let trigrams: Corpus = corpora::load_corpus(&path);
    let stats = analyzer::trigrams(ll, &trigrams);

    match data {
        Some(data) => {
            data.insert(corpus.to_string(), stats);
            data
        },
        None => {
            let mut update: CachedStat = HashMap::new();
            update.insert(corpus.to_string(), stats);
            Box::leak(Box::new(update))
        },
    }
}

fn update<'a>(name: &str, data: &'a mut CachedStat) -> &'a mut CachedStat {
    write_map_str_map_str_f64(&format!("./cache/{}.json", name), &data);
    data
}

pub fn get<'a>(name: &str, corpus: &str) -> Option<HashMap<String, f64>> {
    if name == "" || corpus == "" {
        return None;
    }
    let name = name.to_lowercase();
    let corpus = corpus.to_lowercase();

    let data = cache_get(&name);
    let mut data = data.unwrap_or_else(|| HashMap::new());

    if data.contains_key(&corpus) {
        return data.remove(&corpus);
    }
    let data = update(&name, cache_fill(&memory::find(&name), Some(Box::leak(Box::new(data))), &corpus));
    data.remove(&corpus)
}

fn cache_files() {
    let files = std::fs::read_dir("./layouts/").unwrap();
    let names: Vec<String> = files.filter_map(|file| {
        file.ok().and_then(|file| {
            let path = file.path();
            let file_name = path.file_name()?.to_str()?;
            if !file_name.ends_with(".json") {
                return None;
            }
            let basename = Path::new(file_name).file_stem()?.to_str()?;
            Some(basename.to_string())
        })
    }).collect();
    let corpus_files = std::fs::read_dir("./corpora/").unwrap();
    let corpus_names: Vec<String> = corpus_files.filter_map(|file| {
        file.ok().and_then(|file| {
            Some(file.path().file_name()?.to_str()?.to_string())
        })
    }).collect();
    names[..3].iter().for_each(|name| {
        let get_stuff_start = Instant::now();
        let ll = get_layout(&format!("./layouts/{}.json", name));
        let mut data = cache_get(&name);
        let mut data: Option<&mut CachedStat> = data.as_mut();
        println!("get stuff took: {:?}", get_stuff_start.elapsed());

        let cache_fill_start = Instant::now();
        for corpus in corpus_names.iter() {
            println!("Layout: {name}, Corpus: {corpus}");
            let cache_fill_start = Instant::now();
            data = Some(cache_fill(&ll, data, corpus));
        }
        println!("cache fill took: {:?}", cache_fill_start.elapsed());
        let update_start = Instant::now();
        update(&name, data.unwrap());
        println!("update took: {:?}", update_start.elapsed());
    });
}

pub fn cache_main() {
    let start = Instant::now();
    cache_files();
    let duration = start.elapsed();
    println!("Time elapsed: {:?}", duration);
}
