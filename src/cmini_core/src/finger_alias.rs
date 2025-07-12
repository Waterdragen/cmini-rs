use crate::{Finger, FingerUnion};
use fxhash::FxHashMap;
use once_cell::sync::Lazy;

const fn raw_bit(metric: Finger) -> u16 {
    1 << metric as u16
}

const LP: u16 = raw_bit(Finger::LP);
const LR: u16 = raw_bit(Finger::LR);
const LM: u16 = raw_bit(Finger::LM);
const LI: u16 = raw_bit(Finger::LI);
const LT: u16 = raw_bit(Finger::LT);
const RT: u16 = raw_bit(Finger::RT);
const RI: u16 = raw_bit(Finger::RI);
const RM: u16 = raw_bit(Finger::RM);
const RR: u16 = raw_bit(Finger::RR);
const RP: u16 = raw_bit(Finger::RP);

const PINKY: FingerUnion = FingerUnion::from_raw(LP | RP);
const RING: FingerUnion = FingerUnion::from_raw(LR | RR);
const MIDDLE: FingerUnion = FingerUnion::from_raw(LM | RM);
const INDEX: FingerUnion = FingerUnion::from_raw(LI | RI);
pub const THUMB: FingerUnion = FingerUnion::from_raw(LT | RT);
pub const LH: FingerUnion = FingerUnion::from_raw(LP | LR | LM | LI | LT);
pub const RH: FingerUnion = FingerUnion::from_raw(RP | RR | RM | RI | RT);
pub const NO_FINGER: FingerUnion = FingerUnion::from_raw(0);
pub const ANY_FINGER: FingerUnion = FingerUnion::from_raw(LP | LR | LM | LI | LT | RP | RR | RM | RI | RT);

pub static FINGER_NAMES: Lazy<FxHashMap<String, FingerUnion>> = Lazy::new(|| [
    ("lp", Finger::LP.into()),
    ("lr", Finger::LR.into()),
    ("lm", Finger::LM.into()),
    ("li", Finger::LI.into()),
    ("lt", Finger::LT.into()),
    ("rt", Finger::RT.into()),
    ("ri", Finger::RI.into()),
    ("rm", Finger::RM.into()),
    ("rr", Finger::RR.into()),
    ("rp", Finger::RP.into()),
    ("pinky", PINKY),
    ("ring", RING),
    ("middle", MIDDLE),
    ("index", INDEX),
    ("thumb", THUMB),
    ("lh", LH),
    ("rh", RH),
].into_iter().map(|(s, finger)| (s.to_owned(), finger)).collect());