use crate::core::{FxIndexMap, RawCorpus};
use crate::util::corpora::BorrowCorpus;
use serde::de::DeserializeOwned;
use serde::Serialize;
use std::fs::File;
use std::io::{BufRead, BufReader};
use thiserror::Error;

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
/// # Note
/// - grams are converted into lowercase
/// - vec does not merge duplicates, might affect most common gram counting
// FIXME: implement scripts to merge corpora counts
///
/// # Panics
/// This function will panic if:
/// - File does not exist
/// - Fails to parse JSON
/// - Fails to convert `Vec<char>` into `Gram` due to length mismatch
///     - note: some characters becomes two when converted into lowercase, they might fail here
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

pub fn write_json_checked<T>(path: &str, t: &T) -> Result<(), JsonError>
where T: ?Sized + Serialize {
    let file = File::create(path)?;
    Ok(serde_json::to_writer_pretty(file, t)?)
}

#[track_caller]
pub fn write_json<T>(path: &str, t: &T) where T: ?Sized + Serialize {
    write_json_checked(path, t).unwrap()
}
