#![feature(never_type)]
#![feature(iter_chain)]

mod bridge;

use crate::bridge::Bridge;
use std::fs;

fn main() {
    let input = fs::read_to_string("day_07/input.txt").unwrap();
    println!("Answer 1: {}", &part_1(&input));
    println!("Answer 2: {}", &part_2(&input));
}

fn part_1(input: &str) -> u64 {
    let Ok(bridge) = input.parse::<Bridge>();
    bridge.calibration_result_without_concat()
}

fn part_2(input: &str) -> u64 {
    let Ok(bridge) = input.parse::<Bridge>();
    bridge.calibration_result_with_concat()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("../example.txt");

    #[test]
    fn example_1() {
        assert_eq!(part_1(EXAMPLE), 3749);
    }

    #[test]
    fn example_2() {
        assert_eq!(part_2(EXAMPLE), 11387);
    }
}
