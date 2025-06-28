use crate::core::{FxIndexMap, Key, Position};
use crate::message::Message;
use crate::prelude::Commandable;
use crate::util::memory::LAYOUTS;

pub struct Command;

impl Commandable for Command {
    fn exec(&self, msg: &Message) -> String {
        let arg = msg.arg.to_lowercase();
        let mut chars = arg.chars();
        let Some(letter) = chars.next() else {
            return self.help();
        };
        if chars.next().is_some() {
            return "Error: please enter one letter only".to_owned();
        }
        let mut counter = FxIndexMap::<Key, usize>::default();
        let layouts = LAYOUTS.read();
        for ll in layouts.values() {
            let Some(&Position { finger, ..}) = ll.keys.get(&letter) else { continue };
            ll.keys.iter()
                .filter(|(_, pos)| pos.finger == finger)
                .for_each(|(&key, _)| {
                    *counter.entry(key).or_insert(0) += 1;
                });
        }
        counter.swap_remove(&letter);
        counter.sort_unstable_by(|_, count0, _, count1| count1.cmp(count0));
        let letter = letter.to_uppercase().to_string();
        let mut output = format!("Most common pairings with `{letter}`:\n```\n");
        for (pair_letter, count) in counter.iter().take(15) {
            let pair_letter = pair_letter.to_uppercase().to_string();
            output.push_str(&format!("{pair_letter}{letter} {count:>3}"));
            output.push('\n');
        }
        output.push_str("```");
        output
    }

    fn usage<'a>(&self) -> &'a str {
        "pairings <letter>"
    }

    fn desc<'a>(&self) -> &'a str {
        "find the most common pairings of a letter on cmini layouts"
    }
}