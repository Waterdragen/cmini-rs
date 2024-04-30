use std::path::Path;
use rayon::prelude::*;
use std::time::Instant;
use fxhash::FxHashMap;
use crate::util::jsons::{get_cached_stat, write_cached_stat};
use crate::util::core::{CachedStat, Corpus, RawLayoutConfig, Metric, LayoutConfig};
use crate::util::{analyzer, corpora, memory};

fn get_layout(name: &str) -> LayoutConfig {
    memory::get(name).unwrap()
}

fn get_cache(name: &str) -> Option<CachedStat> {
    let name = name.to_lowercase();
    let path = format!("./cache/{}.json", name);
    if !Path::new(&path).exists() {
        return None;
    }
    Some(get_cached_stat(&path))
}

fn cache_fill<'a>(ll: &RawLayoutConfig, data: Option<&'a mut CachedStat>, corpus: &str) -> &'a mut CachedStat {
    let path = format!("./corpora/{}/trigrams.json", corpus);
    let trigrams: Corpus = corpora::load_corpus(&path);
    let stats = analyzer::trigrams(ll, &trigrams);

    match data {
        Some(data) => {
            data.insert(corpus.to_string(), stats);
            data
        },
        None => {
            let mut update: CachedStat = FxHashMap::default();
            update.insert(corpus.to_string(), stats);
            Box::leak(Box::new(update))
        },
    }
}

fn update<'a>(name: &str, data: &'a mut CachedStat) -> &'a mut CachedStat {
    write_cached_stat(&format!("./cache/{}.json", name), data);
    data
}

pub fn get<'a>(name: &str, corpus: &str) -> Option<FxHashMap<Metric, f64>> {
    if name == "" || corpus == "" {
        return None;
    }
    let name = name.to_lowercase();
    let corpus = corpus.to_lowercase();

    let data = get_cache(&name);
    let mut data = data.unwrap_or_else(|| FxHashMap::default());

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

    // let mut layout_times: Vec<Duration> = Vec::new();
    // let corpora_count = corpus_names.len();
    // let mut update_times = Duration::from_millis(0);
    names.iter().for_each(|name| {
        // let layout_start = Instant::now();
        let ll = get_layout(name);
        let mut data = get_cache(&name);
        let mut data: Option<&mut CachedStat> = data.as_mut();

        for corpus in corpus_names.iter() {
            println!("Layout: {}, Corpus: {}", ll.name, corpus);
            data = Some(cache_fill(&ll, data, corpus));
        }
        // let update_start = Instant::now();
        update(&name, data.unwrap());
        // layout_times.push(layout_start.elapsed());
        // update_times += update_start.elapsed();
    });
    // println!("Initial cache fill: {:?}ms", layout_times[..corpora_count].iter().map(|v| v.as_secs_f64()).sum::<f64>() * 1000.0 / corpora_count as f64);
    // println!("Subsequent cache fill: {:?}ms", layout_times[corpora_count..].iter().map(|v| v.as_secs_f64()).sum::<f64>() * 1000.0 / corpora_count as f64 / 9.0);println!("Initial cache fill: {:?}ms", layout_times[..corpora_count].iter().map(|v| v.as_secs_f64()).sum::<f64>() * 1000.0 / corpora_count as f64);
    // println!("Initial layout: {:?}ms", layout_times[0].as_secs_f64() * 1000.0);
    // println!("Subsequent layout: {:?}ms", layout_times[1..].iter().map(|v| v.as_secs_f64()).sum::<f64>() * 1000.0 / 9.0);
    // println!("update: {:?}", update_times / 10);
}

pub fn cache_main() {
    let start = Instant::now();
    cache_files();
    let duration = start.elapsed();
    println!("Time elapsed: {:?}", duration);
}
