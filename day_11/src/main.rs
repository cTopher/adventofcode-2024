#![feature(never_type)]

mod model;

use crate::model::Stones;
use std::fs;

fn main() {
    let input = fs::read_to_string("day_11/input.txt").unwrap();
    println!("Answer 1: {}", &part_1(&input));
    println!("Answer 2: {}", &part_2(&input));
}

fn part_1(input: &str) -> usize {
    let Ok(mut stones) = input.parse::<Stones>();
    for _ in 0..25 {
        stones.blink();
    }
    stones.amount()
}

fn part_2(input: &str) -> u32 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "125 17";

    #[test]
    fn example_1() {
        assert_eq!(part_1(EXAMPLE), 55312);
    }

    #[test]
    fn example_2() {
        assert_eq!(part_2(EXAMPLE), 0);
    }
}
