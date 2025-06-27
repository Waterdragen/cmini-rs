use crate::core::conv;
use crate::core::{FxIndexMap, Key, LayoutConfig, CachedStat, CachedStatConfig, ServerCachedStats};
use crate::util::corpora::{self, CORPORA};
use crate::util::jsons::{read_json, write_json};
use crate::util::memory::LAYOUTS;
use once_cell::sync::Lazy;
use rayon::prelude::*;
use std::io::Write;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use std::time::Instant;

pub static CACHED_STATS: Lazy<ServerCachedStats> = Lazy::new(|| read_json("./cached_stats.json"));

fn get_cache(name: &str) -> Option<Arc<CachedStatConfig>> {
    let cached_stats = CACHED_STATS.read();
    let name = name.to_lowercase();
    Some(Arc::clone(cached_stats.get(&name)?))
}

fn cache_fill(ll: &LayoutConfig, data: &mut FxIndexMap<String, CachedStat>, corpus: &str, path: &str) {
    let trigrams = corpora::read_corpus(path);
    let stats = ll.trigram_stats(&trigrams);

    data.insert(corpus.to_string(), CachedStat(stats));
}

fn update(name: String, data: Arc<CachedStatConfig>) {
    let mut cached_stats = CACHED_STATS.write();
    cached_stats.insert(name, data);
}

fn sort() {
    let mut cached_stats = CACHED_STATS.write();
    cached_stats.sort_keys();
}

fn cache_files() {
    let start = Instant::now();
    let names = {
        let layouts = LAYOUTS.read();
        layouts.keys().cloned().collect::<Vec<_>>()
    };
    let corpora = CORPORA.iter()
        .map(|corpus| (corpus.to_owned(), format!("./corpora/{}/trigrams.json", corpus)))
        .collect::<Vec<_>>();
    for (_, path) in corpora.iter() {
        corpora::load_corpus::<[Key; 3]>(path);  // preload every corpus
    }
    let counter = AtomicUsize::new(0);
    let total = names.len();
    print!("\n\n");

    names.into_par_iter().for_each(|owned_name| {
        let c = counter.load(Ordering::Relaxed) + 1;
        counter.store(c, Ordering::Relaxed);
        print!("\x1B[1A\x1B[2K({c}/{total}) Caching `{owned_name}`\n\r");
        std::io::stdout().flush().unwrap();

        let layouts = LAYOUTS.arc_clone();
        let name = owned_name.as_str();
        let get_ll = layouts.get(name);
        let ll = &*get_ll;
        let cached = get_cache(name);
        if let Some(cached) = &cached {
            if cached.sum == ll.sum {
                // println!("Layout: {}", &ll.name);
                return;
            }
        }

        let mut stats: FxIndexMap<String, CachedStat> = FxIndexMap::default();

        for (corpus, path) in corpora.iter() {
            // println!("Layout: {}, Corpus: {}", &ll.name, corpus);
            cache_fill(ll, &mut stats, corpus, path);
        }
        let keys = conv::layout::pack(&ll.keys);
        let cached = CachedStatConfig {
            keys,
            user: ll.user,
            sum: ll.sum,
            stats,
        };
        drop(get_ll);  // Unborrow `name` to use owned_name
        update(owned_name, Arc::new(cached));
    });
    sort();
    println!();

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
