use crate::core::Finger::*;
use crate::core::{Finger, Key, Table};
use crate::util::jsons::get_table;
use once_cell::sync::Lazy;
use serenity::model::prelude::ChannelId;
pub const CMINI_CHANNEL: ChannelId = ChannelId(1063291226243207268);

pub const TRIGGERS: [&str; 5] = ["!amini", "bmini", "!cmini", "!dvormini", "!cnini"];

pub static TABLE: Lazy<Table> = Lazy::new(|| get_table("./table.json"));

pub const FMAP_STANDARD: [Finger; 10] = [LP, LR, LM, LI, LI, RI, RI, RM, RR, RP];
pub const FMAP_ANGLE: [Finger; 10] = [LR, LM, LI, LI, LI, RI, RI, RM, RR, RP];
pub const FREE_CHAR: Key = '~';
pub const ROW_LIMIT: usize = 4;
pub const COL_LIMIT: usize = 36;
pub const COL_RADIX: u32 = 36;
pub const ZERO_WIDTH_SPACE: char = '\u{200B}';
