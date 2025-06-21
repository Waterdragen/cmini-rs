mod _8ball;
mod add;
mod admin;
mod alternates;
mod assign;
mod authors;
mod catball;
mod corpus;
mod dofball;
mod github;
mod help;
mod inrolls;
mod inrolltals;
mod like;
mod likes;
mod link;
pub mod maintenance;
mod onehands;
mod outrolltals;
pub mod question;
mod random;
mod redirects;
mod remove;
mod rename;
mod rolls;
mod rolltals;
mod sfbs;
mod sfs;
mod suggest;
mod unlike;
mod unlink;
mod view;
mod wooperball;
mod woopercat;
mod angle;
mod angle_mut;
mod cycle;
mod cycle_mut;
mod unangle;
mod unangle_mut;
mod mirror;
mod mirror_mut;

use crate::util::core::{Commandable, ContainsMetric};
use crate::util::memory::LAYOUTS;
use crate::util::parser::{get_kwargs, KwargType, ParseKwargError};
use crate::util::Message;
use fxhash::FxHashMap;
use once_cell::sync::Lazy;

static COMMANDS: Lazy<FxHashMap<String, Box<dyn Commandable>>> = Lazy::new(|| {
    FxHashMap::from_iter([
        ("8ball", _8ball::Command.init()),
        ("add", add::Command.init()),
        ("admin", admin::Command.init()),
        ("alternates", alternates::Command.init()),
        ("angle", angle::Command.init()),
        ("angle!", angle_mut::Command.init()),
        ("assign", assign::Command.init()),
        ("authors", authors::Command.init()),
        ("catball", catball::Command.init()),
        ("corpus", corpus::Command.init()),
        ("cycle", cycle::Command.init()),
        ("cycle!", cycle_mut::Command.init()),
        ("dofball", dofball::Command.init()),
        ("gh", github::Command.init()),
        ("github", github::Command.init()),
        ("help", help::Command.init()),
        ("inrolls", inrolls::Command.init()),
        ("inrolltals", inrolltals::Command.init()),
        ("like", like::Command.init()),
        ("likes", likes::Command.init()),
        ("link", link::Command.init()),
        ("mirror", mirror::Command.init()),
        ("mirror!", mirror_mut::Command.init()),
        ("onehands", onehands::Command.init()),
        ("outrolltals", outrolltals::Command.init()),
        ("random", random::Command.init()),
        ("redirects", redirects::Command.init()),
        ("remove", remove::Command.init()),
        ("rename", rename::Command.init()),
        ("rolls", rolls::Command.init()),
        ("rolltals", rolltals::Command.init()),
        ("sfbs", sfbs::Command.init()),
        ("sfs", sfs::Command.init()),
        ("suggest", suggest::Command.init()),
        ("swap", cycle::Command.init()),
        ("swap!", cycle_mut::Command.init()),
        ("unangle", unangle::Command.init()),
        ("unangle!", unangle_mut::Command.init()),
        ("unlike", unlike::Command.init()),
        ("unlink", unlink::Command.init()),
        ("wooperball", wooperball::Command.init()),
        ("woopercat", woopercat::Command.init()),
        ("view", view::Command.init()),
    ].into_iter().map(|(name, obj)| (name.to_string(), obj)))
});

static OTHER_COMMANDS: Lazy<FxHashMap<String, Box<dyn Commandable>>> = Lazy::new(|| {
    FxHashMap::from_iter([
        ("maintenance", maintenance::Command.init()),
        ("question", question::Command.init()),
    ].into_iter().map(|(name, obj)| (name.to_string(), obj)))
});

pub fn get_cmd(name: &str) -> Option<&dyn Commandable> {
    COMMANDS.get(name).map(|v| &**v)
}

static KWARGS_FOR_TOP_TRIGRAMS_OF_METRIC: Lazy<FxHashMap<String, KwargType>>
= Lazy::new(|| FxHashMap::from_iter([
    ("top".to_owned(), KwargType::Str),
]));

/// - Implementation used by alternates, inrolls, inrolltals, etc.
/// - Only differences are the metrics and metric names.
fn cmd_for_top_trigrams_of_metric<M: ContainsMetric>(msg: &Message, metric: M, metric_name: &str)
    -> Option<String> {  // FIXME: Handle Commandable::help() as part of error, so we can use Result<_, Box<dyn Error>> here
    const DEFAULT_TOP_N: usize = 10;
    const MAX_TOP_N: usize = 100;
    let kwargs = match get_kwargs(msg.arg, &KWARGS_FOR_TOP_TRIGRAMS_OF_METRIC) {
        Ok(kwargs) => kwargs,
        Err(err) => return Some(err.to_string()),
    };
    if kwargs.arg.is_empty() {
        return None;
    }
    let ll = &*LAYOUTS.find(&kwargs.arg);
    let top_n = match kwargs["top"].unwrap_str() {
        None => DEFAULT_TOP_N,
        Some(top_n_str) => match top_n_str.parse::<usize>() {
            Ok(top_n) => if top_n > MAX_TOP_N {
                return Some(format!("Error: can only display at most {MAX_TOP_N} results, got {top_n}"));
            } else { top_n }
            Err(_) => return Some(ParseKwargError::Invalid("top".to_owned()).to_string()),
        }
    };
    let layout_name = &ll.name;
    let filtered_trigrams = ll.top_trigrams_of_metric(msg.id, metric, top_n);
    let mut s = format!("```\nTop {top_n} {layout_name} {metric_name}:\n");
    for (gram, freq) in filtered_trigrams {
        let freq_percent = freq * 100.0;
        s.push(gram[0]);
        s.push(gram[1]);
        s.push(gram[2]);
        s.push_str(&format!("  {freq_percent:.3}%"));
        s.push('\n');
    }
    s.push_str("```");
    Some(s)
}