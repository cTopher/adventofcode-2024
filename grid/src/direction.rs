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

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct DirectionMap<T> {
    up: T,
    right: T,
    down: T,
    left: T,
}

impl Direction {
    pub const UP: Self = Self { di: -1, dj: 0 };
    pub const RIGHT: Self = Self { di: 0, dj: 1 };
    pub const DOWN: Self = Self { di: 1, dj: 0 };
    pub const LEFT: Self = Self { di: 0, dj: -1 };

    pub const EAST: Self = Self::RIGHT;

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
    pub fn orthogonal_from_arrow(char: char) -> Self {
        match char {
            '^' => Self::UP,
            '>' => Self::RIGHT,
            'v' => Self::DOWN,
            '<' => Self::LEFT,
            _ => panic!("Invalid arrow char: {char}"),
        }
    }

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

    #[must_use]
    pub const fn horizontal(&self) -> bool {
        self.di == 0
    }

    #[must_use]
    pub const fn vertical(&self) -> bool {
        self.dj == 0
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

impl<T: Copy> DirectionMap<T> {
    pub const fn with_value(value: T) -> Self {
        Self {
            up: value,
            right: value,
            down: value,
            left: value,
        }
    }

    pub fn get(&self, direction: Direction) -> T {
        match direction {
            Direction::UP => self.up,
            Direction::RIGHT => self.right,
            Direction::DOWN => self.down,
            Direction::LEFT => self.left,
            _ => unimplemented!("Diagonal directions are not supported"),
        }
    }

    pub fn set(&mut self, direction: Direction, value: T) {
        match direction {
            Direction::UP => self.up = value,
            Direction::RIGHT => self.right = value,
            Direction::DOWN => self.down = value,
            Direction::LEFT => self.left = value,
            _ => unimplemented!("Diagonal directions are not supported"),
        }
    }
}
