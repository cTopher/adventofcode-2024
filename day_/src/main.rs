#![feature(never_type)]

use std::fs;

fn main() {
    let input = fs::read_to_string("day_/input.txt").unwrap();
    println!("Answer 1: {}", &part_1(&input));
    println!("Answer 2: {}", &part_2(&input));
}

fn part_1(input: &str) -> u32 {
    0
}

fn part_2(input: &str) -> u32 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("../example.txt");

    #[test]
    fn example_1() {
        assert_eq!(part_1(EXAMPLE), 0);
    }

    #[test]
    fn example_2() {
        assert_eq!(part_2(EXAMPLE), 0);
    }
}
