#![feature(never_type)]
#![feature(iter_map_windows)]

use crate::ceres::WordSearch;
use std::fs;

mod ceres;

fn main() {
    let input = fs::read_to_string("day_04/input.txt").unwrap();
    println!("Answer 1: {}", &part_1(&input));
    println!("Answer 2: {}", &part_2(&input));
}

fn part_1(input: &str) -> usize {
    let word_search: WordSearch = input.parse().unwrap();
    word_search.count_xmas()
}

fn part_2(input: &str) -> usize {
    let word_search: WordSearch = input.parse().unwrap();
    word_search.count_x_mas()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("../example.txt");

    #[test]
    fn example_1() {
        assert_eq!(part_1(EXAMPLE), 18);
    }

    #[test]
    fn example_2() {
        assert_eq!(part_2(EXAMPLE), 9);
    }
}
