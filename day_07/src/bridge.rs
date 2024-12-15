use std::iter::chain;
use std::str::FromStr;

pub struct Bridge {
    equations: Vec<Equation>,
}

struct Equation {
    test_value: u64,
    numbers: Vec<u64>,
}

impl Bridge {
    pub fn calibration_result_without_concat(&self) -> u64 {
        self.equations
            .iter()
            .filter(|e| e.could_be_true_without_concat())
            .map(|e| e.test_value)
            .sum()
    }

    pub fn calibration_result_with_concat(&self) -> u64 {
        self.equations
            .iter()
            .filter(|e| e.could_be_true_with_concat())
            .map(|e| e.test_value)
            .sum()
    }
}

impl Equation {
    fn could_be_true_without_concat(&self) -> bool {
        let mut values = vec![self.numbers[0]];
        for &number in &self.numbers[1..] {
            let add = values.iter().map(|v| v + number);
            let mul = values.iter().map(|v| v * number);
            values = chain(add, mul).filter(|&v| v <= self.test_value).collect();
        }
        values.contains(&self.test_value)
    }

    fn could_be_true_with_concat(&self) -> bool {
        let mut values = vec![self.numbers[0]];
        for &number in &self.numbers[1..] {
            let add = values.iter().map(|v| v + number);
            let mul = values.iter().map(|v| v * number);
            let concat = values.iter().map(|&v| concat(v, number));
            values = add
                .chain(mul)
                .chain(concat)
                .filter(|&v| v <= self.test_value)
                .collect();
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

const fn concat(a: u64, b: u64) -> u64 {
    a * 10u64.pow(b.ilog10() + 1) + b
}
