use grid::{Grid, Position};
use std::str::FromStr;

pub struct City {
    pub antennas: Grid<Option<char>>,
}

const ANTENNA_START: usize = '0' as usize;
const ANTENNA_END: usize = 'z' as usize;

impl City {
    pub fn antinodes(&self) -> Grid<bool> {
        let mut antinodes = self.antennas.map(|_| false);
        for positions in self.antenna_positions() {
            for (i, &a) in positions.iter().enumerate() {
                for &b in &positions[i + 1..] {
                    let dir = b - a;
                    if let Some(x) = a.checked_sub(dir)
                        && let Some(is_antinode) = antinodes.get_mut(x)
                    {
                        *is_antinode = true;
                    }
                    if let Some(x) = b.checked_add(dir)
                        && let Some(is_antinode) = antinodes.get_mut(x)
                    {
                        *is_antinode = true;
                    }
                }
            }
        }
        antinodes
    }

    pub fn resonant_harmonic_antinodes(&self) -> Grid<bool> {
        let mut antinodes = self.antennas.map(|antenna| antenna.is_some());
        for positions in self.antenna_positions() {
            for (i, &a) in positions.iter().enumerate() {
                for &b in &positions[i + 1..] {
                    let dir = b - a;
                    let mut a = a;
                    let mut b = b;
                    while let Some(x) = a.checked_sub(dir) {
                        a = x;
                        if let Some(is_antinode) = antinodes.get_mut(a) {
                            *is_antinode = true;
                        } else {
                            break;
                        }
                    }
                    while let Some(x) = b.checked_add(dir) {
                        b = x;
                        if let Some(is_antinode) = antinodes.get_mut(b) {
                            *is_antinode = true;
                        } else {
                            break;
                        }
                    }
                }
            }
        }
        antinodes
    }

    pub fn antenna_positions(&self) -> impl Iterator<Item = Vec<Position>> {
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
        let antennas = Grid::from_str_per_char(s, |c| if c == '.' { None } else { Some(c) });
        Ok(Self { antennas })
    }
}
