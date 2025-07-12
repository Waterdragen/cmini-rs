use crate::{Commandable, Message};
use crate::util::memory::LAYOUTS;
use cmini_core::{Finger, LayoutConfig};
use thiserror::Error;
use crate::util::layout::to_pretty;

pub struct Command;

impl Commandable for Command {
    fn exec(&self, msg: &Message) -> String {
        if msg.arg.is_empty() {
            return self.help();
        }
        let mut ll = LAYOUTS.find(msg.arg).clone();
        if let Err(err) = impl_angle(&mut ll) {
            return err.to_string();
        }
        ll.name.push_str(" (angle modded)");
        to_pretty(&ll, msg.id)
    }

    fn usage<'a>(&self) -> &'a str {
        "angle <layout_name>"
    }

    fn desc<'a>(&self) -> &'a str {
        "view the angle modded version of a layout"
    }
}

#[derive(Debug, Error)]
pub(super) enum AngleError {
    #[error("Error: cannot angle mod mini layouts")]
    GotMiniLayout,
}

pub(super) fn impl_angle(ll: &mut LayoutConfig) -> Result<(), AngleError> {
    if &ll.board == "mini" {
        return Err(AngleError::GotMiniLayout);
    }
    if &ll.board == "angle" {
        return Ok(());
    }
    for pos in ll.keys.values_mut() {
        if pos.row != 2 {
            continue;
        }
        let col = &mut pos.col;
        if *col >= 5 {
            continue;
        }
        if *col == 0 {
            *col = 4;
            pos.finger = Finger::LI;
        } else {
            *col -= 1;
        }
    }
    ll.board = "angle".to_owned();
    Ok(())
}