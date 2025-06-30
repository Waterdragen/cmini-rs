use itertools::Itertools;
use crate::core::{Finger, Position, Col, Key};
use crate::message::Message;
use crate::util::memory::LAYOUTS;
use crate::util::parser::split_words;
use crate::core::Finger::*;
use crate::Commandable;

const FINGER_MAP: [&[&[Finger]]; 4] = [
    //  q      w          e          r          t          y          u          i          o          p          [      ]      \
    &[&[LP], &[LR, LP], &[LM, LR], &[LI, LM], &[LI, LM], &[RI, RM], &[RI, RM], &[RM, RR], &[RR, RM], &[RP, RR], &[RP], &[RP], &[RP]],
    //  a      s          d          f          g      h      j          k          l      ;      '
    &[&[LP], &[LR, LM], &[LM, LI], &[LI, LM], &[LI], &[RI], &[RI, RM], &[RM, RR], &[RR], &[RP], &[RP]],
    //       z      x          c          v          b          n      m          ,          .      /
    &[&[], &[LR], &[LM, LR], &[LM, LI], &[LI, LM], &[LI, RI], &[RI], &[RI, RM], &[RM, RR], &[RR], &[RP]],
    &[&[LT], &[LT], &[LT], &[LT], &[LT, RT], &[LT, RT], &[RT], &[RT], &[RT], &[RT], &[RT]],
];

pub struct Command;

impl Commandable for Command {
    fn exec(&self, msg: &Message) -> String {
        let [name, word] = split_words(msg.arg);
        let raw_word = word.to_lowercase();
        if name.is_empty() || raw_word.is_empty() {
            return self.help();
        }
        let mut ll = LAYOUTS.find(name).clone();
        let word = raw_word.chars().collect::<Vec<_>>();
        if let Some(c) = word.iter().find(|c| !ll.keys.contains_key(c)) {
            return format!("Error: `{}` doesn't have the key `{c}`", ll.name);
        }
        if word.len() > 15 {
            return format!("Error: Max word length is 15, got {}", word.len());
        }
        ll.keys.values_mut()
            .for_each(|pos| {
                if pos.row == 2 {
                    pos.col += 1;
                }
            });
        let finger_options = match word.iter()
            .map(|c| {
                let Position { row, col, ..} = ll.keys[c];
                let finger_row = FINGER_MAP[usize::from(row)];
                finger_row.get(usize::from(col))
                    .ok_or((*c, finger_row.len(), col))
            })
            .collect::<Result<Vec<_>, _>>() {
            Ok(finger_options) => finger_options,
            Err((c, expect_len, got_col)) => {
                return format!("Error: key `{c}` is at column {got_col}, expected at most {expect_len} columns");
            }
        };

        let columns = word.iter().map(|c| ll.keys[c].col).collect_vec();
        let (alts, (sfb_score, cross_score)) = get_alt(&word, &finger_options, &columns);
        let sfb_percent = sfb_score as f64 / word.len() as f64 * 100.0;
        let name = &ll.name;
        let suggestion = alts.iter().map(|&&finger| Into::<&str>::into(finger)).join(" ");
        let chars_pretty = word.iter().map(|c| format!("{c:3}")).join("");
        let traditional = word.iter().map(|c| Into::<&str>::into(ll.keys[c].finger)).join(" ");
        format!("```\n\
                 Alt fingering suggestion for '{raw_word}' ({name})\n\
                 {suggestion}\n\
                 {chars_pretty}\n\
                 {traditional} (traditional)\n\
                 SFB: {sfb_score} / {sfb_percent}%\n\
                 Crossovers: {cross_score}\n\
                 ```")
    }

    fn usage<'a>(&self) -> &'a str {
        "alt <layout_name> <word>"
    }

    fn desc<'a>(&self) -> &'a str {
        "view an alt fingering suggestion for a word"
    }
}

fn is_crossover(finger_pair: &[&Finger], col_pair: &[Col]) -> bool {
    col_pair[1] > col_pair[0] && finger_pair[1].as_u8() < finger_pair[0].as_u8() ||
        col_pair[1] < col_pair[0] && finger_pair[1].as_u8() > finger_pair[0].as_u8()
}
fn is_sfb(finger_pair: &[&Finger], key_pair: &[Key]) -> bool {
    finger_pair[0] == finger_pair[1] && key_pair[0] != key_pair[1]
}

fn get_alt<'a>(word: &[char], finger_options: &[&&'a [Finger]], columns: &[Col]) -> (Vec<&'a Finger>, (usize, usize)) {
    finger_options.iter()
        .map(|finger_option| **finger_option)
        .multi_cartesian_product()
        .map(|finger_option| {
            let sfb_score = finger_option.windows(2).zip(word.windows(2))
                .map(|(finger_pair, key_pair)| {
                    is_sfb(finger_pair, key_pair) as usize
                })
                .sum::<usize>();
            let cross_score = finger_option.windows(2).zip(columns.windows(2))
                .map(|(finger_pair, col_pair)| {
                    is_crossover(finger_pair, col_pair) as usize
                })
                .sum::<usize>();
            (finger_option, (sfb_score, cross_score))
        })
        .min_by_key(|(_, score)| *score)
        .unwrap()  // raw_word is always non-empty, finger_options is mapped from raw_word
}

