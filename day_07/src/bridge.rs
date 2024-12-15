use std::str::FromStr;

pub struct Bridge {
    equations: Vec<Equation>,
}

struct Equation {
    test_value: u64,
    numbers: Vec<u64>,
}

impl Bridge {
    pub fn calibration_result(&self) -> u64 {
        self.equations
            .iter()
            .filter(|e| e.could_be_true())
            .map(|e| e.test_value)
            .sum()
    }
}

impl Equation {
    fn could_be_true(&self) -> bool {
        let mut values = vec![self.numbers[0]];
        for &number in &self.numbers[1..] {
            let add = values.iter().map(|v| v + number);
            let mul = values.iter().map(|v| v * number);
            values = add.chain(mul).filter(|&v| v <= self.test_value).collect();
        }
        values.contains(&self.test_value)
    }
}

impl FromStr for Bridge {
    type Err = !;

    fn from_str(s: &str) -> Result<Self, !> {
        let equations = s.lines().map(|l| l.parse().unwrap()).collect();
        Ok(Self { equations })
    }
}

impl FromStr for Equation {
    type Err = !;

    fn from_str(s: &str) -> Result<Self, !> {
        let (left, right) = s.split_once(':').unwrap();
        let test_value = left.parse().unwrap();
        let numbers = right
            .split_whitespace()
            .map(|n| n.parse().unwrap())
            .collect();
        Ok(Self {
            test_value,
            numbers,
        })
    }
}
