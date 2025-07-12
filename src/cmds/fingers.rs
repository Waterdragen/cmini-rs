use crate::util::corpora::{self, get_user_corpus_upper};
use crate::util::memory::AUTHORS;
use crate::util::memory::{LAYOUTS, LIKES};
use crate::util::parser::split_words;
use crate::{Commandable, Message};
use cmini_core::metric_alias::{list_metric_union_names, METRIC_NAMES};
use cmini_core::Finger;
use cmini_core::{FingerUsage, LayoutConfig, Metric};
use itertools::Itertools;

pub struct Command;

impl Commandable for Command {
    fn exec(&self, msg: &Message) -> String {
        let id = msg.id;
        let [layout_name, metric_name] = split_words(msg.arg);
        let mut metric_name = metric_name.to_lowercase();
        if layout_name.is_empty() {
            return self.help();
        }
        let ll = &*LAYOUTS.find(layout_name);
        let mut transform_fn: fn(f64) -> f64 = std::convert::identity;
        let stats = match metric_name.as_str() {
            "usage" | "all" | "" => {
                let monograms = corpora::ngrams::<1>(id);
                metric_name = "finger usage".to_owned();
                ll.fingers_usage(&monograms)
            },
            _ => match METRIC_NAMES.get(&metric_name) {
                None => return format!("Error: `{metric_name}` not supported. \n\
                                        {}", self.help()),
                Some(&metric_union) => {
                    let trigrams = corpora::ngrams::<3>(id);
                    if metric_union == Metric::Sfb.into() {
                        transform_fn = |freq: f64| freq / 2.0;
                    }
                    ll.fingers_usage_of_metric(&trigrams, metric_union)
                },
            }
        };
        pretty_finger_stats(stats, ll, id, &metric_name, transform_fn)
    }

    fn usage<'a>(&self) -> &'a str {
        "fingers <layout> [metric]"
    }

    fn desc<'a>(&self) -> &'a str {
        "view stats of each finger"
    }

    fn help(&self) -> String {
        let supported_metric_names = list_metric_union_names().join(" | ");
        format!("\
```
{}
{}

- metric
  - view all metrics: usage | all | ''
  - one metric: {}
```",
                self.usage(),
                self.desc(),
                supported_metric_names)
    }
}

fn pretty_finger_stats<F>(mut stats: FingerUsage, ll: &LayoutConfig, id: u64, metric_name: &str, mut transform_fn: F) -> String
where F: FnMut(f64) -> f64 {
    let layout_name = &ll.name;
    let author = {
        let authors = AUTHORS.read();
        authors.get_name(ll.user)
            .unwrap()  // Always succeeds: all authors added to LAYOUTS exist in AUTHORS, ensured by add command
            .to_owned()
    };
    let corpus_name = get_user_corpus_upper(id);
    let likes = {
        let likes = LIKES.read();
        likes.get(&ll.name)
            .map(|liked_users| liked_users.len())
            .unwrap_or(0)
    };
    let like_str = if likes == 1 { "like" } else { "likes" };
    let matrix_str = ll.matrix_str();
    let mut output = format!(
        "```\n\
         {layout_name} ({metric_name}) ({author}) ({likes} {like_str})\n\
         {matrix_str}\n\
         \n\
         {corpus_name}:\n");
    const LH: [Finger; 4] = [Finger::LI, Finger::LM, Finger::LR, Finger::LP];
    const RH: [Finger; 4] = [Finger::RI, Finger::RM, Finger::RR, Finger::RP];
    for (&lfinger, &rfinger) in LH.iter().zip(RH.iter()) {
        let lfinger_name: &str = lfinger.into();
        let lfreq = stats[lfinger] * 100.0;
        let rfinger_name: &str = rfinger.into();
        let rfreq = stats[rfinger] * 100.0;
        output.push_str(&format!("  {lfinger_name}: {lfreq:>5.2}%    {rfinger_name}: {rfreq:>5.2}%\n"));
    }
    output.push('\n');

    let mut uses_thumb = false;
    for thumb in [Finger::LT, Finger::RT] {
        let freq = stats[thumb] * 100.0;
        if freq == 0.0 {
            continue;
        }
        let thumb_name: &str = thumb.into();
        output.push_str(&format!("  {thumb_name}: {freq:.2}%\n"));
        uses_thumb = true;
    }
    if uses_thumb {
        output.push('\n');
    }
    for freq in stats.values_mut() {
        *freq = transform_fn(*freq);
    }
    let total = stats.values().sum::<f64>() * 100.0;
    output.push_str(&format!("  Total: {total:.2}%"));
    output.push_str("```");
    output
}