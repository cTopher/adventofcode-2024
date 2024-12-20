use std::str::FromStr;

pub struct Stones {
    stones: Vec<u32>,
}

impl Stones {
    pub fn blink(&mut self) {
        let mut result = Vec::with_capacity(self.stones.len());
        for &stone in &self.stones {
            if stone == 0 {
                result.push(1);
            } else {
                let digits = stone.ilog10() + 1;
                if digits % 2 == 0 {
                    let nom = 10u32.pow(digits / 2);
                    let left = stone / nom;
                    let right = stone % nom;
                    result.push(left);
                    result.push(right);
                } else {
                    result.push(stone * 2024);
                }
            }
        }
        self.stones = result;
    }

    pub fn amount(&self) -> usize {
        self.stones.len()
    }
}

impl FromStr for Stones {
    type Err = !;

    fn from_str(s: &str) -> Result<Self, !> {
        let stones = s.split_whitespace().map(|s| s.parse().unwrap()).collect();
        Ok(Self { stones })
    }
}
