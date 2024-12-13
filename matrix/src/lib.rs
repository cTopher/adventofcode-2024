use std::ops::Add;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Matrix<T> {
    elements: Vec<Vec<T>>,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct Position {
    pub i: usize,
    pub j: usize,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct Direction {
    pub di: isize,
    pub dj: isize,
}

impl<T: Copy> Matrix<T> {
    #[must_use]
    pub const fn new(elements: Vec<Vec<T>>) -> Self {
        Self { elements }
    }

    #[must_use]
    pub fn get(&self, Position { i, j }: Position) -> Option<T> {
        self.elements.get(i).and_then(|row| row.get(j).copied())
    }

    pub fn enumerate(&self) -> impl Iterator<Item = (Position, T)> + '_ {
        self.elements.iter().enumerate().flat_map(|(i, row)| {
            row.iter()
                .enumerate()
                .map(move |(j, &elem)| (Position { i, j }, elem))
        })
    }
}

impl<T: Copy> From<Vec<Vec<T>>> for Matrix<T> {
    fn from(elements: Vec<Vec<T>>) -> Self {
        Self::new(elements)
    }
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

impl Add<Direction> for Position {
    type Output = Self;

    fn add(self, direction: Direction) -> Self {
        self.checked_add(direction).unwrap()
    }
}
