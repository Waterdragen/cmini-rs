use fxhash::FxHasher;
use std::hash::Hasher;

#[inline]
pub fn hash_keys(keys: &str) -> u64 {
    let mut hasher = FxHasher::default();
    hasher.write(keys.as_bytes());
    hasher.finish()
}

pub mod layout {
    use super::pos;
    use crate::core::Layout;

    pub fn pack(layout: &Layout) -> String {
        let mut layout_packed_ordered: Vec<(String, u32)> = layout.iter().map(|(key, pos)| {
            let mut packed_keypos = String::with_capacity(4);
            packed_keypos.push(*key);
            let packed_pos = pos::pack(*pos);
            packed_keypos.push_str(&packed_pos);
            let order = ((pos.row as u32) << 8) + (pos.col as u32);
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


pub mod freq {
    use super::base64;

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
    use crate::consts::COL_RADIX;
    use crate::core::Position;
    use crate::core::Finger;

    #[inline]
    pub fn pack(Position { row, col, finger }: Position) -> String {
        let mut s = String::with_capacity(3);
        s.push(char::from_digit(u32::from(row), 4).unwrap());
        s.push(char::from_digit(u32::from(col), COL_RADIX).unwrap());
        s.push(char::from_digit(finger.as_u8().into(), 10).unwrap());
        s
    }

    #[track_caller]
    #[inline]
    pub fn unpack(packed_str: &str) -> Position {
        let mut chars = packed_str.chars();
        let row = chars.next().unwrap().to_digit(4).unwrap() as u8;
        let col = chars.next().unwrap().to_digit(COL_RADIX).unwrap() as u8;
        let finger = chars.next().unwrap().to_digit(10).unwrap() as u8;
        Position::new(row, col, Finger::from_u8(finger))
    }
}
