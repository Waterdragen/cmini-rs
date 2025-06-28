use crate::core::Position;
use crate::util::memory::LAYOUTS;
use crate::util::parser::get_pattern;
use crate::{BoundedResponseVec, Commandable, Message};
use itertools::Itertools;
use rand::prelude::{SeedableRng, SliceRandom, StdRng};
use regex::{Error as RegexError, Regex};

pub struct Command;

impl Commandable for Command {
    fn exec(&self, msg: &Message) -> String {
        if msg.arg.is_empty() {
            return self.help();
        }

        let layouts = &*LAYOUTS.read();
        let Ok(re) = homerow_regex(msg.arg) else {
            return format!("Error: invalid regex `{}`", msg.arg);
        };

        let mut res = layouts.values().map(|ll| {
            let mut keys = vec![' '; 12];
            ll.keys.iter()
                .for_each(|(key, Position { row, col, .. })| {
                    if *row != 1 {
                        return;
                    }
                    let col = usize::from(*col);
                    while keys.len() <= col {
                        keys.resize(keys.len() + 5, ' ');
                    }
                    keys[col] = *key;
                });
            (ll.name.to_owned(), keys.iter().join(""))
        })
            .filter_map(homerow_filter_fn(&re, msg.arg))
            .collect::<Vec<String>>();
        let mut rng = StdRng::from_entropy();
        res.shuffle(&mut rng);
        let found_count = res.len();

        let mut found_layouts = if msg.is_private() {
            let mut found_layouts = BoundedResponseVec::new().reserve(60);
            let _: Result<(), ()> = res.into_iter().try_for_each(|mut s| {
                s.push('\n');
                found_layouts.push(s)?;
                Ok(())
            });  // Only try push to resp, returned error does not matter here
            found_layouts.finish()
        } else {
            res.into_iter().take(20)
                .map(|mut s| {
                    s.push('\n');
                    s
                })
                .collect()
        };
        found_layouts.sort();
        let found_layouts_str = found_layouts.join("");
        let display_count = found_layouts.len();
        let all_or_n = match display_count == found_count {
            true => "all".to_owned(),
            false => display_count.to_string(),
        };
        format!(
            "I found {found_count} matches, here are {all_or_n} of them:\n```\n\
             {found_layouts_str}\n\
             ```"
        )
    }

    fn usage<'a>(&self) -> &'a str {
        r#"homerow <key_set | "key_sequence">"#
    }

    fn desc<'a>(&self) -> &'a str {
        "search for layouts with a particular string in homerow"
    }
}

pub(super) fn homerow_regex(search_str: &str) -> Result<Option<Regex>, RegexError> {
    if search_str.starts_with('"') && search_str.ends_with('"') {
        let byte_len = search_str.len();
        let pat_str = get_pattern(&search_str[1..byte_len - 1]);  // Always valid character boundary: starts and ends with 1-byte char ('"')
        Ok(Some(Regex::new(&pat_str)?))
    } else {
        Ok(None)
    }
}

pub(super) fn homerow_filter_fn<'a>(re: &'a Option<Regex>, search_str: &'a str) -> Box<dyn Fn((String, String)) -> Option<String> + 'a> {
    match re {
        None => Box::new(|(name, homerow): (String, String)| {
            search_str.chars()
                .all(|c| homerow.trim_end().contains(c))
                .then_some(name)
        }),
        Some(re) => Box::new(|(name, homerow): (String, String)| {
            let homerow = homerow.trim_end();
            (re.is_match(homerow) ||
                re.is_match(&homerow.chars().rev().collect::<String>()))
                .then_some(name)
        }),
    }
}