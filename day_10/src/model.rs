use matrix::{Matrix, Position};
use std::str::FromStr;

pub struct LavaIsland {
    map: Matrix<u8>,
}

impl LavaIsland {
    pub fn total_trailheads_score(&self) -> usize {
        self.map
            .enumerate()
            .filter(|&(_, height)| height == 0)
            .map(|(position, _)| self.trailhead_score(position))
            .sum()
    }

    fn trailhead_score(&self, trailhead: Position) -> usize {
        let mut positions = vec![trailhead];
        for height in 1..=9 {
            positions = positions
                .into_iter()
                .flat_map(|position| {
                    position
                        .neighbours()
                        .filter(|&position| self.map.get(position) == Some(height))
                })
                .collect();
            positions.sort_unstable();
            positions.dedup();
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
