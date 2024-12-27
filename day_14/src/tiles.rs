use std::ops::{AddAssign, Mul};
use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Position<const W: usize, const H: usize> {
    pub x: usize,
    pub y: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Velocity {
    x: i32,
    y: i32,
}

impl<const W: usize, const H: usize> Position<W, H> {
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

impl Mul<usize> for Velocity {
    type Output = Self;

    fn mul(self, rhs: usize) -> Self::Output {
        let rhs = i32::try_from(rhs).unwrap();
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

impl<const W: usize, const H: usize> AddAssign<Velocity> for Position<W, H> {
    fn add_assign(&mut self, rhs: Velocity) {
        add_assign::<W>(&mut self.x, rhs.x);
        add_assign::<H>(&mut self.y, rhs.y);
    }
}

fn add_assign<const D: usize>(a: &mut usize, v: i32) {
    let v = v.rem_euclid(i32::try_from(D).unwrap()) as usize;
    *a = (*a + v) % D;
}

impl<const W: usize, const H: usize> FromStr for Position<W, H> {
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
