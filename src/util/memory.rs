use crate::core::ServerLayouts;
use crate::core::authors::Authors;
use crate::prelude::*;
use crate::util::corpora::CORPORA_PREFS;
use crate::util::jsons::{read_json, write_json};
use crate::util::links::LINKS;
use fxhash::{FxBuildHasher, FxHashMap, FxHashSet};
use std::fmt::Debug;
use std::fs::File;
use std::io::{BufRead, BufReader};
use thiserror::Error;
use crate::util::admins::Admins;
use rphonetic::{Encoder, MatchRatingApproach};
use crate::util::cache;

// Lazy statics are asserted by validate_json
pub static AUTHORS: Lazy<Arc<RwLock<Authors>>> = Lazy::new(||
    Arc::new(RwLock::new(Authors::open("./authors.json").unwrap()))
);
pub static ADMINS: Lazy<Admins> = Lazy::new(|| Admins::open("admins.json"));

pub static LAYOUTS: Lazy<ServerLayouts> = Lazy::new(|| read_json("./layouts.json"));
pub static LIKES: Lazy<RwLock<FxHashMap<String, Vec<u64>>>> = Lazy::new(|| read_json("./likes.json"));
pub static PLACES: Lazy<Vec<String>> = Lazy::new(|| read_json("./places.json"));
pub static PAIRS: Lazy<FxHashSet<[char; 2]>> = Lazy::new(||
    read_json::<Vec<String>>("./pairs.json")
        .into_iter()
        .map(|s| {
            let mut chars = s.chars();
            [chars.next().unwrap(), chars.next().unwrap()]
        }).collect()
);
pub static DIRECT_WRITE_LOCK: Lazy<Mutex<()>> = Lazy::new(|| Mutex::new(()));

#[derive(Debug, Error)]
pub enum RemoveError<'a> {
    #[error("Error: `{0}` does not exist")]
    NotFound(&'a str),
    #[error("Error: you don't own `{0}`")]
    NotOwner(&'a str),
    #[error("Use commands with `--sudo` in a public channel")]
    SudoInPrivateChannel,
}

pub fn get_like_count(name: &str) -> usize {
    let likes = LIKES.read();
    match likes.get(name) {
        Some(liked_users) => liked_users.len(),
        None => 0,
    }
}

pub static PHONETIC_CODE_FREQS: Lazy<FxHashMap<String, (u64, String)>> = Lazy::new(|| {
    let mut freqs = FxHashMap::with_capacity_and_hasher(326609, FxBuildHasher::default());  // Please refer to the line count of freq.json
    let file = BufReader::new(File::open("./freq.json").unwrap());
    let match_rating = MatchRatingApproach;
    file.lines()
        .skip(1)
        .filter_map(|s| {
            let s = s.ok()?;
            let mut split = s.split('"');
            let words = [split.next()?, split.next()?, split.next()?, split.next()?];
            let word = words[1].to_owned();
            let code = match_rating.encode(&word);
            let freq = words[3].parse::<u64>().ok()?;
            Some((code, (freq, word)))
        })
        .for_each(|(code, freq)| { freqs.insert(code, freq); });
    freqs
});

pub fn sync_data_local() {
    write_json("./admins.json", &*ADMINS);
    write_json("./authors.json", &*AUTHORS);
    write_json("./corpora.json", &*CORPORA_PREFS);
    write_json("./layouts.json", &*LAYOUTS);
    write_json("./likes.json", &*LIKES);
    write_json("./links.json", &*LINKS);
    cache::cache_main();
}
