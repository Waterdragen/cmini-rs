use crate::util::core::{CachedStatConfig, CachedStats, LayoutConfig, RawCachedStatConfig, ServerCachedStats, Stat};
use crate::util::jsons::{get_server_cached_stats, write_json};
use crate::util::{analyzer, corpora, memory};
use once_cell::sync::Lazy;
use rayon::prelude::*;
use std::sync::Arc;
use std::time::Instant;

pub static CACHED_STATS: Lazy<ServerCachedStats> = Lazy::new(|| get_server_cached_stats("./cached_stats.json"));

pub fn get(name: &str, corpus: &str) -> Option<Arc<Stat>> {
    if name.is_empty() || corpus.is_empty() {
        return None;
    }
    let name = name.to_lowercase();
    let corpus = corpus.to_lowercase();

    let cached_stats = CACHED_STATS.read().unwrap();
    let stats = cached_stats.get(&name)?.stats.get(&corpus)?;
    Some(Arc::clone(stats))
}

fn get_layout(name: &str) -> Arc<LayoutConfig> {
    memory::get(name).unwrap()
}

fn get_cache(name: &str) -> Option<CachedStatConfig> {
    let cached_stats = CACHED_STATS.read().unwrap();
    let name = name.to_lowercase();
    Some(Arc::clone(cached_stats.get(&name)?))
}

fn cache_fill(ll: &LayoutConfig, data: &mut CachedStats, corpus: &str) {
    let path = format!("./corpora/{}/trigrams.json", corpus);
    let trigrams = corpora::load_corpus(&path);
    let stats = analyzer::trigrams(ll, &trigrams);

    data.insert(corpus.to_string(), Arc::new(stats));
}

fn update(name: &str, data: CachedStatConfig) {
    let mut cached_stats = CACHED_STATS.write().unwrap();
    cached_stats.insert(name.to_string(), data);
}

fn sort() {
    let mut cached_stats = CACHED_STATS.write().unwrap();
    cached_stats.sort_keys();
}

fn cache_files() {
    let start = Instant::now();
    let layouts = memory::LAYOUTS.read().unwrap();
    let names: Vec<&str> = layouts.keys().map(String::as_str).collect();
    let corpus_files = std::fs::read_dir("./corpora/").unwrap();
    let corpus_names: Vec<String> = corpus_files.filter_map(|file| {
        file.ok().and_then(|file| {
            Some(file.path().file_name()?.to_str()?.to_string())
        })
    }).collect();

    names.par_iter().for_each(|name| {
        // let layout_start = Instant::now();
        let ll = get_layout(name);
        let cached = get_cache(name);
        if let Some(cached) = &cached {
            if cached.sum == ll.sum {
                println!("Layout: {}", &ll.name);
                return;
            }
        }

        let mut stats: CachedStats = CachedStats::default();

        for corpus in corpus_names.iter() {
            println!("Layout: {}, Corpus: {}", &ll.name, corpus);
            cache_fill(&ll, &mut stats, corpus);
        }
        let cached = RawCachedStatConfig {
            sum: ll.sum,
            stats,
        };
        update(name, Arc::new(cached));
    });
    sort();

    let duration = start.elapsed();
    println!("Cpu bound elapsed: {:?}", duration);

    let start = Instant::now();
    write_json("./cached_stats.json", &*CACHED_STATS);
    let duration = start.elapsed();
    println!("I/O bound elapsed: {:?}", duration);
}

pub fn cache_main() {
    let start = Instant::now();
    cache_files();
    let duration = start.elapsed();
    println!("Time elapsed: {:?}", duration);
}
