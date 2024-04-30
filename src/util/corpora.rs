use std::sync::{Arc, RwLock};
use fxhash::FxHashMap;
use lazy_static::lazy_static;
use crate::util::jsons::{get_map_str_str, get_corpus};
use crate::util::core::{Corpus, ServerCorpora};

pub const CORPUS: &'static str = "mt-quotes";
pub const NGRAMS: &'static [&'static str; 3] = &["monograms", "bigrams", "trigrams"];

lazy_static!(
    static ref LOADED: ServerCorpora = Arc::new(RwLock::new(FxHashMap::default()));
);

pub fn load_corpus<'a>(path: &str) -> Corpus {
    let loaded = LOADED.read().unwrap();
    if !loaded.contains_key(path) {
        let mut loaded_mut = LOADED.write().unwrap();
        let vec_ = get_corpus(path);
        loaded_mut.insert(path.to_string(), Arc::new(vec_));
    }
    let loaded = LOADED.read().unwrap();
    Arc::clone(loaded.get(path).unwrap())
}

pub fn ngrams<'a>(n: usize, id: u64) -> Corpus {
    let user_corpus = get_user_corpus(id);
    let path = format!("./corpora/{}/{}.json", user_corpus, NGRAMS[n - 1]);
    load_corpus(&path)
}

pub fn words<'a>(id: u64) -> Corpus {
    let user_corpus = get_user_corpus(id);
    let path = format!("./corpora/{}/words.json", user_corpus);
    load_corpus(&path)
}

pub fn get_user_corpus(id: u64) -> String {
    let prefs = get_map_str_str("./corpora.json");
    prefs.get(&id.to_string()).unwrap_or(&CORPUS.to_string()).clone()
}
