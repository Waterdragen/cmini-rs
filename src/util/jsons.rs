use std::fs::File;
use std::io::Read;
use std::sync::{Arc, RwLock};
use serde_json::Value;
use fxhash::{FxHashMap};
use indexmap::IndexMap;
use crate::util::core::{CachedStat, RawLayoutConfig, Metric, RawCorpus, JsonLayoutConfig, LayoutConfig, ServerLayouts};

fn read_json(path: &str) -> Value {
    let mut file = File::open(path).expect(
        format!("Failed to open file {}", path).as_str()
    );
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Failed to read file");
    let json: Value = serde_json::from_str(&contents).expect("Failed to parse JSON");
    json
}

pub fn get_map_str_str(path: &str) -> FxHashMap<String, String> {
    let json = read_json(path);
    let mut map: FxHashMap<String, String> = FxHashMap::default();

    let obj = json.as_object().unwrap();

    for (key, value) in obj {
        map.insert(key.clone(), value.to_string());
    }
    map
}

pub fn get_vec_str(path: &str) -> Vec<String> {
    let json = read_json(path);
    let arr = json.as_array().unwrap();
    arr.into_iter()
        .map(|val| {val.to_string()})
        .collect()
}

pub fn get_map_str_vec_u64(path: &str) -> FxHashMap<String, Vec<u64>> {
    let json = read_json(path);
    let mut map: FxHashMap<String, Vec<u64>> = FxHashMap::default();

    let obj = json.as_object().unwrap();
    for (key, value) in obj {
        let arr = value.as_array().unwrap();
        let vec_ = arr
            .into_iter()
            .filter_map(|val| val.as_u64())
            .collect();
        map.insert(key.clone(), vec_);
    }
    map
}

pub fn get_map_str_u64(path: &str) -> FxHashMap<String, u64> {
    let json = read_json(path);
    let mut map: FxHashMap<String, u64> = FxHashMap::default();

    let obj = json.as_object().unwrap();
    for (key, value) in obj {
        map.insert(key.clone(), value.as_u64().unwrap());
    }
    map
}

pub fn get_raw_layouts(path: &str) -> ServerLayouts {
    let json = read_json(path);
    let raw_layouts: IndexMap<String, JsonLayoutConfig> = serde_json::from_value(json).expect("Failed to parse layout.json");
    let layouts: IndexMap<String, LayoutConfig> = raw_layouts
        .into_iter()
        .map(|(name, raw_layout)| {
            (name.clone(), Arc::new(RawLayoutConfig::from_json(&name, raw_layout)))
        })
        .collect();
    Arc::new(RwLock::new(layouts))
}

pub fn get_map_u64_vec_str(path: &str) -> FxHashMap<u64, Vec<String>> {
    let json = read_json(path);
    let mut map: FxHashMap<u64, Vec<String>> = FxHashMap::default();
    let obj = json.as_object().unwrap();
    for (id_str, names) in obj {
        match id_str.parse::<u64>() {
            Err(_) => continue,
            Ok(u64_) => {
                let arr = names.as_array().unwrap();
                let arr: Vec<String> = arr.iter()
                                          .filter_map(|v| v.as_str())
                                          .map(|s| s.to_string())
                                          .collect();
                map.insert(u64_, arr);
            },
        }
    }
    map
}

pub fn get_corpus(path: &str) -> RawCorpus {
    let json = read_json(path);
    let obj = json.as_object().unwrap();
    obj.into_iter()
        .map(|(key, value)| {
            let mut chars: Vec<char> = Vec::with_capacity(3);
            key.to_lowercase().chars().for_each(|c| {
                chars.push(c);
            });
            let number: u64 = value.as_u64().unwrap_or(0);
            (chars, number)
        })
        .collect()
}

