#![feature(never_type)]

use crate::sleigh::SleighLaunchSafetyManual;
use std::fs;

mod sleigh;

fn main() {
    let input = fs::read_to_string("day_05/input.txt").unwrap();
    println!("Answer 1: {}", &part_1(&input));
    println!("Answer 2: {}", &part_2(&input));
}

fn part_1(input: &str) -> u32 {
    let manual: SleighLaunchSafetyManual = input.parse().unwrap();
    manual.correct_middle_pages()
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
        assert_eq!(part_1(EXAMPLE), 143);
    }

    #[test]
    fn example_2() {
        assert_eq!(part_2(EXAMPLE), 0);
    }
}
