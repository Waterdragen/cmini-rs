use std::fs::File;
use std::io::Read;
use std::sync::{Arc, RwLock};
use serde_json::Value;
use fxhash::{FxHashMap};
use crate::util::core::{CachedStatConfig, FxIndexMap, JsonLayoutConfig, JsonCachedStatConfig, LayoutConfig, Metric, RawLayoutConfig, RawCachedStatConfig, RawCorpus, ServerCachedStats, ServerLayouts, Key};

fn read_json(path: &str) -> Value {
    let mut file = File::open(path).unwrap_or_else(
        |_| panic!("Failed to open file {}", path)
    );
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap_or_else(|_| panic!("Failed to read file"));
    let json: Value = serde_json::from_str(&contents).unwrap_or_else(|_| panic!("Failed to parse JSON {}", path));
    json
}

fn read_json_checked(path: &str) -> Result<Value, String> {
    let mut file = match File::open(path) {
        Ok(file) => file,
        Err(_) => return Err(format!("Failed to open file {}", path)),
    };
    let mut contents = String::new();

    let _ = file.read_to_string(&mut contents)
                .map_err(|_| "Failed to read file".to_owned())?;

    serde_json::from_str::<Value>(&contents)
        .map_err(|e| e.to_string())
}

pub fn get_map_str_str(path: &str) -> FxHashMap<String, String> {
    let json = read_json(path);
    let mut map: FxHashMap<String, String> = FxHashMap::default();

    let obj = json.as_object().unwrap();

    for (key, value) in obj {
        map.insert(key.clone(), value.as_str().unwrap().to_string());
    }
    map
}

pub fn get_vec_str(path: &str) -> Vec<String> {
    let json = read_json(path);
    let arr = json.as_array().unwrap();
    arr.iter()
        .map(|val| {val.as_str().unwrap().to_string()})
        .collect()
}

pub fn get_map_str_vec_u64(path: &str) -> FxHashMap<String, Vec<u64>> {
    let json = read_json(path);
    let mut map: FxHashMap<String, Vec<u64>> = FxHashMap::default();

    let obj = json.as_object().unwrap();
    for (key, value) in obj {
        let arr = value.as_array().unwrap();
        let vec_ = arr
            .iter()
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

pub fn get_server_layouts(path: &str) -> ServerLayouts {
    let json = read_json_checked(path).unwrap_or_else(|e| panic!("Failed to parse server layouts. Cause: {e}"));
    let raw_layouts: FxIndexMap<String, JsonLayoutConfig> = serde_json::from_value(json).expect("Failed to parse layout.json");
    let layouts: FxIndexMap<String, LayoutConfig> = raw_layouts
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
            let mut chars: Vec<Key> = Vec::with_capacity(3);
            key.to_lowercase().chars().for_each(|c| {
                chars.push(c);
            });
            let number: u64 = value.as_u64().unwrap_or(0);
            (chars, number)
        })
        .collect()
}

pub fn get_server_cached_stats(path: &str) -> ServerCachedStats {
    let json = match read_json_checked(path) {
        Ok(json) => json,
        Err(_) => return Arc::new(RwLock::new(FxIndexMap::default())),
    };
    let obj = json.as_object().unwrap();
    let raw_cached_stats: FxIndexMap<String, CachedStatConfig> = FxIndexMap::from_iter(
        obj.iter().map(|(key, value)| {
            let json_cached: JsonCachedStatConfig = serde_json::from_value(value.clone()).unwrap();
            (key.clone(), Arc::new(RawCachedStatConfig::from_json(json_cached)))
    }));
    Arc::new(RwLock::new(raw_cached_stats))
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
                      item.1
                          .as_f64()
                          .map(|f64_| (item.0.to_string(), f64_))
            }).collect();
        map.insert(corpus.clone(), stat_map);
    }
    map
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
        let gram_type = Metric::from_str(gram_type.as_str().unwrap());
        table[usize::from(hash_value)] = gram_type;
    }

    table
}

pub fn write_layouts(path: &str, lls: &ServerLayouts) {
    let file = File::create(path).unwrap();
    serde_json::to_writer_pretty(file, lls).unwrap()
}

pub fn write_map_u64_vec_str(path: &str, map: &FxHashMap<u64, Vec<String>>) {
    let file = File::create(path).unwrap();
    serde_json::to_writer_pretty(file, map).unwrap();
}

pub fn write_map_str_str(path: &str, map: &FxHashMap<String, String>) {
    let file = File::create(path).unwrap();
    serde_json::to_writer_pretty(file, map).unwrap()
}

pub fn write_cached_stats(path: &str, cached_stats: &ServerCachedStats) {
    let file = File::create(path).unwrap();
    serde_json::to_writer_pretty(file, cached_stats).unwrap()
}

#[cfg(test)]
mod tests {
    use std::hash::Hash;
    use super::*;

    #[test]
    fn test_get_map_str_str() {
        let path = "./links.json";
        let map = get_map_str_str(path);
        for (key, value) in map.into_iter() {
            println!("{}, {}", key, value);
        }
    }

    #[test]
    fn test_get_vec_str() {
        let path = "./pairs.json";
        let vec = get_vec_str(path);
        for value in vec.into_iter() {
            println!("{}", value);
        }
    }

    #[test]
    fn test_get_map_str_vec_u64() {
        let path = "./likes.json";
        let map = get_map_str_vec_u64(path);
        for (key, vec_) in map.into_iter() {
            println!("{}, {:?}", key, vec_);
        }
    }

    #[test]
    fn test_get_map_u64_vec_str() {
        let path = "./authors.json";
        let map = get_map_u64_vec_str(path);
        for (key, value) in map {
            println!("{key}: {:?}", value);
        }
    }

    #[test]
    fn test_get_corpus() {
        let path = "./corpora/english-1k/trigrams.json";
        let vec_ = get_corpus(path);
        println!("{:?}", vec_);
    }
}
