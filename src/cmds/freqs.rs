use crate::core::KeyPat;
use crate::message::Message;
use crate::util::corpora;
use crate::util::corpora::get_user_corpus_upper;
use crate::util::parser::get_args;
use crate::Commandable;
use itertools::Itertools;

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
            let mut forward_freq = 0;
            let mut backward_freq = 0;
            dyn_corpus.clone_iter()
                .for_each(|(keys, freq)| {
                    if ngram == keys {
                        forward_freq += freq;
                    }
                    if ngram.iter().zip(keys.iter().rev())
                        .all(|(&pat, &key)| pat == key) {
                        backward_freq += freq;
                    }
                });
            let mut s = "".to_owned();
            for key in ngram {
                s.push(key.0);
            }
            let s_rev = s.chars().rev().join("");
            let forward_percent = forward_freq as f64 / total * 100.0;
            let backward_percent = backward_freq as f64 / total * 100.0;
            let sum_percent = forward_percent + backward_percent;

            output.push_str(&format!("{s} + {s_rev}: {sum_percent:.2}%\n  \
                                              {s}: {forward_percent:.2}%\n  \
                                              {s_rev}: {backward_percent:.2}%\n"));
            subtotal += sum_percent;
        }
        if subtotal == 0.0 {
            return format!("`{}` not found in corpus `{corpus_name}`", msg.arg);
        }
        output.push_str(&format!("Total: {subtotal:.2}%\n```"));
        output
    }

    fn usage<'a>(&self) -> &'a str {
        "freqs <ngrams ...>"
    }

    fn desc<'a>(&self) -> &'a str {
        "see the frequencies of ngrams, including backwards"
    }
}