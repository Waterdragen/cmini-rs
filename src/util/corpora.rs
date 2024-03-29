use std::collections::HashMap;
use std::sync::{Arc, Mutex, RwLock};
use lazy_static::lazy_static;
use crate::util::jsons::{get_map_str_str, get_map_str_u64};

pub static CORPUS: &'static str = "mt-quotes";
pub static NGRAMS: &[&str; 3] = &["monograms", "bigrams", "trigrams"];

pub type Corpus = Arc<RwLock<HashMap<String, u64>>>;

lazy_static!(
    static ref LOADED: Arc<Mutex<HashMap<String, Corpus>>>
    = Arc::new(Mutex::new(HashMap::new()));
);

pub fn load_corpus(path: &str) -> Arc<RwLock<HashMap<String, u64>>> {
    let mut loaded = LOADED.lock().unwrap();
    if !loaded.contains_key(path) {
        let map = get_map_str_u64(path);
        loaded.insert(path.to_string(), Arc::new(RwLock::new(map)));
    }
    let map = loaded.get(path).unwrap();
    Arc::clone(map)
}

pub fn ngrams(n: usize, id: u64) -> Corpus {
    let user_corpus = get_user_corpus(id);
    let path = format!("./corpora/{}/{}.json", user_corpus, NGRAMS[n - 1]);
    load_corpus(&path)
}

pub fn words(id: u64) -> Corpus {
    let user_corpus = get_user_corpus(id);
    let path = format!("./corpora/{}/words.json", user_corpus);
    load_corpus(&path)
}

pub fn get_user_corpus(id: u64) -> String {
    let prefs = get_map_str_str("./corpora.json");
    prefs.get(&id.to_string()).unwrap_or(&CORPUS.to_string()).clone()
}
