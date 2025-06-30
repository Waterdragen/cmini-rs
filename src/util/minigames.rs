use crate::core::Key;
use crate::prelude::{Lazy, Mutex};
use fxhash::FxHashMap;
use rand::prelude::{SliceRandom, StdRng};
use std::cmp::Ordering;
use std::fmt::{Display, Formatter};
use std::ops::ControlFlow;
use std::time::{Instant, SystemTime};

pub static GUESS_PLAYERS: Lazy<Mutex<FxHashMap<u64, GuessPlayer>>> = Lazy::new(|| Mutex::new(FxHashMap::default()));
pub static COUNT_PLAYERS: Lazy<Mutex<CountPlayers>> = Lazy::new(|| Mutex::new(CountPlayers::default()));

pub struct GuessPlayer {
    pub tries: usize,
    pub chars: [Key; 3],
    pub freq_percent: f64,
    pub time: SystemTime,
}

impl GuessPlayer {
    pub fn new(chars: [Key; 3], freq_percent: f64) -> Self {
        let time = SystemTime::now();
        Self {
            tries: 0,
            chars,
            freq_percent,
            time,
        }
    }
}


#[derive(Default)]
pub struct CountPlayers {
    pub count: isize,
    pub last_use: Option<SystemTime>,
    pub players: FxHashMap<u64, CountPlayer>,
}

impl CountPlayers {
    pub const GOAL: usize = 50;
    pub const MIN_DELAY: u8 = 5;
    pub const MAX_DELAY: u8 = 15;
    fn is_game_over(&self) -> ControlFlow<()> {
        match self.count.unsigned_abs() >= Self::GOAL {
            true => ControlFlow::Break(()),
            false => ControlFlow::Continue(()),
        }
    }
    pub fn offset_points(&mut self, team: Team, points: isize) -> ControlFlow<()> {
        self.count += team as isize * points;
        self.is_game_over()
    }
    pub fn reset(&mut self) {
        self.count = 0;
        self.last_use = None;
        self.players.clear();
    }
}

pub struct CountPlayer {
    pub team: Team,
    pub time: Instant,
    pub target_sec: u8,
}

impl CountPlayer {
    const BASE_TOLERANCE: f64 = 0.06;
    pub fn get_points(&self, now: Instant) -> (isize, f64, Ordering) {
        let waited_sec = now.duration_since(self.time).as_secs_f64();
        let target_sec = self.target_sec as f64;
        let error = (target_sec - waited_sec).abs();
        let error = (error - target_sec * Self::BASE_TOLERANCE).max(0.0);
        let points = match error {
            ..0.1 => 5,
            ..0.3 => 3,
            ..0.5 => 1,
            _ => if waited_sec > target_sec { 0 } else { -5 },
        };
        let order = if points == 5 { Ordering::Equal } else {
            waited_sec.total_cmp(&target_sec)
        };
        (points, waited_sec, order)
    }
}

#[derive(Copy, Clone)]
#[repr(isize)]
pub enum Team {
    Plus = 1,
    Minus = -1,
}

impl Team {
    pub fn random(rng: &mut StdRng) -> Self {
        *[Self::Plus, Self::Minus].choose(rng).unwrap()  // options are not empty
    }
}

impl Display for Team {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let sign = match self {
            Team::Plus => '+',
            Team::Minus => '-',
        };
        write!(f, "{sign}")
    }
}