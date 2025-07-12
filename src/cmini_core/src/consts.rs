use once_cell::sync::Lazy;
use crate::{Finger, Key, Table};
use crate::Finger::*;

pub static TABLE: Lazy<Table> = Lazy::new(Table::new);

pub const FMAP_STANDARD: [Finger; 10] = [LP, LR, LM, LI, LI, RI, RI, RM, RR, RP];
pub const FMAP_ANGLE: [Finger; 10] = [LR, LM, LI, LI, LI, RI, RI, RM, RR, RP];
pub const FREE_CHAR: Key = '~';
pub const ROW_LIMIT: usize = 4;
pub const COL_LIMIT: usize = 36;
pub const COL_RADIX: u32 = 36;
pub const ZERO_WIDTH_SPACE: char = '\u{200B}';