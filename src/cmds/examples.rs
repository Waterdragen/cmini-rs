use crate::util::corpora::get_user_corpus;
use crate::util::parser::get_pattern;
use crate::util::corpora;
use crate::{Commandable, Message};
use regex::Regex;

const MOST_COMMON: usize = 10;

pub struct Command;

impl Commandable for Command {
    fn exec(&self, msg: &Message) -> String {
        let arg = msg.arg;
        let pattern = get_pattern(arg);
        let re_pattern = match Regex::new(&pattern) {
            Ok(re) => re,
            Err(_) => return format!("Invalid pattern {}", arg).to_owned(),
        };
        let words = corpora::words(msg.id);
        let total = words.sum as f64;
        let mut sub_total = 0.0;
        let corpus_name = get_user_corpus(msg.id).to_ascii_uppercase();
        let mut examples = "".to_owned();
        words.iter()
            .filter_map(|(word, freq)| {
                let s = word.iter().collect::<String>().to_lowercase();
                re_pattern.is_match(&s).then_some((s, freq))
            })
            .enumerate()
            .for_each(|(index, (word, freq))| {
                if index < MOST_COMMON {
                    let word = word.replace('`', "​`");
                    let freq_str = format!("({freq})");
                    examples.push_str(&format!("{word:<15} {freq_str:>6}\n"));
                }
                sub_total += *freq as f64;
            });
        if sub_total == 0.0 {
            return format!("Error: {arg} does not appear anywhere in this corpus").to_owned();
        }
        let percent = sub_total / total * 100.0;
        format!("Examples of {arg} in {corpus_name}:\n\
                 ```\n\
                 {sub_total} / {total} words ({percent:.3}%)\n\
                 \n\
                 {examples}\
                 ```\n\
                 ")
    }

    fn usage<'a>(&self) -> &'a str {
        "examples <ngrams>"
    }

    fn desc<'a>(&self) -> &'a str {
        "find common examples of an ngram"
    }
}