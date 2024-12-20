use matrix::{Matrix, Position};
use std::str::FromStr;

pub struct LavaIsland {
    map: Matrix<u8>,
}

impl LavaIsland {
    pub fn total_trailheads_score(&self) -> usize {
        self.trailheads()
            .map(|position| self.trailhead_score(position))
            .sum()
    }

    pub fn total_trailheads_rating(&self) -> usize {
        self.trailheads()
            .map(|position| self.trailhead_rating(position))
            .sum()
    }

    fn trailheads(&self) -> impl Iterator<Item = Position> + '_ {
        self.map
            .enumerate()
            .filter(|&(_, height)| height == 0)
            .map(|(position, _)| position)
    }

    fn neighbours_of_height(&self, position: Position, height: u8) -> Vec<Position> {
        position
            .neighbours()
            .filter(|&position| self.map.get(position) == Some(height))
            .collect()
    }

    fn trailhead_score(&self, trailhead: Position) -> usize {
        let mut positions = vec![trailhead];
        for height in 1..=9 {
            positions = positions
                .into_iter()
                .flat_map(|position| self.neighbours_of_height(position, height))
                .collect();
            positions.sort_unstable();
            positions.dedup();
        }
        positions.len()
    }

    fn trailhead_rating(&self, trailhead: Position) -> usize {
        let mut positions = vec![trailhead];
        for height in 1..=9 {
            positions = positions
                .into_iter()
                .flat_map(|position| self.neighbours_of_height(position, height))
                .collect();
        }
        positions.len()
    }
}

impl FromStr for LavaIsland {
    type Err = !;

    fn from_str(s: &str) -> Result<Self, !> {
        let map = s
            .lines()
            .map(|line| {
                line.chars()
                    .map(|c| c.to_digit(10).unwrap().try_into().unwrap())
                    .collect()
            })
            .collect();
        Ok(Self {
            map: Matrix::new(map),
        })
    }
}
