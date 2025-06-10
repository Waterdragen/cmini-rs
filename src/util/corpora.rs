use std::sync::{Arc, RwLock};
use fxhash::FxHashMap;
use once_cell::sync::Lazy;
use glob::glob;
use crate::util::jsons::{get_map_str_str, get_corpus, write_map_str_str};
use crate::util::core::{Corpus, ServerCorpora};

pub const CORPUS: &str = "mt-quotes";
pub const NGRAMS: &[&str; 3] = &["monograms", "bigrams", "trigrams"];

static LOADED: Lazy<ServerCorpora> = Lazy::new(|| Arc::new(RwLock::new(FxHashMap::default())));
static CORPORA: Lazy<Arc<RwLock<()>>> = Lazy::new(|| Arc::new(RwLock::new(())));

pub fn load_corpus(path: &str) -> Corpus {
    {
        let loaded = LOADED.read().unwrap();
        if loaded.contains_key(path) {
            return Arc::clone(loaded.get(path).unwrap());
        }
    }
    let mut loaded_mut = LOADED.write().unwrap();
    let vec_ = get_corpus(path);
    loaded_mut.insert(path.to_string(), Arc::new(vec_));
    Arc::clone(loaded_mut.get(path).unwrap())
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
    let _read = CORPORA.read().unwrap();
    let prefs = get_map_str_str("./corpora.json");
    prefs.get(&id.to_string())
        .map(|s| s.as_str())
        .unwrap_or(CORPUS).to_owned()
}

pub fn set_user_corpus(id: u64, corpus_name: &str) -> Result<(), ()> {
    let _write = CORPORA.write().unwrap();
    let corpus_name = corpus_name.to_lowercase();

    let corpora = list_corpora();
    if !corpora.contains(&corpus_name) {
        return Err(())
    }

    let mut prefs = get_map_str_str("./corpora.json");
    prefs.insert(id.to_string(), corpus_name);
    write_map_str_str("./corpora.json", &prefs);
    Ok(())
}

pub fn list_corpora() -> Vec<String> {
    let pattern = "corpora/*";
    glob(pattern)
        .unwrap_or_else(|_| panic!("Invalid glob pattern"))
        .map(|x|
            x.map(|x1|
                x1.into_os_string()
                    .into_string().
                    unwrap_or_else(|_| panic!("Invalid unicode"))
                    [8..].to_owned()))
        .collect::<Result<Vec<_>, _>>()
        .unwrap_or_else(|_| panic!("Path does not exist"))
}

#[test]
fn test_glob() {
    let pattern = "corpora/*";

    for entry in glob(pattern).expect("Failed to read glob pattern") {
        match entry {
            Ok(path) => println!("{:?}", path),
            Err(e) => println!("Error: {:?}", e),
        }
    }
}
