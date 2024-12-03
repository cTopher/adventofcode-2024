use std::str::FromStr;

pub struct LocationLists {
    left: Vec<u32>,
    right: Vec<u32>,
}

impl LocationLists {
    pub fn sort(&mut self) {
        self.left.sort_unstable();
        self.right.sort_unstable();
    }

    pub fn total_distance(&self) -> u32 {
        self.left
            .iter()
            .zip(self.right.iter())
            .map(|(&left, &right)| left.abs_diff(right))
            .sum()
    }
}

impl FromStr for LocationLists {
    type Err = !;

    fn from_str(s: &str) -> Result<Self, !> {
        let (left, right) = s
            .lines()
            .map(|line| {
                let mut split = line
                    .split_whitespace()
                    .map(|location_id| u32::from_str(location_id).unwrap());
                let left = split.next().unwrap();
                let right = split.next().unwrap();
                assert_eq!(split.next(), None);
                (left, right)
            })
            .unzip();
        Ok(Self { left, right })
    }
}
