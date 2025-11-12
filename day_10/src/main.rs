#![feature(never_type)]

mod model;

use crate::model::LavaIsland;
use std::fs;

fn main() {
    let input = fs::read_to_string("day_10/input.txt").unwrap();
    println!("Answer 1: {}", part_1(&input));
    println!("Answer 2: {}", part_2(&input));
}

fn part_1(input: &str) -> usize {
    let Ok(island) = input.parse::<LavaIsland>();
    island.total_trailheads_score()
}

fn part_2(input: &str) -> usize {
    let Ok(island) = input.parse::<LavaIsland>();
    island.total_trailheads_rating()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("../example.txt");

    #[test]
    fn example_1() {
        assert_eq!(part_1(EXAMPLE), 36);
    }

    #[test]
    fn example_2() {
        assert_eq!(part_2(EXAMPLE), 81);
    }
}
