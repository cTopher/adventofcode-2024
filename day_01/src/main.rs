#![feature(never_type)]

use std::fs;
use crate::location::LocationLists;

mod location;

fn main() {
    let input = fs::read_to_string("day_01/input.txt").unwrap();
    println!("Answer 1: {}", &part_1(&input));
}

fn part_1(input: &str) -> u32 {
    let mut location_lists: LocationLists = input.parse().unwrap();
    location_lists.sort();
    location_lists.total_distance()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("../example.txt");

    #[test]
    fn example_1() {
        assert_eq!(part_1(EXAMPLE), 11);
    }
}
