use crate::Position;
use std::ops::{Index, IndexMut};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Matrix<T> {
    elements: Vec<Vec<T>>,
}

impl<T: Copy> Matrix<T> {
    #[must_use]
    pub const fn new(elements: Vec<Vec<T>>) -> Self {
        Self { elements }
    }

    #[must_use]
    pub fn get(&self, Position { i, j }: Position) -> Option<T> {
        self.elements.get(i).and_then(|row| row.get(j).copied())
    }

    pub fn enumerate(&self) -> impl Iterator<Item = (Position, T)> + '_ {
        self.elements.iter().enumerate().flat_map(|(i, row)| {
            row.iter()
                .enumerate()
                .map(move |(j, &elem)| (Position { i, j }, elem))
        })
    }

    pub fn map<U: Copy, F: FnMut(T) -> U>(&self, mut f: F) -> Matrix<U> {
        let elements = self
            .elements
            .iter()
            .map(|row| row.iter().copied().map(&mut f).collect())
            .collect();
        Matrix::new(elements)
    }
}

impl<T> Index<Position> for Matrix<T> {
    type Output = T;

    fn index(&self, Position { i, j }: Position) -> &Self::Output {
        &self.elements[i][j]
    }
}

impl<T> IndexMut<Position> for Matrix<T> {
    fn index_mut(&mut self, Position { i, j }: Position) -> &mut Self::Output {
        &mut self.elements[i][j]
    }
}

impl<T: Copy> From<Vec<Vec<T>>> for Matrix<T> {
    fn from(elements: Vec<Vec<T>>) -> Self {
        Self::new(elements)
    }
}
