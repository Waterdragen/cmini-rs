use crate::core::{Commandable, FingerUnion};
use crate::message::Message;
use crate::prelude::BoundedResponse;
use crate::util::cache::CACHED_STATS;
use crate::core::finger_alias::FINGER_NAMES;
use crate::util::parser::{get_kwargs, KwargType as KT};
use fxhash::FxHashMap;
use once_cell::sync::Lazy;
use rand::prelude::{StdRng, SeedableRng, SliceRandom};
use std::iter::IntoIterator;
use strsim::jaro_winkler;

static KWARGS: Lazy<FxHashMap<String, KT>> = Lazy::new(|| [
    ("lp", KT::Bool),
    ("lr", KT::Bool),
    ("lm", KT::Bool),
    ("li", KT::Bool),
    ("lt", KT::Bool),
    ("rt", KT::Bool),
    ("ri", KT::Bool),
    ("rm", KT::Bool),
    ("rr", KT::Bool),
    ("rp", KT::Bool),
    ("pinky", KT::Bool),
    ("ring", KT::Bool),
    ("middle", KT::Bool),
    ("index", KT::Bool),
    ("thumb", KT::Bool),
    ("lh", KT::Bool),
    ("rh", KT::Bool),
    ("name", KT::Str),
    ("vowel", KT::Str),
].into_iter().map(|(s, kt)| (s.to_owned(), kt)).collect());

pub struct Command;

impl Commandable for Command {
    fn exec(&self, msg: &Message) -> String {
        let kwargs = match get_kwargs(msg.arg, &KWARGS) {
            Ok(kwargs) => kwargs,
            Err(err) => return err.to_string(),
        };
        let mut finger = None::<FingerUnion>;
        for (finger_name, &finger_union) in FINGER_NAMES.iter() {
            if kwargs[finger_name].unwrap_bool() {
                match finger.as_mut() {
                    None => finger = Some(finger_union),
                    Some(finger) => *finger = *finger | finger_union,
                }
            }
        }
        let filter_name = kwargs["name"].unwrap_str();
        let filter_vowel = kwargs["vowel"].unwrap_str();
        let sfb = &kwargs.arg;
        match (sfb.is_empty(), filter_name) {
            (true, None) => return self.help(),
            (true, Some(filter_name)) => {
                let cached_stats = CACHED_STATS.read();
                let mut resp = BoundedResponse::from("```\n".to_owned()).reserve(100);
                let mut total = 0;
                let mut sub_total = 0;
                cached_stats.keys()
                    .filter(|name| search_is_similar(filter_name, name))
                    .for_each(|name| {
                        total += 1;
                        if resp.push_line(name).is_ok() {
                            sub_total += 1;
                        }
                    });
                let all_or_n = if total == sub_total { "all".to_owned() } else { sub_total.to_string() };
                return format!("I found {total} matches, here are {all_or_n} of them:\n{}```", resp.finish());
            }
            _ => {}
        }

        let cached_stats = CACHED_STATS.read();
        let mut result = cached_stats.iter().filter_map(|(name, stats)| {
            stats.is_sfb(sfb, finger).then_some(())?;
            if let Some(filter_name) = filter_name {
                search_is_similar(name, filter_name).then_some(())?;
            }
            if let Some(filter_vowel) = filter_vowel {
                stats.contains_vowel_in_one_hand(filter_vowel).then_some(())?;
            }
            Some(name)
        })
            .collect::<Vec<_>>();

        let mut rng = StdRng::from_entropy();
        result.shuffle(&mut rng);

        let found = result.len();
        let subtotal = if !msg.is_private() { 20usize } else {
            result.iter()
                .scan(0, |acc, s| {
                    *acc += s.len() + 1;  // String.len() always >= String.chars().count()
                    Some(*acc)
                })
                .position(|len| len >= 1900)
                .unwrap_or(found)
        };
        result.truncate(subtotal);
        result.sort_unstable();
        let all_or_n = match found == subtotal {
            true => "all".to_owned(),
            false => subtotal.to_string(),
        };

        let mut output = format!("I found {found} matches, here are {all_or_n} of them:\n```\n");
        for name in result.iter() {
            output.push_str(name);
            output.push('\n');
        }
        output.push_str("```");
        output
    }

    fn usage<'a>(&self) -> &'a str {
        "search <sfb_keys> [--vowel <letters>] [--fingers <finger_names...>]\n\
         Support fingers: \n\
         li, lm, lr, lp, ri, rm, rr, rp, lt, rt, tb, index, middle, ring, pinky, thumb, lh, rh"
    }

    fn desc<'a>(&self) -> &'a str {
        "find layouts with a particular set of sfbs"
    }
}

const SIMILARITY_THRES: f64 = 0.7;
pub(super) fn search_is_similar(s1: &str, s2: &str) -> bool {
    jaro_winkler(s1, s2) > SIMILARITY_THRES
}