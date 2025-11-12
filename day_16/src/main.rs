#![feature(never_type)]

use std::fs;
use std::str::FromStr;

mod model;
pub use model::*;

fn main() {
    let input = fs::read_to_string("day_16/input.txt").unwrap();
    println!("Answer 1: {}", part_1(&input));
    println!("Answer 2: {}", part_2(&input));
}

fn part_1(input: &str) -> u32 {
    let Ok(maze) = Maze::from_str(input);
    maze.best_path().score()
}

fn part_2(input: &str) -> usize {
    let Ok(maze) = Maze::from_str(input);
    maze.count_best_spots()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_1: &str = include_str!("../example-1.txt");
    const EXAMPLE_2: &str = include_str!("../example-2.txt");

    #[test]
    fn part_1_example_1() {
        assert_eq!(part_1(EXAMPLE_1), 7036);
    }

    #[test]
    fn part_1_example_2() {
        assert_eq!(part_1(EXAMPLE_2), 11048);
    }

    #[test]
    fn part_2_example_1() {
        assert_eq!(part_2(EXAMPLE_1), 45);
    }

    #[test]
    fn part_2_example_2() {
        assert_eq!(part_2(EXAMPLE_2), 64);
    }
}
