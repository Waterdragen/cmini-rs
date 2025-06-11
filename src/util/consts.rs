use fxhash::FxHashSet;
use once_cell::sync::Lazy;
use serenity::model::prelude::ChannelId;
use crate::util::core::Metric;
use crate::util::jsons::get_table;

pub const CMINI_CHANNEL: ChannelId = ChannelId(1063291226243207268);

pub const TRIGGERS: [&str; 5] = ["!amini", "bmini", "!cmini", "!dvormini", "!cnini"];

pub static ADMINS: Lazy<FxHashSet<u64>> = Lazy::new(|| FxHashSet::from_iter([
    169285177481101312,  // Eve
    474550534301548556,  // <3
    401316842083450881,  // Waterdragen
]));

pub static TABLE: Lazy<[Metric; 4096]> = Lazy::new(|| get_table("./table.json"));

pub const FMAP_STANDARD: [u16; 10] = [0, 1, 2, 3, 3, 6, 6, 7, 8, 9];
pub const FMAP_ANGLE: [u16; 10] = [1, 2, 3, 3, 3, 6, 6, 7, 8, 9];
pub const FREE_CHAR: char = '~';
