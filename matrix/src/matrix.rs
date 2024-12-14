use crate::Position;

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
}

impl<T: Copy> From<Vec<Vec<T>>> for Matrix<T> {
    fn from(elements: Vec<Vec<T>>) -> Self {
        Self::new(elements)
    }
}
