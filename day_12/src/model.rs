use matrix::{Direction, Matrix, Position};
use std::collections::HashSet;
use std::str::FromStr;

pub struct Map {
    plants: Matrix<char>,
}

struct Plot {
    positions: HashSet<Position>,
    perimeter: usize,
}

impl Map {
    #[must_use]
    pub fn total_price(&self) -> usize {
        let mut todo: HashSet<_> = self.plants.positions().collect();
        let mut price = 0;
        while let Some(&pos) = todo.iter().next() {
            let plot = self.plot_at(pos);
            price += plot.price();
            for pos in &plot.positions {
                todo.remove(pos);
            }
        }
        price
    }

    fn plot_at(&self, position: Position) -> Plot {
        let mut perimeter = 0;
        let name = self.plants.get(position).unwrap();
        let mut positions = HashSet::new();
        let mut todo = vec![position];
        positions.insert(position);
        while let Some(position) = todo.pop() {
            for direction in Direction::ORTHOGONAL {
                if let Some(neighbour) = position.checked_add(direction)
                    && self.plants.get(neighbour) == Some(name)
                {
                    if positions.insert(neighbour) {
                        todo.push(neighbour);
                    }
                } else {
                    perimeter += 1;
                }
            }
        }
        Plot {
            positions,
            perimeter,
        }
    }
}

impl Plot {
    fn price(&self) -> usize {
        self.positions.len() * self.perimeter
    }
}

impl FromStr for Map {
    type Err = !;

    fn from_str(s: &str) -> Result<Self, !> {
        let Ok(plants) = s.parse();
        Ok(Self { plants })
    }
}
