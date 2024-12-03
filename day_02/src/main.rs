#![feature(never_type)]

use crate::report::Reports;
use std::fs;

mod report;

fn main() {
    let input = fs::read_to_string("day_02/input.txt").unwrap();
    println!("Answer 1: {}", &part_1(&input));
    println!("Answer 2: {}", &part_2(&input));
}

fn part_1(input: &str) -> usize {
    let reports: Reports = input.parse().unwrap();
    reports.count_safe()
}

fn part_2(input: &str) -> usize {
    let reports: Reports = input.parse().unwrap();
    reports.count_safe()
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
        assert_eq!(part_2(EXAMPLE), 2);
    }
}
