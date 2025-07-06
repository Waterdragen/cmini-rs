use crate::message::Message;
use crate::util::corpora::get_user_corpus_upper;
use crate::util::memory::LAYOUTS;
use crate::util::parser::split_words;
use crate::util::{corpora, layout};
use crate::Commandable;

pub struct Command;

impl Commandable for Command {
    fn exec(&self, msg: &Message) -> String {
        let [new_ll, old_ll] = split_words(msg.arg);
        if new_ll.is_empty() {
            return self.help();
        }
        if old_ll.is_empty() {
            return "Error: missing second layout name".to_owned();
        }
        let new_ll = &*LAYOUTS.find(new_ll);
        let old_ll = &*LAYOUTS.find(old_ll);
        let monograms = corpora::ngrams::<1>(msg.id);
        let trigrams = corpora::ngrams::<3>(msg.id);

        let old_stats = old_ll.trigram_stats(&trigrams);
        let mut new_stats = new_ll.trigram_stats(&trigrams);
        let old_use = old_ll.fingers_usage(&monograms);
        let mut new_use = new_ll.fingers_usage(&monograms);

        new_stats.values_mut().zip(old_stats.values())
            .for_each(|(new, old)| *new -= old);
        new_use.values_mut().zip(old_use.values())
            .for_each(|(new, old)| *new -= old);

        let corpus_name = get_user_corpus_upper(msg.id);
        let (new_name, old_name) = (&new_ll.name, &old_ll.name);
        let matrix_str = new_ll.get_common_matrix(old_ll);
        let stats_str = layout::get_stats_str(&new_stats, &new_use);

        format!(
            "```\n\
             {new_name}(new) - {old_name}(old)\n\
             {matrix_str}\
             \n\
             {corpus_name}:\n\
             {stats_str}\n\
             ```"
        )
    }

    fn usage<'a>(&self) -> &'a str {
        "compare <new_layout> <old_layout>"
    }

    fn desc<'a>(&self) -> &'a str {
        "compare the stats of two layouts"
    }
}