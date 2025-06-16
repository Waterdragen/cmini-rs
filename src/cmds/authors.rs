use std::cmp::Ordering;
use itertools::{EitherOrBoth, Itertools};
use crate::util::{BoundedResponse, Commandable, Message};
use crate::util::authors::AUTHORS;

pub struct Command;

impl Commandable for Command {
    fn exec(&self, _: &Message) -> String {
        let mut resp = BoundedResponse::from("Layout Creators:\n```\n".to_owned())
            .reserve(25);
        let authors = AUTHORS.read().unwrap();
        let mut author_names = authors.iter().collect::<Vec<_>>();
        author_names.sort_unstable_by(|a, b| cmp_ascii_ignore_case(a, b));

        let mut count = 0usize;
        let result: Result<(), ()> = author_names
            .iter()
            .try_for_each(|name| {
            resp.push_str(name)?;
            resp.push('\n')?;
            count += 1;
            Ok(())
        });

        let mut resp = resp.finish();
        if result.is_err() {
            // Push reserved characters
            resp.push_str("... (");  // push 5 characters
            resp.push_str(&(author_names.len() - count).to_string());  // reserve 10 digits
            resp.push_str(" more)\n```")  // push 10 characters
        } else {
            resp.push_str("\n```");
        }
        resp
    }

    fn usage<'a>(&self) -> &'a str {
        "authors"
    }

    fn desc<'a>(&self) -> &'a str {
        "list authors of akl layouts"
    }

    fn cmini_channel_only(&self) -> bool {
        true
    }
}

fn cmp_ascii_ignore_case(a: &str, b: &str) -> Ordering {
    for ab in a.chars()
        .flat_map(char::to_lowercase)
        .zip_longest(b.chars().flat_map(char::to_lowercase)) {
        match ab {
            EitherOrBoth::Left(_) => return Ordering::Greater,
            EitherOrBoth::Right(_) => return Ordering::Less,
            EitherOrBoth::Both(a, b) => {
                match a.cmp(&b) {
                    Ordering::Equal => continue,
                    ordering => {
                        return ordering;
                    }
                }
            }
        }
    }
    Ordering::Equal
}
