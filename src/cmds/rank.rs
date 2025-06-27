use crate::util::cache::CACHED_STATS;
use crate::core::{ContainsMetric, FxIndexMap, Metric as M, Metric, MetricUnion, Stat};
use crate::util::corpora::get_user_corpus;
use crate::core::metric_alias::{IN_ROLLTALS, ONEHANDS, OUT_ROLLTALS, REDIRECTS, ROLLS, ROLLTALS, SFS};
use crate::util::parser::{get_kwargs, split_word, KwargType};
use crate::{Commandable, Message};
use fxhash::FxHashMap;
use itertools::Itertools;
use once_cell::sync::Lazy;
use std::borrow::ToOwned;

#[derive(Copy, Clone, PartialEq)]
enum RankMode {
    Only(Metric),
    Sum(MetricUnion),
    Divide(Metric, Metric),
}

impl RankMode {
    fn get_value(self, stat: &Stat) -> f64 {
        match self {
            RankMode::Only(metric) => {
                *stat.get(metric)
            }
            RankMode::Sum(metric_union) => {
                stat.iter()
                    .filter_map(|(metric, freq)| metric_union.contains(metric).then_some(freq))
                    .sum()
            }
            RankMode::Divide(metric_numer, metric_denom) => {
                stat.get(metric_numer) / stat.get(metric_denom)
            }
        }
    }
}

#[derive(Copy, Clone, PartialEq)]
pub(super) struct RankConfig {
    rank_mode: RankMode,
    pub(super) reverse: bool,
    percent: bool,
}

impl RankConfig {
    fn only(metric: Metric) -> Self {
        Self {
            rank_mode: RankMode::Only(metric),
            reverse: false,
            percent: true,
        }
    }
    fn sum(metric_union: MetricUnion) -> Self {
        Self {
            rank_mode: RankMode::Sum(metric_union),
            reverse: false,
            percent: true,
        }
    }
    fn divide(metric_numer: Metric, metric_denom: Metric) -> Self {
        Self {
            rank_mode: RankMode::Divide(metric_numer, metric_denom),
            reverse: false,
            percent: true,
        }
    }
    fn reverse(mut self) -> Self {
        self.reverse = true;
        self
    }
    fn no_percent(mut self) -> Self {
        self.percent = false;
        self
    }
    fn format_f64(&self, float: f64) -> String {
        if self.percent {
            format!("{:.2}%", float * 100.0)
        } else {
            format!("{:.3}", float)
        }
    }
    pub(super) fn get_value(self, stat: &Stat) -> f64 {
        match self.rank_mode {
            RankMode::Only(metric) => {
                *stat.get(metric)
            }
            RankMode::Sum(metric_union) => {
                stat.iter()
                    .filter_map(|(metric, freq)| metric_union.contains(metric).then_some(freq))
                    .sum()
            }
            RankMode::Divide(metric_numer, metric_denom) => {
                stat.get(metric_numer) / stat.get(metric_denom)
            }
        }
    }
}

// Use IndexMap to list metric with dedup_by
pub(super) static RANK_CONFIGS: Lazy<FxIndexMap<String, RankConfig>> = Lazy::new (|| FxIndexMap::from_iter([
    ("alt", RankConfig::only(M::Alt).reverse()),
    ("alts", RankConfig::only(M::Alt).reverse()),
    ("alternate", RankConfig::only(M::Alt).reverse()),
    ("alternates", RankConfig::only(M::Alt).reverse()),
    ("sfb", RankConfig::only(M::Sfb)),
    ("sfbs", RankConfig::only(M::Sfb)),
    ("sft", RankConfig::only(M::Sft)),
    ("sfs", RankConfig::sum(SFS)),
    ("dsfb", RankConfig::sum(SFS)),
    ("sfr", RankConfig::only(M::Sfr)),
    ("red", RankConfig::sum(REDIRECTS)),
    ("redirect", RankConfig::sum(REDIRECTS)),
    ("redirects", RankConfig::sum(REDIRECTS)),
    ("oneh", RankConfig::sum(ONEHANDS).reverse()),
    ("onehand", RankConfig::sum(ONEHANDS).reverse()),
    ("onehands", RankConfig::sum(ONEHANDS).reverse()),
    ("inroll", RankConfig::only(M::InRoll).reverse()),
    ("inrolls", RankConfig::only(M::InRoll).reverse()),
    ("roll-in", RankConfig::only(M::InRoll).reverse()),
    ("outroll", RankConfig::only(M::OutRoll).reverse()),
    ("outrolls", RankConfig::only(M::OutRoll).reverse()),
    ("roll-out", RankConfig::only(M::OutRoll).reverse()),
    ("inrolltal", RankConfig::sum(IN_ROLLTALS).reverse()),
    ("inrolltals", RankConfig::sum(IN_ROLLTALS).reverse()),
    ("outrolltal", RankConfig::sum(OUT_ROLLTALS).reverse()),
    ("outrolltals", RankConfig::sum(OUT_ROLLTALS).reverse()),
    ("roll", RankConfig::sum(ROLLS).reverse()),
    ("rolls", RankConfig::sum(ROLLS).reverse()),
    ("roll-total", RankConfig::sum(ROLLS).reverse()),
    ("rolltal", RankConfig::sum(ROLLTALS).reverse()),
    ("rolltals", RankConfig::sum(ROLLTALS).reverse()),
    ("rolltotal", RankConfig::sum(ROLLTALS).reverse()),
    ("inrollratio", RankConfig::divide(M::InRoll, M::OutRoll).reverse().no_percent()),
    ("outrollratio", RankConfig::divide(M::OutRoll, M::InRoll).reverse().no_percent()),
].into_iter().map(|(s, metric)| (s.to_owned(), metric))));

