use crate::Coordinate;
use std::fmt;
use std::ops::{Index, IndexMut};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Grid<T, const N: usize> {
    elements: [[T; N]; N],
}

impl<T: Copy, const N: usize> Grid<T, N> {}

impl<T: fmt::Display, const N: usize> fmt::Display for Grid<T, N> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for row in &self.elements {
            for elem in row {
                write!(f, "{elem}")?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl<T, const N: usize> Index<Coordinate<N>> for Grid<T, N> {
    type Output = T;

    fn index(&self, Coordinate { x, y }: Coordinate<N>) -> &T {
        &self.elements[y][x]
    }
}

impl<T, const N: usize> IndexMut<Coordinate<N>> for Grid<T, N> {
    fn index_mut(&mut self, Coordinate { x, y }: Coordinate<N>) -> &mut Self::Output {
        &mut self.elements[y][x]
    }
}

impl<T: Default + Copy, const N: usize> Default for Grid<T, N> {
    fn default() -> Self {
        Self {
            elements: [[T::default(); N]; N],
        }
    }
}
