use std::ops::Index;
use crate::core::{FingerCombo, Metric};

#[derive(Debug)]
pub struct Table([Metric; 1000]);

impl Table {
    pub fn from_inner(inner: [Metric; 1000]) -> Self {
        Self(inner)
    }
}

impl Index<FingerCombo<3>> for Table {
    type Output = Metric;

    fn index(&self, finger_combo: FingerCombo<3>) -> &Self::Output {
        &self.0[finger_combo.index()]
    }
}