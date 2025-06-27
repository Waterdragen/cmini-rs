use crate::cmds::homerow::{homerow_filter_fn, homerow_regex};
use crate::cmds::rank::{RankConfig, RANK_CONFIGS};
use crate::cmds::search::search_is_similar;
use crate::core::{Commandable, FingerUnion};
use crate::message::Message;
use crate::util::cache::CACHED_STATS;
use crate::util::cmp::Comparator;
use crate::util::corpora::get_user_corpus;
use crate::core::finger_alias::FINGER_NAMES;
use crate::util::memory::AUTHORS;
use crate::core::metric_alias::METRIC_NAMES;
use crate::util::parser::{get_kwargs, KwargType as KT};
use fxhash::FxHashMap;
use once_cell::sync::Lazy;
use rand::prelude::{StdRng, SeedableRng, SliceRandom};
use std::borrow::ToOwned;
use std::cmp::Ordering;
use std::iter::{IntoIterator, Iterator};
use thiserror::Error;

static KWARGS: Lazy<FxHashMap<String, KT>> = Lazy::new(|| [
    ("col", KT::Vec),
    ("column", KT::Vec),
    ("homerow", KT::Str),
    ("sort", KT::Str),
    ("sfb", KT::Str),
    ("sfs", KT::Str),
    ("alt", KT::Str),
    ("red", KT::Str),
    ("roll", KT::Str),
    ("oneh", KT::Str),
    ("inroll", KT::Str),
    ("outroll", KT::Str),
    ("rolltal", KT::Str),
    ("inrolltal", KT::Str),
    ("name", KT::Str),
    ("partial", KT::Bool),
    ("punc", KT::Bool),
    ("thumb", KT::Bool),
    ("vowel", KT::Str),
    ("author", KT::Vec),
].into_iter()
    .map(|(s, kt)| (s.to_owned(), kt))
    .collect()
);

pub struct Command;

impl Commandable for Command {
    fn exec(&self, msg: &Message) -> String {
        let kwargs = match get_kwargs(&msg.arg, &KWARGS) {
            Ok(kwargs) => kwargs,
            Err(err) => return format!("{}\n{}", err.to_string(), self.help()),
        };

        let column = kwargs["column"].unwrap_vec().or_else(|| kwargs["col"].unwrap_vec());
        let homerow = kwargs["homerow"].unwrap_str();
        let filter_name = kwargs["name"].unwrap_str();
        let filter_partial = !kwargs["partial"].unwrap_bool();
        let filter_punc = !kwargs["punc"].unwrap_bool();
        let filter_thumb = kwargs["thumb"].unwrap_bool();
        let filter_vowel = kwargs["vowel"].unwrap_str();
        let filter_author = kwargs["author"].unwrap_vec();
        let sort_metric = kwargs["sort"].unwrap_str();

        let mut target_authors = Vec::<u64>::new();
        if let Some(filter_authors) = filter_author {
            let authors = AUTHORS.read();
            for name in filter_authors {
                target_authors.push(authors.get_id(name));
            }
        }
        let filter_stats = match METRIC_NAMES.iter()
            .filter_map(|(name, metric_union)| {
                Some(compare_with_str(kwargs.get(name)?.unwrap_str()?)  // Propagate unsupported/unused metrics
                    .map(|comparator| (*metric_union, comparator)))
            })
            .collect::<Result<Vec<_>, _>>() {
            Ok(filter_stats) => filter_stats,
            Err(err) => return err.to_string(),
        };
        let sort_method: Option<RankConfig> = match sort_metric {
            None => None,
            Some(metric) => match RANK_CONFIGS.get(metric) {
                None => return format!("Error: unsupported metric `{metric}`"),
                rank_config => rank_config.copied(),
            }
        };
        let homerow_regex = match homerow {
            None => None,
            Some(s) => match homerow_regex(s) {
                Ok(regex) => Some((regex, s)),
                Err(_) => return format!("Error: invalid regex `{s}`"),
            }
        };
        let sfb = match column {
            None => None,
            Some(columns) => {
                let mut finger_union = None::<FingerUnion>;
                for finger in columns[1..].iter() {
                    match FINGER_NAMES.get(&finger.to_lowercase()) {
                        None => return format!("Error: unsupported finger: {finger}"),
                        Some(finger) => match finger_union {
                            None => { finger_union = Some(*finger); }
                            Some(finger2) => { finger_union = Some(*finger | finger2); }
                        }
                    }
                }
                Some((&columns[0], finger_union))
            }
        };
        if kwargs.values().all(|kwarg| kwarg.is_unused()) {
            return self.help();
        }
        let cached_stat = CACHED_STATS.read();
        let mut result = cached_stat.iter().filter_map(|(name, stat)|{
            if let Some(filter_vowel) = filter_vowel {
                (filter_vowel.is_empty() ||
                    stat.contains_vowel_in_one_hand(filter_vowel)).then_some(())?;
            }

            (target_authors.is_empty() || target_authors.contains(&stat.user)).then_some(())?;
            (!filter_partial || stat.is_total_layout()).then_some(())?;
            (!filter_punc || stat.contains_full_punc()).then_some(())?;
            (!filter_thumb || stat.contains_thumb_keys()).then_some(())?;

            if let Some((sfb, fingers)) = sfb {
                stat.is_sfb(sfb, fingers).then_some(())?;
            }
            if let Some((ref homerow_regex, search_str)) = homerow_regex {
                let homerow = stat.get_homerow();
                let filter_fn = homerow_filter_fn(homerow_regex, search_str);
                filter_fn(("".to_owned(), homerow))?;
            }
            if let Some(filter_name) = filter_name {
                search_is_similar(filter_name, name).then_some(())?
            }
            let corpus_name = get_user_corpus(msg.id);
            let stat = stat.stats.get(&corpus_name)?;
            if !filter_stats.is_empty() {
                filter_stats.iter().all(|(metric_union, comparator)| {
                    comparator.check(stat.sum(*metric_union))
                }).then_some(())?;
            }
            match sort_method {
                None => Some((name, 0.0)),
                Some(sort_method) => {
                    let value = sort_method.get_value(stat);
                    value.is_finite().then_some((name, value))
                }
            }
        }).collect::<Vec<_>>();

        match sort_method {
            None => {
                let mut rng = StdRng::from_entropy();
                result.shuffle(&mut rng);
            },
            Some(sort_method) => {
                match sort_method.reverse {
                    true => result.sort_unstable_by(|(_, value0), (_, value1)| value1.total_cmp(value0)),
                    false => result.sort_unstable_by(|(_, value0), (_, value1)| value0.total_cmp(value1)),
                }
            }
        }
        let found = result.len();
        let subtotal = if !msg.is_private() { 20usize } else {
            result.iter()
                .scan(0, |acc, (s, _)| {
                    *acc += s.len() + 1;  // String.len() always >= String.chars().count()
                    Some(*acc)
                })
                .position(|len| len >= 1900)
                .unwrap_or(found)
        };
        result.truncate(subtotal);
        if sort_method.is_none() {
            result.sort_unstable_by_key(|(name, _)| *name)
        }
        let all_or_n = match found == subtotal {
            true => "all".to_owned(),
            false => subtotal.to_string(),
        };

        let mut output = format!("I found {found} matches, here are {all_or_n} of them:\n```\n");
        for (name, _) in result.iter() {
            output.push_str(name);
            output.push('\n');
        }
        output.push_str("```");
        output
    }

