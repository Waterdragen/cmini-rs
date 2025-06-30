use std::cmp::Reverse;
use itertools::Itertools;
use crate::message::Message;
use crate::prelude::Commandable;
use crate::util::parser::get_layout;
use rphonetic::{Encoder, MatchRatingApproach};
use crate::util::memory::PHONETIC_CODE_FREQS;

pub struct Command;

impl Commandable for Command {
    fn exec(&self, msg: &Message) -> String {
        let (_, matrix) = get_layout(msg.arg);
        if matrix.is_empty() {
            return self.help();
        }

        let layout = matrix.split('\n')
            .map(|s| {
                let s = s.chars()
                    .filter(|&c| c != ' ').join("");
                let indices = s.char_indices().map(|(idx, _)| idx).collect_vec();
                (s, indices)
            })
            .collect_vec();

        let match_rating = MatchRatingApproach;
        let mut res = layout.iter()
            .flat_map(|(row, indices)| {
                indices.iter().zip(indices.iter().skip(4))
                    .map(|(&start, &end)| {
                        let substr = &row[start..end];
                        match_rating.encode(substr)
                    })
            })
            .filter_map(|code| {
                PHONETIC_CODE_FREQS.get(&code)
            })
            .collect_vec();

        res.sort_unstable_by_key(|(freq, _)| Reverse(*freq));
        let mut output = "Here are a few names I could come up with:\n```\n".to_owned();
        for (_, name) in res.iter().take(10) {
            output.push_str(name);
            output.push('\n');
        }
        output.push_str("```");
        output
    }

    fn usage<'a>(&self) -> &'a str {
        "names <``\u{200b}`keys`\u{200b}``>"
    }

    fn desc<'a>(&self) -> &'a str {
        "get name suggestions for a layout"
    }
}