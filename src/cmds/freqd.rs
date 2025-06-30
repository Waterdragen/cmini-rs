use crate::core::{ClonableIterator, KeyPat};
use crate::util::corpora;
use crate::util::corpora::get_user_corpus;
use crate::util::parser::get_args;
use crate::{Commandable, Message};

pub struct Command;

impl Commandable for Command {
    fn exec(&self, msg: &Message) -> String {
        let Ok(query) = get_args(msg.arg)
            .iter().map(|ngram| {
                let mut chars = ngram.chars();
                match (chars.next(), chars.next(), chars.next()) {
                    (Some(key0), Some(key1), None) => Ok([KeyPat(key0), KeyPat(key1)]),
                    _ => Err(()),
                }
            })
            .collect::<Result<Vec<_>, _>>() else {
            return "Error: please provide bigrams".to_owned();
        };
        match query.len() {
            0 => return self.help(),
            7.. => return "Error: please provide no more than 6 bigrams".to_owned(),
            _ => {}
        }
        let bigrams = corpora::ngrams::<2>(msg.id);
        let trigrams = corpora::ngrams::<3>(msg.id);
        let totals = [bigrams.sum as f64, trigrams.sum as f64];
        const NGRAM_TYPES: usize = 2;
        let bigrams_iter = bigrams.iter()
            .map(|([key_first, key_last], freq)| (key_first, key_last, freq, 0usize));
        let trigrams_iter = trigrams.iter()
            .map(|([key_first, .., key_last], freq)| (key_first, key_last, freq, 1));
        let ngrams_iter = bigrams_iter.chain(trigrams_iter);

        let mut subtotals = [0.0; NGRAM_TYPES];
        let sep_line = "-".repeat(11 + 9 * NGRAM_TYPES) + "\n";
        let corpus_name = get_user_corpus(msg.id).to_ascii_uppercase();
        let mut output = format!("```\n\
                                         {corpus_name}\n{sep_line} bigrams ");
        for i in 0..NGRAM_TYPES {
            let gram_type_repr = format!("x{}x", "_".repeat(i));
            output.push_str(&format!("| {gram_type_repr:^7} "));
        }
        output.push('\n');
        output.push_str(&sep_line);

        query.iter().map(|&bigram_pat| {
            let mut counter = FreqDCounter::<NGRAM_TYPES>::new();
            for (&key_first, &key_last, &freq, idx) in ngrams_iter.clone_iter() {
                if bigram_pat == [key_first, key_last] {
                    *counter.forward_mut(idx) += freq;
                }
                if bigram_pat == [key_last, key_first] {
                    *counter.backward_mut(idx) += freq;
                }
            }
            (bigram_pat, counter)
        })
            .for_each(|([pat0, pat1],  counter)| {
                let (key0, key1) = (pat0.0, pat1.0);
                let mut sum_str = format!(" {key0}{key1} + {key1}{key0} ");
                let mut forward_str = format!("   {key0}{key1}    ");
                let mut backward_str = format!("   {key1}{key1}    ");
                for idx in 0..NGRAM_TYPES {
                    let total = totals[idx];
                    let sum = counter.sum(idx) as f64 * 100.0 / total;
                    let forward = counter.forward(idx) as f64 * 100.0 / total;
                    let backward = counter.backward(idx) as f64 * 100.0 / total;
                    sum_str.push_str(&format!("| {sum:>6.2}% "));
                    forward_str.push_str(&format!("| {forward:>6.2}% "));
                    backward_str.push_str(&format!("| {backward:>6.2}% "));
                    subtotals[idx] += sum;
                }
                output.push_str(&sum_str);
                output.push('\n');
                output.push_str(&forward_str);
                output.push('\n');
                output.push_str(&backward_str);
                output.push('\n');
                output.push_str(&sep_line);
            });

        output.push_str("  total  ");
        for subtotal in subtotals {
            output.push_str(&format!("| {subtotal:>6.2}% "));
        }
        output.push('\n');

        output.push_str("```");
        output
    }

    fn usage<'a>(&self) -> &'a str {
        "freqd <bigrams ...>"
    }

    fn desc<'a>(&self) -> &'a str {
        "view the frequencies of dynamic bigram combinations"
    }
}

#[repr(transparent)]
struct FreqDCounter<const NGRAM_TYPES: usize>([[u64; 2]; NGRAM_TYPES]);

impl<const N: usize> FreqDCounter<N> {
    const fn new() -> Self {
        Self([[0; 2]; N])
    }
    fn forward(&self, idx: usize) -> u64 {
        self.0[idx][0]
    }
    fn forward_mut(&mut self, idx: usize) -> &mut u64 {
        &mut self.0[idx][0]
    }
    fn backward(&self, idx: usize) -> u64 {
        self.0[idx][1]
    }
    fn backward_mut(&mut self, idx: usize) -> &mut u64 {
        &mut self.0[idx][1]
    }
    fn sum(&self, idx: usize) -> u64 {
        self.forward(idx) + self.backward(idx)
    }
}