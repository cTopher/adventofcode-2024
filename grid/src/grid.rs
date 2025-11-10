use crate::Position;
use std::fmt;
use std::ops::{Index, IndexMut};
use std::str::FromStr;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Grid<T> {
    elements: Vec<Vec<T>>,
}

impl<T: Copy> Grid<T> {
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

    // this could be done using width and height, but I'm lazy
    pub fn positions(&self) -> impl Iterator<Item = Position> + '_ {
        self.elements
            .iter()
            .enumerate()
            .flat_map(|(i, row)| row.iter().enumerate().map(move |(j, _)| Position { i, j }))
    }

    pub fn neighbours(&self, position: Position) -> impl Iterator<Item = (Position, T)> + '_ {
        position
            .neighbours()
            .filter_map(|position| self.get(position).map(|elem| (position, elem)))
    }

    pub fn map<U: Copy, F: FnMut(T) -> U>(&self, mut f: F) -> Grid<U> {
        self.elements
            .iter()
            .map(|row| row.iter().copied().map(&mut f).collect())
            .collect()
    }

    pub fn iter_rows(&self) -> impl Iterator<Item = &[T]> {
        self.elements.iter().map(Vec::as_slice)
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

    pub fn from_str_per_char(s: &str, f: impl Fn(char) -> T) -> Self {
        s.lines()
            .map(|line| line.chars().map(&f).collect())
            .collect()
    }
}

impl<T: PartialEq + Copy> Grid<T> {
    pub fn position(&self, target: T) -> Option<Position> {
        self.enumerate().find_map(
            |(position, elem)| {
                if elem == target { Some(position) } else { None }
            },
        )
    }
}

impl<T: fmt::Display> fmt::Display for Grid<T> {
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

impl<T> Index<Position> for Grid<T> {
    type Output = T;

    fn index(&self, Position { i, j }: Position) -> &Self::Output {
        &self.elements[i][j]
    }
}

impl<T> IndexMut<Position> for Grid<T> {
    fn index_mut(&mut self, Position { i, j }: Position) -> &mut Self::Output {
        &mut self.elements[i][j]
    }
}

impl<T: Copy + From<char>> FromStr for Grid<T> {
    type Err = !;

    fn from_str(s: &str) -> Result<Self, !> {
        Ok(s.lines()
            .map(|line| line.chars().map(T::from).collect())
            .collect())
    }
}

impl<T: Copy> FromIterator<Vec<T>> for Grid<T> {
    fn from_iter<I: IntoIterator<Item = Vec<T>>>(iter: I) -> Self {
        Self::new(iter.into_iter().collect())
    }
}
