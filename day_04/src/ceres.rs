use matrix::{Direction, Matrix, Position};
use std::str::FromStr;

pub struct WordSearch {
    letters: Matrix<char>,
}

impl WordSearch {
    fn is_mas(&self, mut pos: Position, dir: Direction) -> bool {
        for c in ['M', 'A', 'S'] {
            pos = if let Some(pos) = pos.checked_add(dir) {
                pos
            } else {
                return false;
            };
            if self.letters.get(pos) != Some(c) {
                return false;
            }
        }
        true
    }

    pub fn count_xmas(&self) -> u32 {
        self.letters
            .enumerate()
            .filter(|&(_, letter)| letter == 'X')
            .map(|(pos, _)| self.count_mas_at(pos))
            .sum()
    }

    fn count_mas_at(&self, pos: Position) -> u32 {
        Direction::ALL
            .into_iter()
            .filter(|&dir| self.is_mas(pos, dir))
            .count()
            .try_into()
            .unwrap()
    }
}
impl FromStr for WordSearch {
    type Err = !;

    fn from_str(s: &str) -> Result<Self, !> {
        let letters = Matrix::new(s.lines().map(|line| line.chars().collect()).collect());
        Ok(Self { letters })
    }
}
