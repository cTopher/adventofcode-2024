use matrix::{Direction, Matrix, Position};
use std::collections::HashSet;
use std::str::FromStr;

pub struct Map {
    plants: Matrix<char>,
}

pub struct Plot {
    positions: HashSet<Position>,
    fence: Vec<Fence>,
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
struct Fence {
    position: Position,
    direction: Direction,
}

impl Map {
    pub fn plots(&self) -> impl Iterator<Item = Plot> + '_ {
        let mut plotted = HashSet::<Position>::new();
        self.plants.enumerate().filter_map(move |(position, name)| {
            if plotted.contains(&position) {
                None
            } else {
                let plot = self.plot_at(position, name);
                plotted.extend(&plot.positions);
                Some(plot)
            }
        })
    }

    fn plot_at(&self, position: Position, name: char) -> Plot {
        let mut fence = Vec::new();
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
                    fence.push(Fence {
                        position,
                        direction,
                    });
                }
            }
        }
        Plot { positions, fence }
    }
}

impl Plot {
    #[must_use]
    pub fn price(&self) -> usize {
        self.positions.len() * self.fence.len()
    }

    #[must_use]
    pub fn bulk_discount_price(&self) -> usize {
        self.positions.len() * self.sides()
    }

    fn sides(&self) -> usize {
        let mut todo: HashSet<_> = self.fence.iter().collect();
        let mut sides = 0;
        while let Some(&&fence) = todo.iter().next() {
            todo.remove(&fence);
            sides += 1;
            let mut position = fence.position;
            let right = fence.direction.turn_right();
            while let Some(next) = position.checked_add(right)
                && todo.remove(&Fence {
                    position: next,
                    direction: fence.direction,
                })
            {
                position = next;
            }
            let mut position = fence.position;
            let left = fence.direction.turn_left();
            while let Some(next) = position.checked_add(left)
                && todo.remove(&Fence {
                    position: next,
                    direction: fence.direction,
                })
            {
                position = next;
            }
        }
        sides
    }
}

impl FromStr for Map {
    type Err = !;

    fn from_str(s: &str) -> Result<Self, !> {
        let Ok(plants) = s.parse();
        Ok(Self { plants })
    }
}