#[deprecated]
pub fn get_map_str_map_str_f64(path: &str) -> FxHashMap<String, FxHashMap<String, f64>> {
    let json = read_json(path);
    let mut map: FxHashMap<String, FxHashMap<String, f64>> = FxHashMap::default();
    let obj = json.as_object().unwrap();
    for (corpus, stat) in obj {
        let stat = stat.as_object().unwrap();
        let stat_map: FxHashMap<String, f64>
            = stat.iter()
                  .filter_map(|item| {
                match item.1.as_f64() {
                    Some(f64_) => Some((item.0.to_string(), f64_)),
                    None => None,
                }
            }).collect();
        map.insert(corpus.clone(), stat_map);
    }
    map
}

pub fn get_cached_stat(path: &str) -> CachedStat {
    let json = read_json(path);
    let mut cached_stat: CachedStat = FxHashMap::default();

    let obj = json.as_object().unwrap();
    for (corpus, stat) in obj {
        let stat = stat.as_object().unwrap();
        let stat: FxHashMap<Metric, f64> = stat.iter()
            .map(|(metric, freq)| {
                (Metric::from_string(metric), freq.as_f64().unwrap())
            }).collect();
        cached_stat.insert(corpus.clone(), stat);
    }
    cached_stat
}

pub fn get_table(path: &str) -> [Metric; 4096] {
    let fingers: FxHashMap<String, u16> = FxHashMap::from_iter([
            ("LP", 0u16), ("LR", 1), ("LM", 2), ("LI", 3), ("LT", 4),
            ("RT", 5), ("RI", 6), ("RM", 7), ("RR", 8), ("RP", 9)
        ]
        .into_iter()
        .map(|(finger, value)| { (finger.to_string(), value) })
    );

    let json = read_json(path);
    let mut table = [Metric::Unknown; 4096];
    let obj = json.as_object().unwrap();
    for (finger_combo, gram_type) in obj {
        let finger0 = *fingers.get(&finger_combo[0..2]).unwrap();
        let finger1 = *fingers.get(&finger_combo[2..4]).unwrap();
        let finger2 = *fingers.get(&finger_combo[4..6]).unwrap();
        let hash_value = (finger0 << 8) | (finger1 << 4) | finger2;
        let gram_type = Metric::from_string(gram_type.as_str().unwrap());
        table[usize::from(hash_value)] = gram_type;
    }
    println!("{:?}", table[0]);

    table
}

pub fn write_cached_stat(path: &str, map: &mut CachedStat) {
    let file = File::create(path).unwrap();
    serde_json::to_writer_pretty(file, map).unwrap();
}

pub fn write_layouts(path: &str, lls: &ServerLayouts) {
    let file = File::create(path).unwrap();
    serde_json::to_writer_pretty(file, lls).unwrap()
}

pub fn write_map_u64_vec_str(path: &str, map: &FxHashMap<u64, Vec<String>>) {
    let file = File::create(path).unwrap();
    serde_json::to_writer_pretty(file, map).unwrap();
}

#[cfg(test)]
mod tests {
    use std::hash::Hash;
    use super::*;

    fn test_get_map_str_str() {
        let path = "./links.json";
        let map = get_map_str_str(path);
        for (key, value) in map.into_iter() {
            println!("{}, {}", key, value);
        }
    }

    fn test_get_vec_str() {
        let path = "./pairs.json";
        let vec = get_vec_str(path);
        for value in vec.into_iter() {
            println!("{}", value);
        }
    }

    fn test_get_map_str_vec_u64() {
        let path = "./likes.json";
        let map = get_map_str_vec_u64(path);
        for (key, vec_) in map.into_iter() {
            println!("{}, {:?}", key, vec_);
        }
    }

    fn test_get_map_u64_vec_str() {
        let path = "./authors.json";
        let map = get_map_u64_vec_str(path);
        for (key, value) in map {
            println!("{key}: {:?}", value);
        }
    }

    #[deprecated]
    fn test_get_map_str_map_str_f64() {
        let path = "./cache/a02.json";
        let map = get_map_str_map_str_f64(path);
        for (key, value) in map {
            println!("{key}: {:?}", value);
        }
    }

    fn test_get_corpus() {
        let path = "./corpora/english-1k/trigrams.json";
        let vec_ = get_corpus(path);
        println!("{:?}", vec_);
    }
}
