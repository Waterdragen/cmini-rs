use fxhash::FxHashMap;
use strsim::jaro_winkler;
use serenity::model::channel::Message;
use crate::util::jsons::{get_map_u64_vec_str, write_map_u64_vec_str};

fn get_authors() -> FxHashMap<u64, Vec<String>> {
    get_map_u64_vec_str("./authors.json")
}

pub fn update(msg: &Message) {
    let user = &msg.author.name;
    let id = msg.author.id.as_u64();
    let mut changed = true;

    let mut authors = get_authors();
    match authors.get_mut(id) {
        Some(names) => if !names.contains(user) {
            names.push(user.to_string());
        } else { changed = false },
        None => { authors.insert(*id, vec![user.to_string()]); },
    }
    if changed {
        write_map_u64_vec_str("./authors.json", &authors);
    }
}

pub fn get_id(name: &str) -> u64 {
    let mut match_id = 0u64;
    let mut max_score = 0.0;
    let authors = get_authors();

    for (id, author_names) in authors.iter() {
        for author_name in author_names {
            let score = jaro_winkler(name, author_name);
            if score > max_score {
                max_score = score;
                match_id = *id;
            }
        }
    }
    match_id
}

pub fn get_name(id: u64) -> String {
    let authors = get_authors();
    match authors.get(&id) {
        Some(names) => names[0].to_string(),
        None => String::from("unknown"),
    }
}
