use crate::core::{FingerCombo, FxIndexMap, Metric, RawCorpus, Table};
use crate::util::corpora::BorrowCorpus;
use fxhash::FxHashMap;
use serde::de::DeserializeOwned;
use serde::Serialize;
use std::fs::File;
use std::io::{BufRead, BufReader};
use thiserror::Error;
use crate::core::Finger;

#[derive(Debug, Error)]
pub enum JsonError {
    #[error("File does not exist")]
    FileNotFound,
    #[error(transparent)]
    ParseFails(#[from] serde_json::Error),
    #[error(transparent)]
    Io(#[from] std::io::Error),
}

pub fn read_json_checked<T: DeserializeOwned>(path: &str) -> Result<T, JsonError> {
    let file = File::open(path).map_err(|_| JsonError::FileNotFound)?;
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

#[track_caller]
pub fn read_json_allow_empty<T: DeserializeOwned + Default>(path: &str) -> T {
    let file = File::open(path).unwrap();
    let mut reader = BufReader::new(file);
    match reader.fill_buf() {
        Ok(&[]) => T::default(),
        _ => serde_json::from_reader(reader).unwrap(),
    }
}

/// Reads corpus files with given file path, and converts into target `Gram`
///
/// # Panics
/// This function will panic if:
/// - File does not exist
/// - Fails to parse JSON
/// - Fails to convert `Vec<char>` into `Gram` due to length mismatch
#[track_caller]
pub fn get_corpus<Gram>(path: &str) -> RawCorpus<Gram>
where Gram: BorrowCorpus {
    let map = read_json::<FxIndexMap<String, u64>>(path);
    map.into_iter()
        .map(|(gram, freq)| {
            let chars = gram.to_lowercase().chars().collect::<Vec<_>>();
            (chars.try_into().unwrap(), freq)
        })
        .collect()
}

#[track_caller]
pub fn get_table(path: &str) -> Table {
    let fingers: FxHashMap<String, u8> = FxHashMap::from_iter([
            ("LP", 0u8), ("LR", 1), ("LM", 2), ("LI", 3), ("LT", 4),
            ("RT", 5), ("RI", 6), ("RM", 7), ("RR", 8), ("RP", 9)
        ]
        .into_iter()
        .map(|(finger, value)| { (finger.to_string(), value) })
    );
    let map = read_json::<FxHashMap<String, String>>(path);
    let mut table = [Metric::Unknown; 1000];
    for (finger_combo, gram_type) in map.iter() {
        let finger0 = Finger::from_u8(fingers[&finger_combo[0..2]]);
        let finger1 = Finger::from_u8(fingers[&finger_combo[2..4]]);
        let finger2 = Finger::from_u8(fingers[&finger_combo[4..6]]);
        let finger_combo = FingerCombo::from([finger0, finger1, finger2]);
        let gram_type = Metric::try_from(gram_type.as_str()).unwrap();
        table[finger_combo.index()] = gram_type;
    }
    Table::from_inner(table)
}

pub fn write_json_checked<T>(path: &str, t: &T) -> Result<(), JsonError>
where T: ?Sized + Serialize {
    let file = File::create(path)?;
    Ok(serde_json::to_writer_pretty(file, t)?)
}

#[track_caller]
pub fn write_json<T>(path: &str, t: &T) where T: ?Sized + Serialize {
    write_json_checked(path, t).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::Key;

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
        let vec_ = &*get_corpus::<[Key; 3]>(path);
        dbg!(vec_);
    }
}
