use regex::Regex;
use std::str::FromStr;

pub struct Computer {
    memory: String,
    mul_regex: Regex,
}

impl Computer {
    fn new(memory: String) -> Self {
        let mul_regex = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
        Self { memory, mul_regex }
    }

    pub fn calculate_sum_of_multiplications(&self) -> u32 {
        self.sum_of_multiplications(&self.memory)
    }

    pub fn calculate(&self) -> u32 {
        self.memory
            .split("do()")
            .map(|s| self.sum_of_multiplications(s.split("don't()").next().unwrap()))
            .sum()
    }

    fn sum_of_multiplications(&self, s: &str) -> u32 {
        self.mul_regex
            .captures_iter(s)
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
        Ok(Self::new(s.to_string()))
    }
}
