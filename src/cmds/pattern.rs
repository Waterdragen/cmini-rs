use crate::message::Message;
use crate::util::corpora;
use crate::util::corpora::NGRAMS;
use crate::util::memory::LAYOUTS;
use crate::util::parser::split_words;
use crate::Commandable;
use cmini_core::finger_alias::{ANY_FINGER, NO_FINGER};
use cmini_core::{Finger, FingerCombo};
use itertools::Itertools;

pub struct Command;

impl Commandable for Command {
    fn exec(&self, msg: &Message) -> String {
        let [name, query] = split_words(msg.arg);
        if name.is_empty() {
            return self.help();
        }
        let query = match query.split_whitespace()
            .map(str::to_ascii_uppercase)
            .map(|s| if s == "_" { Ok(ANY_FINGER) } else {
                s.split('|')
                    .try_fold(NO_FINGER, |finger_union, finger_str| {
                        Ok(finger_union | Finger::try_from_str(finger_str)?)
                    })
                    .map_err(|_x: ()| s)
            })
            .collect::<Result<Vec<_>, _>>() {
            Ok(query) => query,
            Err(s) => return format!("Error: invalid finger {s}"),
        };
        if query.is_empty() {
            return format!("Error: please provide finger values (e.g., LI, _, LI|RR)\n{}", self.help());
        }

        let query_len = query.len();
        if query_len > NGRAMS.len() {
            return "Error: please provide no more than 3 finger values".to_owned();
        }
        let ll = LAYOUTS.find(name);
        let trigrams = corpora::ngrams::<3>(msg.id);
        let total = trigrams.sum as f64;

        let res = trigrams.iter()
            .filter(|(gram, _)| {
                let Some(finger_combo) = FingerCombo::from_ngrams(&ll.keys, gram) else { return false };
                finger_combo.inner.windows(query_len)
                    .any(|fingers| fingers.iter().zip(query.iter())
                        .all(|(&finger, &finger_union)| finger_union.contains(finger)))
            })
            .take(15)  // trigrams are iterated in descending order by freq
            .map(|(gram, freq)| (gram, *freq as f64 / total * 100.0))
            .collect::<Vec<_>>();
        let found = res.len();
        let name = &ll.name;
        let finger_pattern = query.iter()
            .map(|finger_union| {
                finger_union.iter()
                    .map(Into::<&str>::into).join("|")
            })
            .join("-");
        let subtotal_percent = res.iter()
            .map(|(_, freq_percent)| freq_percent).sum::<f64>();

        let mut output = format!("```\n\
                                         Top {found} {name} patterns for {finger_pattern}:\n");
        for (gram, freq_percent) in res.iter() {
            let gram = gram.iter().join("");
            output.push_str(&format!("{gram:<6} {freq_percent:.3}%\n"));
        }
        output.push_str(&format!("Total {subtotal_percent:.3}%"));
        output.push_str("```");

        output
    }

    fn usage<'a>(&self) -> &'a str {
        "pattern <layout_name> <finger_string>"
    }

    fn desc<'a>(&self) -> &'a str {
        "see the most common pattern for a given finger string (e.g., RM LI|RR or LP _ LM)"
    }
}