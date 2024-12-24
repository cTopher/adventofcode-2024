use std::str::FromStr;

pub struct Computer {
    memory: String,
}

impl Computer {
    pub fn calculate_sum_of_multiplications(&self) -> u32 {
        sum_of_multiplications(&self.memory)
    }
    pub fn calculate(&self) -> u32 {
        self.memory
            .split("do()")
            .map(|s| sum_of_multiplications(s.split("don't()").next().unwrap()))
            .sum()
    }
}

fn sum_of_multiplications(s: &str) -> u32 {
    s.match_indices("mul(")
        .filter_map(|(index, _)| {
            let start = index + 4;
            let len = s[start..].find(')')?;
            let mut parts = s[start..start + len].split(',');
            let a: u32 = parts.next()?.parse().ok()?;
            let b: u32 = parts.next()?.parse().ok()?;
            if parts.next().is_some() {
                None
            } else {
                Some(a * b)
            }
        })
        .sum()
}

impl FromStr for Computer {
    type Err = !;

    fn from_str(s: &str) -> Result<Self, !> {
        let memory = s.to_string();
        Ok(Self { memory })
    }
}
