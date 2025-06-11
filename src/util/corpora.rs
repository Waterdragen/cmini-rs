use std::fmt::Debug;
use crate::util::core::{Corpus, Key, RawCorpus, RawServerCorpora, ServerCorpora, ServerWordCorpora, WordCorpus};
use crate::util::jsons::{get_corpus, read_json};
use fxhash::FxHashMap;
use glob::glob;
use once_cell::sync::Lazy;
use std::sync::{Arc, RwLock};

pub const CORPUS: &str = "mt-quotes";
pub const NGRAMS: &[&str; 3] = &["monograms", "bigrams", "trigrams"];

static LOADED_1: Lazy<ServerCorpora<1>> = Lazy::new(|| Arc::new(RwLock::new(FxHashMap::default())));
static LOADED_2: Lazy<ServerCorpora<2>> = Lazy::new(|| Arc::new(RwLock::new(FxHashMap::default())));
static LOADED_3: Lazy<ServerCorpora<3>> = Lazy::new(|| Arc::new(RwLock::new(FxHashMap::default())));
static LOADED_WORD: Lazy<ServerWordCorpora> = Lazy::new(|| Arc::new(RwLock::new(FxHashMap::default())));
pub static CORPORA: Lazy<Vec<String>> = Lazy::new(|| list_corpora());
pub static CORPORA_PREFS: Lazy<Arc<RwLock<FxHashMap<u64, String>>>> = Lazy::new(|| read_json("./corpora.json"));

pub trait BorrowCorpus: Sized + TryFrom<Vec<Key>, Error: Debug> {
    fn borrow_corpus() -> &'static RawServerCorpora<Self>;
}
impl BorrowCorpus for [Key; 1] {
    fn borrow_corpus() -> &'static RawServerCorpora<Self> { &LOADED_1 }
}
impl BorrowCorpus for [Key; 2] {
    fn borrow_corpus() -> &'static RawServerCorpora<Self> { &LOADED_2 }
}
impl BorrowCorpus for [Key; 3] {
    fn borrow_corpus() -> &'static RawServerCorpora<Self> { &LOADED_3 }
}
impl BorrowCorpus for Vec<Key> {
    fn borrow_corpus() -> &'static RawServerCorpora<Self> { &LOADED_WORD }
}

pub fn load_corpus<Gram: BorrowCorpus + 'static>(path: &str) -> Arc<RawCorpus<Gram>> {
    {
        let loaded = Gram::borrow_corpus().read().unwrap();
        if loaded.contains_key(path) {
            return Arc::clone(loaded.get(path).unwrap());
        }
    }
    let mut loaded_mut = Gram::borrow_corpus().write().unwrap();
    let corpus = get_corpus(path);
    loaded_mut.insert(path.to_owned(), corpus);
    Arc::clone(loaded_mut.get(path).unwrap())
}

pub fn ngrams<const N: usize>(id: u64) -> Arc<Corpus<N>>
where [Key; N]: BorrowCorpus {
    let user_corpus = get_user_corpus(id);
    let path = format!("./corpora/{}/{}.json", user_corpus, NGRAMS[N - 1]);
    load_corpus::<[Key; N]>(&path)
}

pub fn words(id: u64) -> Arc<WordCorpus> {
    let user_corpus = get_user_corpus(id);
    let path = format!("./corpora/{}/words.json", user_corpus);
    load_corpus(&path)
}

pub fn get_user_corpus(id: u64) -> String {
    let prefs = CORPORA_PREFS.read().unwrap();
    prefs.get(&id)
        .map(|s| s.as_str())
        .unwrap_or(CORPUS).to_owned()
}

pub fn set_user_corpus(id: u64, corpus_name: &str) -> Result<(), ()> {
    let corpus_name = corpus_name.to_lowercase();

    let corpora = list_corpora();
    if !corpora.contains(&corpus_name) {
        return Err(())
    }

    let mut prefs = CORPORA_PREFS.write().unwrap();
    prefs.insert(id, corpus_name);
    Ok(())
}

/// Initializes `CORPORA` names
///
/// # Panics
/// This function will panic if:
/// - Invalid glob pattern
/// - Invalid path
/// - Invalid unicode
#[track_caller]
fn list_corpora() -> Vec<String> {
    let pattern = "corpora/*";
    let mut corpora = glob(pattern)
        .unwrap_or_else(|_| panic!("Invalid glob pattern"))
        .map(|x|
            x.map(|x1|
                x1.into_os_string()
                    .into_string().
                    unwrap_or_else(|_| panic!("Invalid unicode"))
                    [8..].to_owned()))
        .collect::<Result<Vec<_>, _>>()
        .unwrap_or_else(|_| panic!("Path does not exist"));
    corpora.sort();
    corpora
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
