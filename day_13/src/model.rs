use math_2d::{Matrix, Vector};
use regex::Regex;
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

const MACHINE_RE: &str =
    r"Button A: X\+(\d+), Y\+(\d+)\nButton B: X\+(\d+), Y\+(\d+)\nPrize: X=(\d+), Y=(\d+)";
impl FromStr for Machine {
    type Err = !;

    fn from_str(s: &str) -> Result<Self, !> {
        let regex = Regex::new(MACHINE_RE).unwrap();
        let (_, [ax, ay, bx, by, px, py]) = regex.captures(s).unwrap().extract();
        Ok(Self {
            a: Vector::new(ax.parse().unwrap(), ay.parse().unwrap()),
            b: Vector::new(bx.parse().unwrap(), by.parse().unwrap()),
            price: Vector::new(px.parse::<f64>().unwrap(), py.parse::<f64>().unwrap()),
        })
    }
}
