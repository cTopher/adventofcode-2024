use grid::Grid;
use std::str::FromStr;

pub struct RaceTrack {
    map: Grid<Tile>,
}

impl RaceTrack {
    #[must_use]
    pub fn count_cheats(&self, duration: usize, picoseconds_saved: usize) -> usize {
        let times = self.run_picoseconds();
        times
            .enumerate()
            .filter_map(|(position, time)| time.map(|time| (position, time)))
            .map(|(p0, t0)| {
                times
                    .enumerate()
                    .filter_map(|(p1, t1)| t1.map(|t1| (p1, t1)))
                    .filter(|&(p1, t1)| {
                        let distance = p0.distance_to(p1);
                        distance <= duration
                            && t1.saturating_sub(distance).saturating_sub(t0) >= picoseconds_saved
                    })
                    .count()
            })
            .sum()
    }

    fn run_picoseconds(&self) -> Grid<Option<usize>> {
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
