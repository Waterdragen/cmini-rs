use crate::core::Finger;

pub type Row = u8;
pub type Col = u8;

pub type Key = char;

#[derive(Debug, Copy, Clone)]
pub struct Position {
    pub row: Row,
    pub col: Col,
    pub finger: Finger,
}

impl Position {
    pub fn new(row: Row, col: Col, finger: Finger) -> Self {
        Position { row, col, finger }
    }
}