#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub struct Direction {
    pub di: isize,
    pub dj: isize,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[allow(clippy::struct_excessive_bools)]
pub struct DirectionSet {
    up: bool,
    right: bool,
    down: bool,
    left: bool,
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

    pub const ORTHOGONAL: [Self; 4] = [Self::UP, Self::RIGHT, Self::DOWN, Self::LEFT];

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

    #[must_use]
    pub const fn turn_left(self) -> Self {
        Self {
            di: -self.dj,
            dj: self.di,
        }
    }
}

impl DirectionSet {
    #[must_use]
    pub const fn empty() -> Self {
        Self {
            up: false,
            right: false,
            down: false,
            left: false,
        }
    }

    pub fn insert(&mut self, direction: Direction) -> bool {
        let value = match direction {
            Direction::UP => &mut self.up,
            Direction::RIGHT => &mut self.right,
            Direction::DOWN => &mut self.down,
            Direction::LEFT => &mut self.left,
            _ => unimplemented!("Diagonal directions are not supported"),
        };
        let result = !*value;
        *value = true;
        result
    }

    #[must_use]
    pub const fn is_empty(&self) -> bool {
        !self.up && !self.right && !self.down && !self.left
    }
}
