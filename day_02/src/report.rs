use std::str::FromStr;

pub struct Reports {
    reports: Vec<Report>,
}

struct Report {
    levels: Vec<u32>,
}

impl Reports {
    pub fn count_safe(&self) -> usize {
        self.reports
            .iter()
            .filter(|report| report.is_safe())
            .count()
    }
}

impl Report {
    fn is_safe(&self) -> bool {
        if !self.levels.is_sorted() && !self.levels.iter().rev().is_sorted() {
            return false;
        }
        self.levels
            .windows(2)
            .map(|w| w[0].abs_diff(w[1]))
            .all(|diff| (1..=3).contains(&diff))
    }
}

impl FromStr for Reports {
    type Err = !;

    fn from_str(s: &str) -> Result<Self, !> {
        let reports = s.lines().map(|line| line.parse().unwrap()).collect();
        Ok(Self { reports })
    }
}

impl FromStr for Report {
    type Err = !;

    fn from_str(s: &str) -> Result<Self, !> {
        let levels = s.split(' ').map(|level| level.parse().unwrap()).collect();
        Ok(Self { levels })
    }
}
