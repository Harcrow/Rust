use std::ops::{BitAnd, BitOr, Mul};

pub struct Board(pub u64);

impl Board {
    pub fn is_empy(&self) -> bool {
        self.0 == 0
    }
    pub fn any(self) -> bool {
        self.0 != 0
    }
    pub fn empty() -> Self {
        Self(0)
    }
}
