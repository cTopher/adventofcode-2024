use crate::Direction;
use std::ops::Add;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Position {
    pub i: usize,
    pub j: usize,
}

impl Position {
    pub const ZERO: Self = Self { i: 0, j: 0 };

    #[must_use]
    pub const fn saturating_add(self, Direction { di, dj }: Direction) -> Self {
        let i = self.i.saturating_add_signed(di);
        let j = self.j.saturating_add_signed(dj);
        Self { i, j }
    }

    #[must_use]
    pub fn checked_add(self, Direction { di, dj }: Direction) -> Option<Self> {
        let i = self.i.checked_add_signed(di)?;
        let j = self.j.checked_add_signed(dj)?;
        Some(Self { i, j })
    }
}

impl Add<Direction> for Position {
    type Output = Self;

    fn add(self, direction: Direction) -> Self {
        self.checked_add(direction).unwrap()
    }
}
