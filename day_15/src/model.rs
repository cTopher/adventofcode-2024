use grid::{Direction, Grid, Position};
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
}

impl Warehouse {
    pub fn boxes(&self) -> impl Iterator<Item = Position> + '_ {
        self.map
            .enumerate()
            .filter(|(_, tile)| *tile == Tile::Box)
            .map(|(pos, _)| pos)
    }

    pub fn do_moves(&mut self) {
        for i in 0..self.moves.len() {
            self.do_move(self.moves[i]);
        }
    }

    fn do_move(&mut self, direction: Direction) {
        let target = self.robot + direction;
        match self.map[target] {
            Tile::Empty => {
                self.robot = target;
            }
            Tile::Wall => {}
            Tile::Box => {
                let mut box_target = target;
                loop {
                    box_target += direction;
                    match self.map[box_target] {
                        Tile::Empty => {
                            self.map[target] = Tile::Empty;
                            self.map[box_target] = Tile::Box;
                            self.robot = target;
                            return;
                        }
                        Tile::Wall => {
                            return;
                        }
                        Tile::Box => {}
                    }
                }
            }
        }
    }
}

impl FromStr for Warehouse {
    type Err = !;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (map, moves) = s.split_once("\n\n").unwrap();
        let mut robot = Position::ZERO;
        let map = Grid::from_str_per_enumerated_char(map, |tile, position| match tile {
            '@' => {
                robot = position;
                Tile::Empty
            }
            'O' => Tile::Box,
            '#' => Tile::Wall,
            '.' => Tile::Empty,
            _ => panic!("Invalid tile: {tile}"),
        });
        let moves = moves
            .chars()
            .filter(|c| *c != '\n')
            .map(Direction::orthogonal_from_arrow)
            .collect();
        Ok(Self { robot, map, moves })
    }
}

#[must_use]
pub const fn gps_coordinates(position: Position) -> usize {
    100 * position.i + position.j
}
