use matrix::{Direction, Matrix, Position};
use std::str::FromStr;

pub struct Lab {
    map: Matrix<Tile>,
    guard: Guard,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Guard {
    position: Position,
    direction: Direction,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Tile {
    Empty,
    Obstruction,
}

impl Lab {
    pub fn guard_path(&self) -> Vec<Position> {
        let mut path = vec![self.guard.position];
        let mut guard = self.guard;
        loop {
            let Some(forward) = guard.forward() else {
                break;
            };
            match self.map.get(forward) {
                Some(Tile::Obstruction) => guard.turn_right(),
                Some(Tile::Empty) => {
                    guard.position = forward;
                    path.push(guard.position);
                }
                None => break,
            }
            // if guard == self.guard {
            //     break;
            // }
        }
        path
    }
}

impl Guard {
    fn forward(&self) -> Option<Position> {
        self.position.checked_add(self.direction)
    }

    fn turn_right(&mut self) {
        self.direction = self.direction.turn_right();
    }
}

impl FromStr for Lab {
    type Err = !;

    fn from_str(s: &str) -> Result<Self, !> {
        let mut guard = Guard {
            position: Position::ZERO,
            direction: Direction::UP,
        };
        let mut map = Vec::new();
        for (i, line) in s.lines().enumerate() {
            let mut row = Vec::new();
            for (j, c) in line.chars().enumerate() {
                row.push(match c {
                    '#' => Tile::Obstruction,
                    '^' => {
                        guard.position = Position { i, j };
                        Tile::Empty
                    }
                    '.' => Tile::Empty,
                    _ => panic!("Invalid map character: {c}"),
                });
            }
            map.push(row);
        }
        Ok(Self {
            map: Matrix::new(map),
            guard,
        })
    }
}
