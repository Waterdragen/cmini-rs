use crate::util::corpora::{self, get_user_corpus_upper};
use crate::util::parser::get_args;
use crate::{Commandable, Message};
use cmini_core::KeyPat;

pub struct Command;

impl Commandable for Command {
    fn exec(&self, msg: &Message) -> String {
        let id = msg.id;
        let ngrams = get_args(msg.arg)
            .iter().map(|ngrams| {
                ngrams.chars().map(KeyPat).collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();
        let Some(ngram_len) = ngrams.first().map(|first| first.len()) else {
            return self.help();
        };
        if ngrams.len() > 20 {
            return "Please provide no more than 20 ngrams".to_owned();
        }
        if !ngrams.iter().all(|ngrams| ngrams.len() == ngram_len) {
            return "All ngrams must be the same length".to_owned();
        }
        let monograms = corpora::ngrams::<1>(id);
        let bigrams = corpora::ngrams::<2>(id);
        let trigrams = corpora::ngrams::<3>(id);
        let (dyn_corpus, total) = match ngram_len {
            1 => (monograms.dyn_len_iter(), monograms.sum as f64),
            2 => (bigrams.dyn_len_iter(), bigrams.sum as f64),
            3 => (trigrams.dyn_len_iter(), trigrams.sum as f64),
            _ => return "Please provide ngrams between 1-3 chars".to_owned(),
        };
        let corpus_name = get_user_corpus_upper(id);
        let mut output = format!("```\n{corpus_name}\n");
        let mut subtotal = 0.0;

        for ngram in ngrams.iter() {
            let freq = dyn_corpus.clone_iter()
                .filter_map(|(keys, freq)| (ngram == keys).then_some(freq))
                .sum::<u64>();
            let freq_percent = freq as f64 / total * 100.0;
            let mut ngram_str = "".to_owned();
            for key in ngram {
                ngram_str.push(key.0);
            }
            output.push_str(&format!("{ngram_str}: {freq_percent:.2}%\n"));
            subtotal += freq_percent;
        }
        if subtotal == 0.0 {
            return format!("`{}` not found in corpus `{corpus_name}`", msg.arg);
        }
        output.push_str(&format!("Total: {subtotal:.2}%\n```"));
        output
    }

    fn usage<'a>(&self) -> &'a str {
        "freq [ngrams ...]"
    }

    fn desc<'a>(&self) -> &'a str {
        "see the frequency of ngrams"
    }
}