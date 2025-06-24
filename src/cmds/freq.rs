use crate::util::corpora::get_user_corpus;
use crate::util::{corpora, Commandable, Message};
use regex::Regex;
use crate::util::parser::get_pattern;

pub struct Command;

impl Commandable for Command {
    fn exec(&self, msg: &Message) -> String {
        let id = msg.id;
        let pattern = get_pattern(msg.arg);
        let ngrams = pattern.split_whitespace().collect::<Vec<_>>();
        let ngram_len = match ngrams.first() {
            None => return self.help(),
            Some(ngram) => ngram.chars().count(),
        };
        let monograms = corpora::ngrams::<1>(id);
        let bigrams = corpora::ngrams::<2>(id);
        let trigrams = corpora::ngrams::<3>(id);
        let (dyn_corpus, total) = match ngram_len {
            1 => (monograms.dyn_len_iter(), monograms.sum as f64),
            2 => (bigrams.dyn_len_iter(), bigrams.sum as f64),
            3 => (trigrams.dyn_len_iter(), trigrams.sum as f64),
            _ => return "Please provide ngrams between 1-3 chars".to_owned(),
        };
        if ngrams.len() > 20 {
            return "Please provide no more than 20 ngrams".to_owned();
        }
        if !ngrams.iter().all(|ngram| ngram.chars().count() == ngram_len) {
            return "All ngrams must be the same length".to_owned();
        }
        let corpus_name = get_user_corpus(id).to_ascii_uppercase();
        let mut output = format!("```{corpus_name}");
        let mut sub_total = 0.0;

        for ngram in ngrams.iter() {
            let re = match Regex::new(ngram) {
                Ok(re) => re,
                Err(_) => return format!("Error: invalid ngram {ngram}"),
            };
            let corpus = dyn_corpus.clone_iter();
            let freq = corpus.filter_map(|(gram, freq)| {
                let s = gram.iter().collect::<String>();
                re.is_match(&s).then_some(freq)
            })
                .sum::<u64>();
            let freq_percent = freq as f64 / total * 100.0;
            output.push_str(&format!("{ngram}: {freq_percent:.2}%\n"));
            sub_total += freq_percent;
        }
        if sub_total == 0.0 {
            return format!("`{}` not found in corpus `{corpus_name}`", msg.arg);
        }
        output.push_str(&format!("Total: {sub_total:.2}%\n```"));
        output
    }

    fn usage<'a>(&self) -> &'a str {
        "freq [ngrams ...]"
    }

    fn desc<'a>(&self) -> &'a str {
        "see the frequency of ngrams"
    }
}