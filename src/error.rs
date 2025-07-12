use std::fmt::{Debug, Formatter};
use std::io;

pub enum Signal {
    AdminRestart,
    ForceEnd,
}

pub enum BotError {
    Io(io::Error),
    CtrlC,
}

impl Debug for BotError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            BotError::Io(err) => write!(f, "{err:?}"),
            BotError::CtrlC => write!(f, "The program was terminated by ctrl-c"),
        }
    }
}

impl From<io::Error> for BotError {
    fn from(err: io::Error) -> Self {
        BotError::Io(err)
    }
}