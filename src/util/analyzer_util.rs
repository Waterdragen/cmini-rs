use std::hash::{Hash, Hasher};

static FINGER_VALUES: [u16; 85] = {
    let mut values = [0; 85];
    values[b'I' as usize] = 0;
    values[b'M' as usize] = 1;
    values[b'R' as usize] = 2;
    values[b'P' as usize] = 3;
    values[b'T' as usize] = 4;
    values[b'B' as usize] = 5;
    values
};

static METRIC_NAME_HASH_TABLE: [u8; 120] = {
    let mut values = [0; 120];
    values[b'R' as usize] = 0;  // sf[R]
    values[b'T' as usize] = 1;  // sf[T]
    values[b'b' as usize] = 2;  // sf[b]
    values[b't' as usize] = 3;  // sf[t]
    values[b'f' as usize] = 4;  // ds[f]b

    values[b'n' as usize] = 5;   // alter[n]ate
    values[b'a' as usize] = 6;   // dsfb-[a]lt
    values[b'r' as usize] = 7;   // dsfb-[r]ed
    values[b'w' as usize] = 8;   // unkno[w]n
    values[b'e' as usize] = 10;  // bad-r[e]direct, redir[e]ct
    values[b'i' as usize] = 32;  // oneh-[i]n, roll-[i]n, red[i]rect
    values[b'o' as usize] = 16;  // oneh-[o]ut, roll-[o]ut
    values[b'-' as usize] = 64;  // bad[-]redirect
    values[b'h' as usize] = 0;   // one[h]
    values[b'l' as usize] = 1;   // rol[l]

    values
};


#[derive(PartialEq, Eq)]
pub struct ThreeFingerCombo {
    fingers: String,
}

impl ThreeFingerCombo {
    #[inline]
    pub fn new(fingers: String) -> Self {
        ThreeFingerCombo{fingers}
    }
}


impl Hash for ThreeFingerCombo {
    #[inline]
    fn hash<H: Hasher>(&self, state: &mut H) {
        let bytes = self.fingers.as_bytes();
        let mut hash_value: u16 = if bytes[0] == b'R' {0b0000_1000_0000_0000} else {0};

        // 0 + n == 0 | n
        if bytes[2] == b'R' {
            hash_value |= 0b0000_0000_1000_0000u16;
        }
        if bytes[4] == b'R' {
            hash_value |= 0b0000_0000_0000_1000u16;
        }

        hash_value |= FINGER_VALUES[bytes[1] as usize] << 8;
        hash_value |= FINGER_VALUES[bytes[3] as usize] << 4;
        hash_value |= FINGER_VALUES[bytes[5] as usize];

        hash_value.hash(state);
    }
}


#[derive(PartialEq, Eq, Clone)]
pub struct MetricName<'a> {
    name: &'a str,
}

impl<'a> MetricName<'a> {
    #[inline]
    pub fn new(name: &'a str) -> Self {
        MetricName{ name }
    }
    #[inline]
    pub fn to_string(&self) -> String {
        self.name.to_string()
    }
}

impl Hash for MetricName<'_> {
    #[inline]
    fn hash<H: Hasher>(&self, state: &mut H) {
        let bytes = self.name.as_bytes();
        if bytes.len() <= 4 {
            state.write_u8(METRIC_NAME_HASH_TABLE[bytes[2] as usize]);
            return;
        }
        let mut hash_value = METRIC_NAME_HASH_TABLE[bytes[5] as usize];
        if hash_value >= 10 {
            hash_value |= METRIC_NAME_HASH_TABLE[bytes[3] as usize];
        }
        hash_value.hash(state);
    }
}
