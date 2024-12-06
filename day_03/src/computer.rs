use regex::Regex;
use std::str::FromStr;

pub struct Computer {
    memory: String,
}

impl Computer {
    pub fn calculate_sum_of_multiplications(&self) -> u32 {
        let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
        re.captures_iter(&self.memory)
            .map(|caps| {
                let a: u32 = caps[1].parse().unwrap();
                let b: u32 = caps[2].parse().unwrap();
                a * b
            })
            .sum()
    }
}

impl FromStr for Computer {
    type Err = !;

    fn from_str(s: &str) -> Result<Self, !> {
        let memory = s.to_string();
        Ok(Self { memory })
    }
}
