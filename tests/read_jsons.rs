#[cfg(test)]
mod read_jsons {
    use cmini_rs::util::jsons::{get_corpus, read_json};
    use cmini_rs::cmini_core::Key;
    use fxhash::FxHashMap;

    #[test]
    fn test_get_authors() {
        let path = "./authors.json";
        let authors = read_json::<FxHashMap<u64, Vec<String>>>(path);
        assert!(!authors.is_empty());
    }

    #[test]
    fn test_get_links() {
        let path = "./links.json";
        let links = read_json::<FxHashMap<String, String>>(path);
        assert!(!links.is_empty());
    }

    #[test]
    fn test_get_likes() {
        let path = "./likes.json";
        let likes = read_json::<FxHashMap<String, Vec<u64>>>(path);
        assert!(!likes.is_empty());
    }

    #[test]
    fn test_get_pairs() {
        let path = "./pairs.json";
        let pairs = read_json::<Vec<String>>(path);
        assert!(!pairs.is_empty());
    }

    #[test]
    fn test_get_corpus() {
        let path = "./corpora/english-1k/trigrams.json";
        let corpus = &*get_corpus::<[Key; 3]>(path);
        assert!(!corpus.is_empty());
    }
}