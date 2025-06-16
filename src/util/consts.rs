use crate::util::core::Metric;
use crate::util::jsons::get_table;
use once_cell::sync::Lazy;
use serenity::model::prelude::ChannelId;

pub const CMINI_CHANNEL: ChannelId = ChannelId(1063291226243207268);

pub const TRIGGERS: [&str; 5] = ["!amini", "bmini", "!cmini", "!dvormini", "!cnini"];

pub static TABLE: Lazy<[Metric; 4096]> = Lazy::new(|| get_table("./table.json"));

pub const FMAP_STANDARD: [u16; 10] = [0, 1, 2, 3, 3, 6, 6, 7, 8, 9];
pub const FMAP_ANGLE: [u16; 10] = [1, 2, 3, 3, 3, 6, 6, 7, 8, 9];
pub const FREE_CHAR: char = '~';
