use std::fmt::{Debug, Formatter};
use crate::util::jsons::read_json;
use fxhash::FxHashMap;
use once_cell::sync::Lazy;
use std::sync::{Arc, RwLock};
use serde::{Serialize, Serializer};
use strsim::jaro_winkler;

pub static AUTHORS: Lazy<Arc<RwLock<Authors>>> = Lazy::new(||
    Arc::new(RwLock::new(Authors::open("./authors.json").unwrap()))
);

pub struct Authors {
    id_to_str: FxHashMap<u64, Vec<String>>,
    str_to_id: FxHashMap<String, u64>,
}

impl Authors {
    pub fn open(path: &str) -> Option<Self> {
        let id_to_str = read_json::<FxHashMap<u64, Vec<String>>>(path);
        // Check if hashmap is empty or any Vec is empty
        if id_to_str.is_empty() || id_to_str.values().any(|v| v.is_empty()) {
            return None;
        }

        let str_to_id = id_to_str
            .iter()
            .flat_map(|(&id, names)|
                names.iter().map(move |name| (name.to_owned(), id)))
            .collect();

        Some(Self {
            id_to_str,
            str_to_id,
        })
    }

    pub fn len(&self) -> usize {
        self.id_to_str.len()
    }
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn get_name(&self, id: u64) -> Option<&str> {
        let names = self.id_to_str.get(&id)?;
        Some(&names[0])
    }

    pub fn get_id(&self, name: &str) -> u64 {
        self.str_to_id
            .iter()
            .max_by(|(name1, _), (name2, _)| {
                jaro_winkler(name1, name)
                    .partial_cmp(&jaro_winkler(name2, name))
                    .unwrap_or(std::cmp::Ordering::Equal)
            })
            .map(|(_, id)| *id)
            .unwrap()  // id_to_str is always non-empty
    }

    pub fn update(&mut self, id: u64, new_name: String) {
        match self.id_to_str.get_mut(&id) {
            None => {
                // New author
                self.id_to_str.insert(id, vec![new_name.clone()]);
            }
            Some(names) => if !names.contains(&new_name) {
                // Existing author, new name
                names.push(new_name.clone());
            } else {
                return;
            }
        }
        // Always keep the existing name first
        self.str_to_id.entry(new_name).or_insert(id);
    }
}

impl Debug for Authors {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.id_to_str.fmt(f)
    }
}

impl Serialize for Authors {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer
    {
        self.id_to_str.serialize(serializer)
    }
}
