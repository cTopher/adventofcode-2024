#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct Direction {
    pub di: isize,
    pub dj: isize,
}

impl Direction {
    pub const UP: Self = Self { di: -1, dj: 0 };
    pub const RIGHT: Self = Self { di: 0, dj: 1 };
    pub const DOWN: Self = Self { di: 1, dj: 0 };
    pub const LEFT: Self = Self { di: 0, dj: -1 };

    pub const ALL: [Self; 8] = [
        Self { di: -1, dj: -1 },
        Self::UP,
        Self { di: -1, dj: 1 },
        Self::RIGHT,
        Self { di: 1, dj: 1 },
        Self::DOWN,
        Self { di: 1, dj: -1 },
        Self::LEFT,
    ];

    #[must_use]
    pub const fn new(di: isize, dj: isize) -> Self {
        Self { di, dj }
    }

    #[must_use]
    pub const fn turn_right(self) -> Self {
        Self {
            di: self.dj,
            dj: -self.di,
        }
    }
}
