use fxhash::FxHasher;
use std::hash::Hasher;

#[inline]
pub fn hash_keys(keys: &str) -> u64 {
    let mut hasher = FxHasher::default();
    hasher.write(keys.as_bytes());
    hasher.finish()
}

pub mod layout {
    use crate::util::conv::pos;
    use crate::util::core::Layout;

    pub fn pack(layout: &Layout) -> String {
        let mut layout_packed_ordered: Vec<(String, u32)> = layout.iter().map(|(key, pos)| {
            let mut packed_keypos = String::with_capacity(4);
            packed_keypos.push(*key);
            let packed_pos = pos::pack(pos);
            packed_keypos.push_str(&packed_pos);
            let order = ((pos.0 as u32) << 8) + (pos.1 as u32);
            (packed_keypos, order)
        }).collect();
        layout_packed_ordered.sort_by(|item0, item1| {
            item0.1.cmp(&item1.1)
        });
        let layout_packed: String = layout_packed_ordered.into_iter().map(|(keypos, _)| {
            keypos
        }).collect();

        layout_packed
    }

    pub fn unpack(layout_packed: &str) -> Layout {
        let mut layout = Layout::default();
        let unpacked_chars: Vec<char> = layout_packed.chars().collect();

        for start in (0..unpacked_chars.len()).step_by(4) {
            let key = unpacked_chars[start];
            let mut chunk = String::with_capacity(3);
            (start + 1 .. start + 4).for_each(|index| {
                chunk.push(unpacked_chars[index])
            });
            let pos = pos::unpack(&chunk);
            layout.insert(key, pos);
        }
        layout
    }
}


pub mod stats {
    use crate::util::conv::freq;
    use crate::util::core::{Metric, Stat};

    const INTERVAL: usize = 3;

    pub fn pack(stats: &Stat) -> String {
        let mut packed: Vec<char> = Vec::new();
        packed.resize(INTERVAL * stats.len(), ' ');
        stats.iter()
            .for_each(|(metric, freq)| {
                let index = usize::from(metric.pack()) * INTERVAL;
                packed[index..index + INTERVAL].copy_from_slice(&freq::pack(*freq));
            });
        packed.into_iter().collect()
    }

    pub fn unpack(packed: &str) -> Stat {
        let mut stats = Stat::default();
        let packed: Vec<char> = packed.chars().collect();
        (0..packed.len()).step_by(INTERVAL).enumerate().for_each(|(metric_num, index)| {
            let packed_freq = &packed[index..index + INTERVAL];
            stats.insert(Metric::unpack(metric_num as u8), freq::unpack(packed_freq));
        });
        stats
    }
}


mod base64 {
    #[inline]
    pub fn pack(value: u8) -> char {
        if value < 26 { return char::from(value + 65); }  // A-Z
        if value < 52 { return char::from(value + 71); }  // a-z
        if value < 62 { return char::from(value - 4); }   // 0-9
        if value == 62 { '+' } else { '/' }
    }

    #[inline]
    pub const fn unpack(c: char) -> u32 {
        let ord = c as u32;
        if ord >= 97 { return ord - 71; }  // a-z
        if ord >= 65 { return ord - 65; }  // A-Z
        if ord >= 48 { return ord + 4; }   // 0-9
        if c == '+' { 62 } else { 63 }
    }
}


mod freq {
    use crate::util::conv::base64;

    #[inline]
    pub fn pack(f: f64) -> [char; 3] {
        let num = (f * 100_000.0).round() as u32;
        [
            base64::pack((num >> 12 & 0x3f) as u8),
            base64::pack((num >> 6 & 0x3f) as u8),
            base64::pack((num & 0x3f) as u8),
        ]
    }

    #[inline]
    pub fn unpack(chars: &[char]) -> f64 {
        let num = base64::unpack(chars[0]) << 12 | base64::unpack(chars[1]) << 6 | base64::unpack(chars[2]);
        num as f64 / 100_000.0
    }
}


mod pos {
    use crate::util::core::Position;

    #[inline]
    pub fn pack((row, col, finger): &Position) -> String {
        let mut packed = (u16::from(*row) & 0xf) << 8;
        packed |= (u16::from(*col) & 0xf) << 4;
        packed |= finger & 0xf;
        format!("{:03x}", packed)
    }

    #[inline]
    pub fn unpack(packed_str: &str) -> Position {
        let packed = u16::from_str_radix(packed_str, 16).unwrap();
        let row = (packed >> 8 & 0xf) as u8;
        let col = (packed >> 4 & 0xf) as u8;
        let finger = packed & 0xf;
        (row, col, finger)
    }
}



