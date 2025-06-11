use std::error::Error;
use std::fmt::Debug;
use crate::util::core::{CachedStatConfig, FxIndexMap, JsonCachedStatConfig, Key, Metric, RawCachedStatConfig, RawCorpus, ServerCachedStats};
use fxhash::FxHashMap;
use serde_json::Value;
use std::fs::File;
use std::io::BufReader;
use std::sync::{Arc, RwLock};
use serde::de::DeserializeOwned;
use serde::Serialize;

fn read_json_checked<T: DeserializeOwned>(path: &str) -> Result<T, Box<dyn Error>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    Ok(serde_json::from_reader(reader)?)
}

/// Reads json with given file path
///
/// # Panics
/// This function will panic if:
/// - File does not exist
/// - Fails to parse JSON
#[track_caller]
pub fn read_json<T: DeserializeOwned>(path: &str) -> T {
    read_json_checked(path).unwrap()
}

/// Reads corpus files with given file path, and converts into target `Gram`
///
/// # Panics
/// This function will panic if:
/// - File does not exist
/// - Fails to parse JSON
/// - Fails to convert `Vec<char>` into `Gram` due to length mismatch
#[track_caller]
pub fn get_corpus<Gram>(path: &str) -> Arc<RawCorpus<Gram>>
where Gram: TryFrom<Vec<Key>, Error: Debug> {
    let json = read_json::<Value>(path);
    let obj = json.as_object().unwrap();
    obj.into_iter()
        .map(|(key, value)| {
            let mut chars: Vec<Key> = Vec::with_capacity(3);
            key.to_lowercase().chars().for_each(|c| {
                chars.push(c);
            });
            let number: u64 = value.as_u64().unwrap_or(0);
            (chars.try_into().unwrap(), number)
        })
        .collect()
}

#[track_caller]
pub fn get_server_cached_stats(path: &str) -> ServerCachedStats {
    let json = match read_json_checked::<Value>(path) {
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

#[track_caller]
pub fn get_table(path: &str) -> [Metric; 4096] {
    let fingers: FxHashMap<String, u16> = FxHashMap::from_iter([
            ("LP", 0u16), ("LR", 1), ("LM", 2), ("LI", 3), ("LT", 4),
            ("RT", 5), ("RI", 6), ("RM", 7), ("RR", 8), ("RP", 9)
        ]
        .into_iter()
        .map(|(finger, value)| { (finger.to_string(), value) })
    );

    let json = read_json::<Value>(path);
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

fn write_json_checked<T>(path: &str, t: &T) -> Result<(), Box<dyn Error>>
where T: ?Sized + Serialize {
    let file = File::create(path)?;
    Ok(serde_json::to_writer_pretty(file, t)?)
}

#[track_caller]
pub fn write_json<T>(path: &str, t: &T) where T: ?Sized + Serialize {
    write_json_checked(path, t).unwrap()
}

#[track_caller]
pub fn write_cached_stats(path: &str, cached_stats: &ServerCachedStats) {
    let file = File::create(path).unwrap();
    serde_json::to_writer_pretty(file, cached_stats).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_map_str_str() {
        let path = "./links.json";
        let map = read_json::<FxHashMap<String, String>>(path);
        dbg!(map);
    }

    #[test]
    fn test_get_vec_str() {
        let path = "./pairs.json";
        let vec = read_json::<Vec<String>>(path);
        dbg!(vec);
    }

    #[test]
    fn test_get_map_str_vec_u64() {
        let path = "./likes.json";
        let map = read_json::<FxHashMap<String, Vec<u64>>>(path);
        dbg!(map);
    }

    #[test]
    fn test_get_map_u64_vec_str() {
        let path = "./authors.json";
        let map = read_json::<FxHashMap<u64, Vec<String>>>(path);
        dbg!(map);
    }

    #[test]
    fn test_get_corpus() {
        let path = "./corpora/english-1k/trigrams.json";
        let vec_ = get_corpus(path);
        dbg!(vec_);
    }
}
