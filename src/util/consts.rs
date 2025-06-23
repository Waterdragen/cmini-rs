use crate::util::core::{Finger, FingerUnion, Key, Metric};
use Finger::*;
use crate::util::jsons::get_table;
use once_cell::sync::Lazy;
use serenity::model::prelude::ChannelId;

pub const CMINI_CHANNEL: ChannelId = ChannelId(1063291226243207268);

pub const TRIGGERS: [&str; 5] = ["!amini", "bmini", "!cmini", "!dvormini", "!cnini"];

pub static TABLE: Lazy<[Metric; 4096]> = Lazy::new(|| get_table("./table.json"));

pub const FMAP_STANDARD: [Finger; 10] = [LP, LR, LM, LI, LI, RI, RI, RM, RR, RP];
pub const FMAP_ANGLE: [Finger; 10] = [LR, LM, LI, LI, LI, RI, RI, RM, RR, RP];
pub const FREE_CHAR: Key = '~';
pub static LH: Lazy<FingerUnion> = Lazy::new(|| LP | LR | LM | LI | LT);
pub static RH: Lazy<FingerUnion> = Lazy::new(|| RP | RR | RM | RI | RT);
