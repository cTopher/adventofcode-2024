use std::collections::HashMap;
use std::mem;
use std::str::FromStr;

#[derive(Default, Debug)]
pub struct Stones {
    counts: HashMap<u64, u64>,
}

impl Stones {
    pub fn blink_times(&mut self, times: usize) {
        for _ in 0..times {
            self.blink();
        }
    }

    fn blink(&mut self) {
        for (engraving, amount) in mem::take(&mut self.counts) {
            if engraving == 0 {
                self.push(1, amount);
            } else {
                let digits = engraving.ilog10() + 1;
                if digits % 2 == 0 {
                    let nom = 10u64.pow(digits / 2);
                    let left = engraving / nom;
                    let right = engraving % nom;
                    self.push(left, amount);
                    self.push(right, amount);
                } else {
                    self.push(engraving * 2024, amount);
                }
            }
        }
    }

    fn push(&mut self, engraving: u64, amount: u64) {
        self.counts
            .entry(engraving)
            .and_modify(|a| *a += amount)
            .or_insert(amount);
    }

    pub fn amount(&self) -> u64 {
        self.counts.values().sum()
    }
}

impl FromStr for Stones {
    type Err = !;

    fn from_str(s: &str) -> Result<Self, !> {
        let mut stones = Self::default();
        for engraving in s.split_whitespace() {
            stones.push(engraving.parse().unwrap(), 1);
        }
        Ok(stones)
    }
}
