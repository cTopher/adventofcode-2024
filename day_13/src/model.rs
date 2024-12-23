use math_2d::{Matrix, Vector};
use std::str::FromStr;

pub struct Arcade {
    machines: Vec<Machine>,
}

#[derive(Debug)]
struct Machine {
    a: Vector,
    b: Vector,
    price: Vector,
}

impl Arcade {
    pub fn tokens_to_win_all(&self) -> u64 {
        self.machines
            .iter()
            .filter_map(Machine::tokens_to_win)
            .sum()
    }

    pub fn increase_price(&mut self, increase: u64) {
        #[allow(clippy::cast_precision_loss)]
        let increase = increase as f64;
        for machine in &mut self.machines {
            machine.price.x += increase;
            machine.price.y += increase;
        }
    }
}

impl Machine {
    pub fn tokens_to_win(&self) -> Option<u64> {
        let m = Matrix::from_columns(self.a, self.b);
        let pushes = m.inverse() * self.price;
        let a = pushes.x.round();
        let b = pushes.y.round();
        if self.a * a + self.b * b == self.price {
            #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
            Some(a as u64 * 3 + b as u64)
        } else {
            None
        }
    }
}

impl FromStr for Arcade {
    type Err = !;

    fn from_str(s: &str) -> Result<Self, !> {
        let machines = s.split("\n\n").map(|s| s.parse().unwrap()).collect();
        Ok(Self { machines })
    }
}

impl FromStr for Machine {
    type Err = !;

    fn from_str(s: &str) -> Result<Self, !> {
        let mut lines = s.lines();
        Ok(Self {
            a: parse_button(lines.next().unwrap()),
            b: parse_button(lines.next().unwrap()),
            price: parse_price(lines.next().unwrap()),
        })
    }
}

fn parse_button(s: &str) -> Vector {
    let mut split = s.split_whitespace().skip(2);
    let x = split
        .next()
        .unwrap()
        .trim_start_matches("X+")
        .trim_end_matches(',')
        .parse()
        .unwrap();
    let y = split
        .next()
        .unwrap()
        .trim_start_matches("Y+")
        .parse()
        .unwrap();
    Vector::new(x, y)
}

fn parse_price(s: &str) -> Vector {
    let mut split = s.split_whitespace().skip(1);
    let x = split
        .next()
        .unwrap()
        .trim_start_matches("X=")
        .trim_end_matches(',')
        .parse()
        .unwrap();
    let y = split
        .next()
        .unwrap()
        .trim_start_matches("Y=")
        .parse()
        .unwrap();
    Vector::new(x, y)
}
