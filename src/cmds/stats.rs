use std::cmp::Reverse;
use fxhash::FxHashMap;
use crate::Commandable;
use crate::message::Message;
use crate::util::corpora::CORPORA_PREFS;
use crate::util::memory::{AUTHORS, LAYOUTS, LIKES};

pub struct Command;

impl Commandable for Command {
    fn exec(&self, _: &Message) -> String {
        let authors_count = {
            let authors = AUTHORS.read();
            authors.len()
        };
        let likes = LIKES.read();
        let mut like_counter = likes.iter()
            .map(|(name, users)| (name, users.len()))
            .collect::<Vec<_>>();
        like_counter.sort_unstable_by_key(|(_, likes)| Reverse(*likes));

        let corpora_prefs = CORPORA_PREFS.read();
        let mut corpora_counter = FxHashMap::<&str, usize>::default();
        for name in corpora_prefs.values() {
            *corpora_counter.entry(name).or_insert(0) += 1;
        }
        let mut corpora_counter = Vec::from_iter(corpora_counter);
        corpora_counter.sort_unstable_by_key(|(_, users)| Reverse(*users));

        let layouts_count = {
            let layouts = LAYOUTS.read();
            layouts.len()
        };

        let mut output = format!("```\n\
                                     --- CMINI STATS --- \n\
                                     Layouts: {layouts_count}\n\
                                     Authors: {authors_count}\n\
                                     \n\
                                     Most liked layouts:\n");

        for (name, likes) in like_counter.iter().take(30) {
            output.push_str(&format!("    {name:<15} ({likes} likes)\n"));
        }
        output.push_str("\nTop Corpora:\n");
        for (name, users) in corpora_counter.iter().take(5) {
            output.push_str(&format!("    {name:<15} ({users} users)\n"));
        }
        output.push_str("```");
        output
    }

    fn usage<'a>(&self) -> &'a str {
        "stats"
    }

    fn desc<'a>(&self) -> &'a str {
        "see the global stats"
    }
}