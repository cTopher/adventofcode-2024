#![feature(never_type)]
#![feature(exhaustive_patterns)]

mod lab;

use crate::lab::Lab;
use std::fs;

fn main() {
    let input = fs::read_to_string("day_06/input.txt").unwrap();
    println!("Answer 1: {}", &part_1(&input));
    println!("Answer 2: {}", &part_2(&input));
}

fn part_1(input: &str) -> usize {
    let Ok(lab) = input.parse::<Lab>();
    lab.guard_path().len()
}

fn part_2(input: &str) -> usize {
    let Ok(lab) = input.parse::<Lab>();
    lab.looping_obstruction_count()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("../example.txt");

    #[test]
    fn example_1() {
        assert_eq!(part_1(EXAMPLE), 41);
    }

    #[test]
    fn example_2() {
        assert_eq!(part_2(EXAMPLE), 6);
    }
}