fn list_rank_config_names<'a>() -> impl Iterator<Item = &'a str> {
    RANK_CONFIGS.iter()
        .dedup_by(|(_, &metric0), (_, &metric1)| {
            metric0 == metric1
        })
        .map(|(name, _)| name.as_str())
}

static KWARGS: Lazy<FxHashMap<String, KwargType>> = Lazy::new(|| FxHashMap::from_iter([
    ("min".to_owned(), KwargType::Bool),
    ("max".to_owned(), KwargType::Bool),
]));

const LENGTH: usize = 15;

pub struct Command;

impl Commandable for Command {
    fn exec(&self, msg: &Message) -> String {
        let kwargs = match get_kwargs(msg.arg, &KWARGS) {
            Ok(kwargs) => kwargs,
            Err(err) => return err.to_string(),
        };
        let mut arg = &*kwargs.arg;
        let metric = split_word(&mut arg);
        let start = arg;

        if metric.is_empty() {
            return self.help();
        }
        let Some(rank_config) = RANK_CONFIGS.get(metric) else {
            return format!("Error: metric `{metric}` not supported\nHelp:\n{}", self.help());
        };
        let start = if start.is_empty() { 0 } else {
            match start.parse::<usize>() {
                Ok(start) => start,
                Err(_) => return format!("Error: invalid starting index `{start}`"),
            }
        };
        let sort_asc = kwargs["min"].unwrap_bool();
        let sort_desc = kwargs["max"].unwrap_bool();
        if sort_asc && sort_desc {
            return "Error: cannot rank ascending and descending altogether".to_owned();
        }
        let should_rev = match sort_asc || sort_desc {
            true => sort_desc,
            false => rank_config.reverse,
        };

        let corpus_name = get_user_corpus(msg.id);
        let cached_stats = CACHED_STATS.read();
        let mut res = cached_stats.iter()
            .filter_map(|(name, stats)| {
                if !stats.is_total_layout() {
                    return None;  // Only for layouts that has a to z
                }
                let stats = stats.stats.get(&corpus_name)?;
                let freq = rank_config.get_value(stats);
                freq.is_finite().then_some((name, freq))
            })
            .collect::<Vec<_>>();
        res.sort_unstable_by(match should_rev {
            false => |(_, freq0): &(&String, f64), (_, freq1): &(&String, f64)| freq0.total_cmp(freq1),
            true => |(_, freq0): &(&String, f64), (_, freq1): &(&String, f64)| freq1.total_cmp(freq0),
        });
        let Some(res) = res.get(start..start + LENGTH) else {
            return format!("Error: got index {start}, but there are only {} layouts", res.len());
        };
        let corpus_name_upper = corpus_name.to_ascii_uppercase();
        let mut output = format!("```\n{corpus_name_upper}\n");

        for (i, (name, freq)) in res.iter().enumerate() {
            let index = start + i;
            let freq = rank_config.format_f64(*freq);
            output.push_str(&format!("{index}: {freq} -- {name}\n"));
        }
        output.push_str("```");
        output
    }

    fn usage<'a>(&self) -> &'a str {
        "rank <metric> [--min | --max]"
    }

    fn desc<'a>(&self) -> &'a str {
        "rank layouts based on a metric"
    }

    fn help(&self) -> String {
        let mut help = format!("```\n{}\n{}\nSupported rank metrics:\n", self.usage(), self.desc());
        let mut names = list_rank_config_names();
        help.push_str(names.next().unwrap());  // names are not empty
        for name in names {
            help.push_str(", ");
            help.push_str(name);
        }
        help.push_str("```");
        help
    }
}