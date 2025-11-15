#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Default)]
pub struct Coordinate<const N: usize> {
    pub x: usize,
    pub y: usize,
}

impl<const N: usize> Coordinate<N> {
    pub const ZERO: Self = Self { x: 0, y: 0 };

    #[must_use]
    pub const fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }

    #[must_use]
    pub const fn up(self) -> Option<Self> {
        if self.y == 0 {
            None
        } else {
            Some(Self::new(self.x, self.y - 1))
        }
    }

    #[must_use]
    pub const fn right(self) -> Option<Self> {
        let x = self.x + 1;
        if x == N {
            None
        } else {
            Some(Self::new(x, self.y))
        }
    }

    #[must_use]
    pub const fn down(self) -> Option<Self> {
        let y = self.y + 1;
        if y == N {
            None
        } else {
            Some(Self::new(self.x, y))
        }
    }

    #[must_use]
    pub const fn left(self) -> Option<Self> {
        if self.x == 0 {
            None
        } else {
            Some(Self::new(self.x - 1, self.y))
        }
    }

    pub fn neighbours(self) -> impl Iterator<Item = Self> {
        [self.up(), self.right(), self.down(), self.left()]
            .into_iter()
            .flatten()
    }
}
