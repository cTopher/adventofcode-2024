use crate::Position;
use std::fmt;
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

    pub fn get_mut(&mut self, Position { i, j }: Position) -> Option<&mut T> {
        self.elements.get_mut(i).and_then(|row| row.get_mut(j))
    }

    pub fn enumerate(&self) -> impl Iterator<Item = (Position, T)> + '_ {
        self.elements.iter().enumerate().flat_map(|(i, row)| {
            row.iter()
                .enumerate()
                .map(move |(j, &elem)| (Position { i, j }, elem))
        })
    }

    pub fn neighbours(&self, position: Position) -> impl Iterator<Item = (Position, T)> + '_ {
        position
            .neighbours()
            .filter_map(|position| self.get(position).map(|elem| (position, elem)))
    }

    pub fn map<U: Copy, F: FnMut(T) -> U>(&self, mut f: F) -> Matrix<U> {
        let elements = self
            .elements
            .iter()
            .map(|row| row.iter().copied().map(&mut f).collect())
            .collect();
        Matrix::new(elements)
    }

    pub fn print(&self, mut f: impl FnMut(T) -> char) {
        for row in &self.elements {
            for (index, &elem) in row.iter().enumerate() {
                if index > 0 {
                    print!(" ");
                }
                print!("{}", f(elem));
            }
            println!();
        }
    }
}

impl<T: fmt::Display> fmt::Display for Matrix<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for row in &self.elements {
            for (index, elem) in row.iter().enumerate() {
                if index > 0 {
                    write!(f, " ")?;
                }
                write!(f, "{elem}")?;
            }
            writeln!(f)?;
        }
        Ok(())
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
