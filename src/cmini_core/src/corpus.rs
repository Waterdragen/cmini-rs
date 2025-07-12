use std::ops::Deref;
use fxhash::FxHashMap;
use std::sync::Arc;
use crate::lock::RwLock;
use crate::{ClonableIterator, Key};

pub struct RawCorpus<Gram: AsRef<[Key]>> {
    inner: Arc<[(Gram, u64)]>,
    pub sum: u64,
}

impl<Gram: AsRef<[Key]>> RawCorpus<Gram> {
    pub fn dyn_len_iter(&self) -> Box<dyn ClonableIterator<(&[Key], u64)> + '_> {
        Box::new(
            self.inner.iter()
                .map(|item| (item.0.as_ref(), item.1))
        )
    }
    pub fn arc_clone(&self) -> Self {
        Self {
            inner: Arc::clone(&self.inner),
            sum: self.sum,
        }
    }
}

impl<Gram: AsRef<[Key]>> Deref for RawCorpus<Gram> {
    type Target = [(Gram, u64)];

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl<Gram: AsRef<[Key]>> FromIterator<(Gram, u64)> for RawCorpus<Gram> {
    fn from_iter<T: IntoIterator<Item=(Gram, u64)>>(iter: T) -> Self {
        let inner = iter.into_iter().collect::<Arc<[(Gram, u64)]>>();
        let sum = inner.iter()
            .filter_map(|(gram, freq)|
                gram.as_ref().iter()
                    .all(|key| *key != ' ')
                    .then_some(freq)
            )
            .sum::<u64>();
        Self {
            inner,
            sum,
        }
    }
}

pub type Corpus<const N: usize> = RawCorpus<[Key; N]>;
pub type WordCorpus = RawCorpus<Vec<Key>>;
pub type RawServerCorpora<Corpus> = Arc<RwLock<FxHashMap<String, Corpus>>>;
pub type ServerCorpora<const N: usize> = RawServerCorpora<Corpus<N>>;
pub type ServerWordCorpora = RawServerCorpora<WordCorpus>;