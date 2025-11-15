use fixed_grid::{Coordinate, Grid};
use std::collections::VecDeque;
use std::str::FromStr;

pub struct Computer<const N: usize> {
    incoming_byte_positions: Vec<Coordinate<N>>,
    memory_space: Grid<ByteState, N>,
}

#[derive(Copy, Clone, Default, Eq, PartialEq)]
enum ByteState {
    #[default]
    Safe,
    Corrupted,
}

#[derive(Copy, Clone, Default, Eq, PartialEq, Debug)]
pub struct Path<const N: usize> {
    pub steps: u32,
    position: Coordinate<N>,
}

impl<const N: usize> Computer<N> {
    const EXIT: Coordinate<N> = Coordinate::new(N - 1, N - 1);

    pub fn simulate(&mut self, size: usize) {
        for &position in self.incoming_byte_positions.iter().take(size) {
            self.memory_space[position] = ByteState::Corrupted;
        }
    }

    #[must_use]
    pub fn shortest_safe_path(&self) -> Path<N> {
        let mut visited = Grid::<bool, N>::default();
        let mut paths = VecDeque::new();
        paths.push_back(Path::default());
        while let Some(path) = paths.pop_front() {
            if visited[path.position] {
                continue;
            }
            if path.position == Self::EXIT {
                return path;
            }
            visited[path.position] = true;
            path.position
                .neighbours()
                .filter(|&neighbour| self.memory_space[neighbour] == ByteState::Safe)
                .for_each(|neighbour| {
                    paths.push_back(Path {
                        steps: path.steps + 1,
                        position: neighbour,
                    });
                });
        }
        panic!("No safe path found");
    }
}

impl<const N: usize> FromStr for Computer<N> {
    type Err = !;

    fn from_str(s: &str) -> Result<Self, !> {
        Ok(Self {
            incoming_byte_positions: s.lines().map(parse_position).collect(),
            memory_space: Grid::default(),
        })
    }
}
fn parse_position<const N: usize>(s: &str) -> Coordinate<N> {
    let (x, y) = s.split_once(',').unwrap();
    Coordinate {
        x: x.parse().unwrap(),
        y: y.parse().unwrap(),
    }
}
