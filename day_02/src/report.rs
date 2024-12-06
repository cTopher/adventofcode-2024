use std::str::FromStr;

pub struct Reports {
    reports: Vec<Report>,
}

pub struct Report {
    levels: Vec<u32>,
}

impl Reports {
    pub fn count(&self, mut predicate: impl FnMut(&Report) -> bool) -> usize {
        self.reports
            .iter()
            .filter(|&report| predicate(report))
            .count()
    }
}

impl Report {
    pub fn is_safe(&self) -> bool {
        is_safe(self.levels.iter().copied())
    }

    pub fn is_safe_using_problem_dampener(&self) -> bool {
        (0..self.levels.len()).any(|tolerate_index| {
            is_safe(
                self.levels
                    .iter()
                    .enumerate()
                    .filter(|&(index, _)| index != tolerate_index)
                    .map(|(_, &level)| level),
            )
        })
    }
}

fn is_safe(mut levels: impl Iterator<Item = u32>) -> bool {
    let start = levels.next().unwrap();
    let mut a = levels.next().unwrap();
    let Some(direction) = SafeDirection::from(start, a) else {
        return false;
    };
    for b in levels {
        if SafeDirection::from(a, b) != Some(direction) {
            return false;
        }
        a = b;
    }
    true
}

#[derive(Debug, PartialEq, Copy, Clone, Eq)]
enum SafeDirection {
    Rising,
    Decreasing,
}

impl SafeDirection {
    fn from(a: u32, b: u32) -> Option<Self> {
        let diff = b.checked_signed_diff(a)?;
        if (1..=3).contains(&diff) {
            Some(Self::Rising)
        } else if (-3..=-1).contains(&diff) {
            Some(Self::Decreasing)
        } else {
            None
        }
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