    fn usage<'a>(&self) -> &'a str {
        "filter [--kwargs]\n\
         Supported options: \n\
         [--col(umn) <sfb_keys> [fingers...]],\n\
         [--homerow <keys | \"sequence\">],\n\
         [--sort <metric>],\n\
         [--name <name>]\n\
         [--<metric> {< or >}{num}]\n\
         [--partial]\n\
         [--punc]\n\
         [--thumb]\n\
         [--vowel]\n\
         [--author <name>]\n\
         metrics: sfb, sfs, alt, red, roll, oneh, inroll, outroll, rolltal, inrolltal\n"
    }

    fn desc<'a>(&self) -> &'a str {
        "Filters layouts by column, homerow, name, metric.\n\
         Sort the layouts alphabetically or by metric.\n\
         Filters out layouts that doesn't complete the English alphabet\n\
         Use `--partial` to include partial layouts\n\
         Use `--punc` to include layouts without .,\"\n\
         Use `--thumb` to include thumb layouts\n\
         Use `--vowel` to filter for vowel hand letters\n\
         Use `--author` to include authors\n"
    }
}

#[derive(Debug, Error)]
enum ParseComparatorError<'a> {
    #[error("Error: '{0}' does not start with greater or less than operator")]
    MissingCmpOperator(&'a str),
    #[error("Error: cannot convert '{0}' into a number")]
    NotANumber(&'a str),
}

fn compare_with_str(rule: &str) -> Result<Comparator<f64>, ParseComparatorError> {
    let gt = rule.starts_with('>');
    let lt = rule.starts_with('<');
    let ordering = match (gt, lt) {
        (false, false) => return Err(ParseComparatorError::MissingCmpOperator(rule)),
        (true, false) => Ordering::Greater,
        (false, true) => Ordering::Less,
        (true, true) => unreachable!(),
    };
    let num = match rule[1..].parse::<f64>() {
        Ok(num) => num,
        Err(_) => return Err(ParseComparatorError::NotANumber(&rule[1..])),
    };
    // Convert percent to ratio
    Ok(Comparator::new(num / 100.0, ordering))
}