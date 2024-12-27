use std::ops::{AddAssign, Mul};
use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Position<const W: u32, const H: u32> {
    pub x: u32,
    pub y: u32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Velocity {
    x: i32,
    y: i32,
}

impl<const W: u32, const H: u32> Position<W, H> {
    pub const fn quadrant(self) -> Option<u8> {
        if self.x < W / 2 && self.y < H / 2 {
            Some(0)
        } else if self.x > W / 2 && self.y < H / 2 {
            Some(1)
        } else if self.x < W / 2 && self.y > H / 2 {
            Some(2)
        } else if self.x > W / 2 && self.y > H / 2 {
            Some(3)
        } else {
            None
        }
    }
}

impl Mul<u32> for Velocity {
    type Output = Self;

    fn mul(self, rhs: u32) -> Self::Output {
        let rhs = i32::try_from(rhs).unwrap();
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

impl<const W: u32, const H: u32> AddAssign<Velocity> for Position<W, H> {
    fn add_assign(&mut self, rhs: Velocity) {
        add_assign::<W>(&mut self.x, rhs.x);
        add_assign::<H>(&mut self.y, rhs.y);
    }
}

fn add_assign<const D: u32>(a: &mut u32, v: i32) {
    let v = v.rem_euclid(i32::try_from(D).unwrap()) as u32;
    *a = (*a + v) % D;
}

impl<const W: u32, const H: u32> FromStr for Position<W, H> {
    type Err = !;

    fn from_str(s: &str) -> Result<Self, !> {
        let (x, y) = s.trim_start_matches("p=").split_once(',').unwrap();
        let x = x.parse().unwrap();
        let y = y.parse().unwrap();
        Ok(Self { x, y })
    }
}

impl FromStr for Velocity {
    type Err = !;

    fn from_str(s: &str) -> Result<Self, !> {
        let (x, y) = s.trim_start_matches("v=").split_once(',').unwrap();
        let x = x.parse().unwrap();
        let y = y.parse().unwrap();
        Ok(Self { x, y })
    }
}
