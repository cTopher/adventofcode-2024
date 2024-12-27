use grid::{Direction, DirectionSet, Grid, Position};
use std::str::FromStr;

pub struct Lab {
    map: Grid<Tile>,
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
        let mut guard = self.guard;
        let mut path = vec![guard.position];
        let mut states = self.map.map(|_| DirectionSet::empty());
        states[guard.position].insert(guard.direction);
        loop {
            let Some(forward) = guard.forward() else {
                break;
            };
            match self.map.get(forward) {
                Some(Tile::Obstruction) => guard.turn_right(),
                Some(Tile::Empty) => {
                    guard.position = forward;
                }
                None => break,
            }
            let visited = &mut states[guard.position];
            if visited.is_empty() {
                path.push(guard.position);
            }
            if !visited.insert(guard.direction) {
                break;
            }
        }
        path
    }

    pub fn looping_obstruction_count(&self) -> usize {
        self.guard_path()
            .into_iter()
            .filter(|&position| position != self.guard.position)
            .filter(|&position| self.obstruction_causes_loop(position))
            .count()
    }

    fn obstruction_causes_loop(&self, obstruction: Position) -> bool {
        let mut states = self.map.map(|_| DirectionSet::empty());
        let mut guard = self.guard;
        states[guard.position].insert(guard.direction);
        loop {
            let Some(forward) = guard.forward() else {
                return false;
            };
            if forward == obstruction {
                guard.turn_right();
            } else {
                match self.map.get(forward) {
                    Some(Tile::Obstruction) => guard.turn_right(),
                    Some(Tile::Empty) => {
                        guard.position = forward;
                    }
                    None => return false,
                }
            }
            if !states[guard.position].insert(guard.direction) {
                return true;
            }
        }
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
            map: Grid::new(map),
            guard,
        })
    }
}
