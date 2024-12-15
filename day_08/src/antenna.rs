use matrix::{Matrix, Position};
use std::str::FromStr;

pub struct City {
    antennas: Matrix<Option<char>>,
}

const ANTENNA_START: usize = '0' as usize;
const ANTENNA_END: usize = 'z' as usize;

impl City {
    pub fn count_antinodes(&self) -> usize {
        self.antinodes()
            .enumerate()
            .filter(|&(_, is_antinode)| is_antinode)
            .count()
    }

    fn antinodes(&self) -> Matrix<bool> {
        let mut antinodes = self.antennas.map(|_| false);
        for positions in self.antenna_positions() {
            for (i, &a) in positions.iter().enumerate() {
                for &b in &positions[i + 1..] {
                    let dir = b - a;
                    if let Some(x) = a.checked_sub(dir) {
                        if let Some(is_antinode) = antinodes.get_mut(x) {
                            *is_antinode = true;
                        }
                    }
                    if let Some(x) = b.checked_add(dir) {
                        if let Some(is_antinode) = antinodes.get_mut(x) {
                            *is_antinode = true;
                        }
                    }
                }
            }
        }
        antinodes
    }

    fn antenna_positions(&self) -> impl Iterator<Item = Vec<Position>> {
        let mut antenna_positions = [const { Vec::new() }; ANTENNA_END - ANTENNA_START + 1];
        self.antennas.enumerate().for_each(|(position, antenna)| {
            if let Some(antenna) = antenna {
                antenna_positions[antenna as usize - ANTENNA_START].push(position);
            }
        });
        antenna_positions
            .into_iter()
            .filter(|positions| !positions.is_empty())
    }
}

impl FromStr for City {
    type Err = !;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let antennas = s
            .lines()
            .map(|line| {
                line.chars()
                    .map(|c| if c == '.' { None } else { Some(c) })
                    .collect()
            })
            .collect();
        let antennas = Matrix::new(antennas);
        Ok(Self { antennas })
    }
}
