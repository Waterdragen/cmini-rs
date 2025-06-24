use crate::util::core::{LayoutConfig, Position};
use crate::util::memory::LAYOUTS;
use crate::util::parser::split_word;
use crate::util::{Commandable, Message};
use fxhash::FxHashSet;
use thiserror::Error;

pub struct Command;

impl Commandable for Command {
    fn exec(&self, msg: &Message) -> String {
        let mut arg = msg.arg;
        let name = split_word(&mut arg);
        let cycles = arg.split_whitespace().collect::<Vec<_>>();

        if name.is_empty() {
            return self.help();
        }
        let mut ll = LAYOUTS.find(name).clone();
        if let Err(err) = impl_cycle(&mut ll, &cycles) {
            return err.to_string();
        }
        ll.name.push_str(" (modified)");
        ll.to_pretty(msg.id)
    }

    fn usage<'a>(&self) -> &'a str {
        "cycle | swap <layout_name> <chars>"
    }

    fn desc<'a>(&self) -> &'a str {
        "cycle a layout's letters around"
    }
}

#[derive(Debug, Error)]
pub(super) enum CycleError {
    #[error("Error: cannot swap letters that aren't in the layout")]
    CharNotExist,
    #[error("Error: cannot use duplicate letters in cycle command")]
    DuplicateChar,
}

pub(super) fn impl_cycle(ll: &mut LayoutConfig, cycles: &[&str]) -> Result<(), CycleError> {
    if !cycles.iter()
        .flat_map(|s| s.chars())
        .all(|c| ll.keys.contains_key(&c)) {
        return Err(CycleError::CharNotExist);
    }
    for cycle in cycles {
        let len = cycle.chars().count();
        if len != FxHashSet::<char>::from_iter(cycle.chars()).len() {
            return Err(CycleError::DuplicateChar);
        }
        let mut yield_char = cycle.chars()
            .rev()  // counter-clockwise cycle in values is clockwise cycle in keys
            .cycle()
            .take(len);
        let prev_char = yield_char.next().unwrap();  // Caller should ensure cycle is not empty
        for new_char in yield_char {
            // get_mut always succeeds, checked by contains_key
            let prev_pos = ll.keys.get_mut(&prev_char).unwrap() as *mut Position;
            let new_pos = ll.keys.get_mut(&new_char).unwrap() as *mut Position;
            // SAFETY: both ptrs are from valid &mut Position
            unsafe {
                std::ptr::swap(prev_pos, new_pos);
            }
        }
    }
    Ok(())
}
