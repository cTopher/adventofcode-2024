#![feature(never_type)]

use std::fs;
use std::str::FromStr;

mod model;
pub use model::*;

fn main() {
    let input = fs::read_to_string("day_20/input.txt").unwrap();
    println!("Answer 1: {}", part_1(&input, 100));
    println!("Answer 2: {}", part_2(&input, 100));
}

fn part_1(input: &str, threshold: usize) -> usize {
    let Ok(track) = RaceTrack::from_str(input);
    track.count_cheats(2, threshold)
}

fn part_2(input: &str, threshold: usize) -> usize {
    let Ok(track) = RaceTrack::from_str(input);
    track.count_cheats(20, threshold)
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("../example.txt");

    #[test]
    fn example_1() {
        assert_eq!(part_1(EXAMPLE, 1), 44);
    }

    #[test]
    fn example_2() {
        assert_eq!(part_2(EXAMPLE, 50), 285);
    }
}
