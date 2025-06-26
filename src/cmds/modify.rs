use crate::cmds::angle::impl_angle;
use crate::cmds::cycle::impl_cycle;
use crate::cmds::mirror::impl_mirror;
use crate::cmds::unangle::impl_unangle;
use crate::util::memory::LAYOUTS;
use crate::util::parser::{get_kwargs, KwargType};
use crate::{Commandable, Message};
use fxhash::FxHashMap;
use once_cell::sync::Lazy;
use std::borrow::ToOwned;

pub struct Command;

static KWARGS: Lazy<FxHashMap<String, KwargType>> = Lazy::new(|| FxHashMap::from_iter([
    ("angle".to_owned(), KwargType::Bool),
    ("unangle".to_owned(), KwargType::Bool),
    ("mirror".to_owned(), KwargType::Bool),
    ("cycle".to_owned(), KwargType::Str),
    ("swap".to_owned(), KwargType::Str),
]));

impl Commandable for Command {
    fn exec(&self, msg: &Message) -> String {
        let kwargs = match get_kwargs(msg.arg, &KWARGS) {
            Ok(kwargs) => kwargs,
            Err(err) => return err.to_string(),
        };
        let layout_name = &kwargs.arg;
        if layout_name.is_empty() {
            return self.help();
        }
        let mut ll = LAYOUTS.find(layout_name).clone();
        // Unangle prevails if both angle and unangle
        if kwargs["unangle"].unwrap_bool() {
            impl_unangle(&mut ll);
        } else if kwargs["angle"].unwrap_bool() {
            if let Err(err) = impl_angle(&mut ll) {
                return err.to_string();
            }
        }
        if kwargs["mirror"].unwrap_bool() {
            impl_mirror(&mut ll);
        }
        for cycle_kw in ["cycle", "swap"] {
            if let Some(cycle) = kwargs[cycle_kw].unwrap_str() {
                let cycles = cycle.split_whitespace().collect::<Vec<_>>();
                if let Err(err) = impl_cycle(&mut ll, &cycles) {
                    return err.to_string();
                }
            }
        }
        ll.name.push_str(" (modified)");
        ll.to_pretty(msg.id)
    }

    fn usage<'a>(&self) -> &'a str {
        "\
mod <layout_name> [--kwarg1, --kwarg2, ...]
--angle:
    view the angle modded version of a layout
--unangle:
    view the non angle modded version of a layout
--mirror:
    view the mirrored version of a layout
--cycle <chars>:
    cycle a layout's letters around
--swap <chars>:
    alias of --cycle"
    }

    fn desc<'a>(&self) -> &'a str {
        "see the stats of a layout with chained modifications"
    }
}