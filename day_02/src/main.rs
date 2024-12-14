#![feature(never_type)]
#![feature(unsigned_signed_diff)]
#![feature(exhaustive_patterns)]

use crate::report::{Report, Reports};
use std::fs;

mod report;

fn main() {
    let input = fs::read_to_string("day_02/input.txt").unwrap();
    println!("Answer 1: {}", &part_1(&input));
    println!("Answer 2: {}", &part_2(&input));
}

fn part_1(input: &str) -> usize {
    let Ok(reports) = input.parse::<Reports>();
    reports.count(Report::is_safe)
}

fn part_2(input: &str) -> usize {
    let Ok(reports) = input.parse::<Reports>();
    reports.count(Report::is_safe_using_problem_dampener)
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("../example.txt");

    #[test]
    fn example_1() {
        assert_eq!(part_1(EXAMPLE), 2);
    }

    #[test]
    fn example_2() {
        assert_eq!(part_2(EXAMPLE), 4);
    }
}
