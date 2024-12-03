#![feature(never_type)]

use crate::location::LocationLists;
use std::fs;

mod location;

fn main() {
    let input = fs::read_to_string("day_01/input.txt").unwrap();
    println!("Answer 1: {}", &part_1(&input));
    println!("Answer 2: {}", &part_2(&input));
}

fn part_1(input: &str) -> u32 {
    let mut location_lists: LocationLists = input.parse().unwrap();
    location_lists.sort();
    location_lists.total_distance()
}

fn part_2(input: &str) -> u32 {
    let location_lists: LocationLists = input.parse().unwrap();
    location_lists.similarity_score()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("../example.txt");

    #[test]
    fn example_1() {
        assert_eq!(part_1(EXAMPLE), 11);
    }

    #[test]
    fn example_2() {
        assert_eq!(part_2(EXAMPLE), 31);
    }
}
