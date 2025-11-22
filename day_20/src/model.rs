use grid::{Direction, Grid};
use std::str::FromStr;

pub struct RaceTrack {
    map: Grid<Tile>,
}

impl RaceTrack {
    const CHEAT_MOVES: [Direction; 8] = [
        Direction::new(2, 0),
        Direction::new(1, -1),
        Direction::new(0, -2),
        Direction::new(-1, -1),
        Direction::new(-2, 0),
        Direction::new(-1, 1),
        Direction::new(0, 2),
        Direction::new(1, 1),
    ];

    #[must_use]
    pub fn count_cheats(&self, threshold: u32) -> usize {
        let times = self.run_picoseconds();
        times
            .enumerate()
            .filter_map(|(position, time)| time.map(|time| (position, time)))
            .map(|(p0, t0)| {
                Self::CHEAT_MOVES
                    .iter()
                    .filter_map(|&movement| p0.checked_add(movement))
                    .filter_map(|p2| times.get(p2).flatten())
                    .filter(|t2| t2.saturating_sub(2).saturating_sub(t0) >= threshold)
                    .count()
            })
            .sum()
    }

    fn run_picoseconds(&self) -> Grid<Option<u32>> {
        let mut times = self.map.map(|_| None);
        let mut current = self.map.position(Tile::End).unwrap();
        let mut previous = current;
        let mut time = 0;
        loop {
            times[current] = Some(time);
            time += 1;
            if let Some(next) = current
                .neighbours()
                .find(|&next| self.map[next] == Tile::Track && next != previous)
            {
                previous = current;
                current = next;
            } else {
                let start = current
                    .neighbours()
                    .find(|&position| self.map[position] == Tile::Start)
                    .unwrap();
                times[start] = Some(time);
                return times;
            }
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Tile {
    Track,
    Wall,
    Start,
    End,
}

impl From<char> for Tile {
    fn from(c: char) -> Self {
        match c {
            '.' => Self::Track,
            '#' => Self::Wall,
            'S' => Self::Start,
            'E' => Self::End,
            _ => panic!("Unknown tile character: {c}"),
        }
    }
}

impl FromStr for RaceTrack {
    type Err = !;

    fn from_str(s: &str) -> Result<Self, !> {
        let map = Grid::from_str(s)?;
        Ok(Self { map })
    }
}
