use grid::{Direction, DirectionMap, Grid, Position};
use std::collections::BinaryHeap;
use std::fmt;
use std::fmt::Formatter;
use std::rc::Rc;
use std::str::FromStr;

pub struct Maze {
    grid: Grid<Tile>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Tile {
    Empty,
    Wall,
    Start,
    End,
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Path {
    inner: Rc<InnerPath>,
}
#[derive(Debug, PartialEq, Eq)]
struct InnerPath {
    pub score: u32,
    position: Position,
    direction: Direction,
    prev: Option<Rc<Self>>,
}

impl Path {
    fn new(position: Position) -> Self {
        let inner = Rc::new(InnerPath {
            score: 0,
            position,
            direction: Direction::EAST,
            prev: None,
        });
        Self { inner }
    }

    #[must_use]
    pub fn score(&self) -> u32 {
        self.inner.score
    }

    #[must_use]
    pub fn position(&self) -> Position {
        self.inner.position
    }

    #[must_use]
    pub fn direction(&self) -> Direction {
        self.inner.direction
    }

    fn forward(&self) -> Self {
        let inner = Rc::new(InnerPath {
            score: self.score() + 1,
            position: self.position() + self.direction(),
            direction: self.direction(),
            prev: Some(self.inner.clone()),
        });
        Self { inner }
    }

    fn rotate_clockwise(&self) -> Self {
        let inner = Rc::new(InnerPath {
            score: self.score() + 1000,
            position: self.position(),
            direction: self.direction().turn_right(),
            prev: Some(self.inner.clone()),
        });
        Self { inner }
    }

    fn rotate_counterclockwise(&self) -> Self {
        let inner = Rc::new(InnerPath {
            score: self.score() + 1000,
            position: self.position(),
            direction: self.direction().turn_left(),
            prev: Some(self.inner.clone()),
        });
        Self { inner }
    }

    fn for_each_position(&self, mut f: impl FnMut(Position)) {
        let mut inner = &self.inner;
        f(inner.position);
        while let Some(prev) = &inner.prev {
            inner = prev;
            f(inner.position);
        }
    }
}

impl Maze {
    #[must_use]
    pub fn best_path(&self) -> Path {
        let mut scores = self.grid.map(|_| DirectionMap::with_value(u32::MAX));
        let mut paths = BinaryHeap::new();
        paths.push(self.start_path());
        while let Some(path) = paths.pop() {
            match self.grid[path.position()] {
                Tile::Start | Tile::Empty => {
                    if path.score() >= scores[path.position()].get(path.direction()) {
                        continue;
                    }
                    scores[path.position()].set(path.direction(), path.score());
                    paths.push(path.forward());
                    paths.push(path.rotate_counterclockwise());
                    paths.push(path.rotate_clockwise());
                }
                Tile::Wall => {}
                Tile::End => {
                    return path;
                }
            }
        }
        panic!("No path found to the end tile");
    }

    #[must_use]
    pub fn count_best_spots(&self) -> usize {
        let mut is_best_spot = self.grid.map(|_| false);
        let mut scores = self.grid.map(|_| DirectionMap::with_value(u32::MAX));
        let mut end_score = u32::MAX;
        let mut paths = BinaryHeap::new();
        paths.push(self.start_path());
        while let Some(path) = paths.pop() {
            match self.grid[path.position()] {
                Tile::Start | Tile::Empty => {
                    if path.score() > scores[path.position()].get(path.direction()) {
                        continue;
                    }
                    scores[path.position()].set(path.direction(), path.score());
                    paths.push(path.forward());
                    paths.push(path.rotate_clockwise());
                    paths.push(path.rotate_counterclockwise());
                }
                Tile::Wall => {}
                Tile::End => {
                    if path.score() > end_score {
                        break;
                    }
                    end_score = path.score();
                    path.for_each_position(|position| {
                        is_best_spot[position] = true;
                    });
                }
            }
        }
        is_best_spot.iter().filter(|&x| x).count()
    }

    fn start_path(&self) -> Path {
        Path::new(self.grid.position(Tile::Start).unwrap())
    }
}

impl From<char> for Tile {
    fn from(c: char) -> Self {
        match c {
            '.' => Self::Empty,
            '#' => Self::Wall,
            'S' => Self::Start,
            'E' => Self::End,
            _ => panic!("Invalid tile character: {c}"),
        }
    }
}

impl Ord for Path {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.score().cmp(&self.score())
    }
}

impl PartialOrd for Path {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl FromStr for Maze {
    type Err = !;

    fn from_str(s: &str) -> Result<Self, !> {
        let grid: Grid<Tile> = s.parse()?;
        Ok(Self { grid })
    }
}

impl fmt::Display for Tile {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let c = match self {
            Self::Empty => '.',
            Self::Wall => '#',
            Self::Start => 'S',
            Self::End => 'E',
        };
        write!(f, "{c}")
    }
}
