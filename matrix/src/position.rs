use crate::Direction;
use std::ops::{Add, Sub};

#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
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

    #[must_use]
    pub fn checked_sub(self, Direction { di, dj }: Direction) -> Option<Self> {
        let i = self.i.checked_sub_signed(di)?;
        let j = self.j.checked_sub_signed(dj)?;
        Some(Self { i, j })
    }

    pub fn neighbours(self) -> impl Iterator<Item = Self> {
        Direction::ORTHOGONAL
            .into_iter()
            .filter_map(move |direction| self.checked_add(direction))
    }
}

impl Add<Direction> for Position {
    type Output = Self;

    fn add(self, Direction { di, dj }: Direction) -> Self {
        let i = self.i.strict_add_signed(di);
        let j = self.j.strict_add_signed(dj);
        Self { i, j }
    }
}

impl Sub<Self> for Position {
    type Output = Direction;

    fn sub(self, rhs: Self) -> Self::Output {
        let di = self.i.checked_signed_diff(rhs.i).unwrap();
        let dj = self.j.checked_signed_diff(rhs.j).unwrap();
        Direction { di, dj }
    }
}
