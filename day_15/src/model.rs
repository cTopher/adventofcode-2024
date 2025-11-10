use grid::{Direction, Grid, Position};
use std::fmt;
use std::fmt::Formatter;
use std::str::FromStr;

pub struct Warehouse {
    robot: Position,
    map: Grid<Tile>,
    moves: Vec<Direction>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Tile {
    Empty,
    Wall,
    Box,
    Robot,
    LeftBox,
    RightBox,
}

impl Warehouse {
    pub fn resize(&mut self) {
        self.map = self
            .map
            .iter_rows()
            .map(|row| row.iter().flat_map(|&tile| tile.expand()).collect())
            .collect();
        self.robot.j *= 2;
    }

    pub fn boxes(&self) -> impl Iterator<Item = Position> + '_ {
        self.map
            .enumerate()
            .filter(|(_, tile)| *tile == Tile::Box || *tile == Tile::LeftBox)
            .map(|(pos, _)| pos)
    }

    pub fn do_moves(&mut self) {
        #[cfg(feature = "verbose")]
        println!("Initial state:");
        #[cfg(feature = "verbose")]
        println!("{}", self.map);
        for i in 0..self.moves.len() {
            let direction = self.moves[i];
            #[cfg(feature = "verbose")]
            println!("Move {}:", fmt_direction(direction));
            self.do_move(direction);
            #[cfg(feature = "verbose")]
            println!("{}", self.map);
        }
    }

    fn do_move(&mut self, direction: Direction) -> bool {
        let mut updates = Vec::new();
        let mut todo = Vec::new();
        todo.push((self.robot, Tile::Empty));
        while let Some((position, update)) = todo.pop() {
            updates.push((position, update));
            match self.map[position] {
                Tile::Empty => {}
                Tile::Wall => return false,
                Tile::LeftBox => {
                    todo.push((position + direction, Tile::LeftBox));
                    if direction.vertical() {
                        let right = position + Direction::RIGHT;
                        if updates.iter().all(|&(position, _)| position != right) {
                            todo.push((position + Direction::RIGHT, Tile::Empty));
                        }
                    }
                }
                Tile::RightBox => {
                    todo.push((position + direction, Tile::RightBox));
                    if direction.vertical() {
                        let left = position + Direction::LEFT;
                        if updates.iter().all(|&(position, _)| position != left) {
                            todo.push((position + Direction::LEFT, Tile::Empty));
                        }
                    }
                }
                tile => {
                    todo.push((position + direction, tile));
                }
            }
        }
        for (position, tile) in updates {
            self.map[position] = tile;
        }
        self.robot += direction;
        true
    }
}

impl From<char> for Tile {
    fn from(c: char) -> Self {
        match c {
            '.' => Self::Empty,
            '#' => Self::Wall,
            'O' => Self::Box,
            '@' => Self::Robot,
            _ => panic!("Invalid tile character: {c}"),
        }
    }
}

impl Tile {
    fn expand(self) -> [Self; 2] {
        match self {
            Self::Empty => [Self::Empty, Self::Empty],
            Self::Wall => [Self::Wall, Self::Wall],
            Self::Box => [Self::LeftBox, Self::RightBox],
            Self::Robot => [Self::Robot, Self::Empty],
            Self::LeftBox | Self::RightBox => {
                panic!("Cannot expand already expanded box");
            }
        }
    }
}

impl FromStr for Warehouse {
    type Err = !;

    fn from_str(s: &str) -> Result<Self, !> {
        let (map, moves) = s.split_once("\n\n").unwrap();
        let map: Grid<Tile> = map.parse()?;
        let moves = moves
            .chars()
            .filter(|c| *c != '\n')
            .map(Direction::orthogonal_from_arrow)
            .collect();
        let robot = map.position(Tile::Robot).unwrap();
        Ok(Self { robot, map, moves })
    }
}

#[must_use]
pub const fn gps_coordinates(position: Position) -> usize {
    100 * position.i + position.j
}

impl fmt::Display for Tile {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let c = match self {
            Self::Empty => '.',
            Self::Wall => '#',
            Self::Box => 'O',
            Self::Robot => '@',
            Self::LeftBox => '[',
            Self::RightBox => ']',
        };
        write!(f, "{c}")
    }
}

#[cfg(feature = "verbose")]
fn fmt_direction(direction: Direction) -> char {
    match direction {
        Direction::UP => '^',
        Direction::DOWN => 'v',
        Direction::LEFT => '<',
        Direction::RIGHT => '>',
        _ => panic!("Invalid direction: {direction:?}"),
    }
}
