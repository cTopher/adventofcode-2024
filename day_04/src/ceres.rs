use grid::{Direction, Grid, Position};
use std::str::FromStr;

pub struct WordSearch {
    letters: Grid<char>,
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

    pub fn count_xmas(&self) -> usize {
        self.letters
            .enumerate()
            .filter(|&(_, letter)| letter == 'X')
            .map(|(pos, _)| self.count_mas_at(pos))
            .sum()
    }

    pub fn count_x_mas(&self) -> usize {
        self.letters
            .enumerate()
            .filter(|&(pos, letter)| self.is_x_mas(pos, letter))
            .count()
    }

    fn is_x_mas(&self, pos: Position, letter: char) -> bool {
        if letter != 'A' {
            return false;
        }
        let diagonal1 = [Direction::new(-1, -1), Direction::new(1, 1)];
        let diagonal2 = [Direction::new(-1, 1), Direction::new(1, -1)];
        [diagonal1, diagonal2].into_iter().all(|diagonal| {
            let diagonal: Vec<_> = diagonal
                .into_iter()
                .filter_map(|dir| pos.checked_add(dir))
                .filter_map(|pos| self.letters.get(pos))
                .collect();
            diagonal.contains(&'M') && diagonal.contains(&'S')
        })
    }

    fn count_mas_at(&self, pos: Position) -> usize {
        Direction::ALL
            .into_iter()
            .filter(|&dir| self.is_mas(pos, dir))
            .count()
    }
}
impl FromStr for WordSearch {
    type Err = !;

    fn from_str(s: &str) -> Result<Self, !> {
        let Ok(letters) = s.parse();
        Ok(Self { letters })
    }
}
