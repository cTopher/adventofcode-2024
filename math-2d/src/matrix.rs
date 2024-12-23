use crate::Vector;
use std::ops::Mul;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Matrix {
    a: f64,
    b: f64,
    c: f64,
    d: f64,
}

impl Matrix {
    #[must_use]
    pub const fn from_columns(c1: Vector, c2: Vector) -> Self {
        Self {
            a: c1.x,
            b: c2.x,
            c: c1.y,
            d: c2.y,
        }
    }

    #[must_use]
    pub const fn determinant(&self) -> f64 {
        self.a * self.d - self.b * self.c
    }

    #[must_use]
    pub const fn inverse(&self) -> Self {
        let det = self.determinant();
        Self {
            a: self.d / det,
            b: -self.b / det,
            c: -self.c / det,
            d: self.a / det,
        }
    }
}

impl Mul<Vector> for Matrix {
    type Output = Vector;

    fn mul(self, Vector { x, y }: Vector) -> Vector {
        Vector {
            x: self.a.mul_add(x, self.b * y),
            y: self.c.mul_add(x, self.d * y),
        }
    }
}
