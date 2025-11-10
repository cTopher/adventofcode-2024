#![feature(never_type)]

use std::fs;
use std::str::FromStr;

mod model;
pub use model::*;

fn main() {
    let input = fs::read_to_string("day_17/input.txt").unwrap();
    println!("Answer 1: {}", &part_1(&input));
    println!("Answer 2: {}", &part_2(&input));
}

fn part_1(input: &str) -> String {
    let Ok(mut computer) = Computer::from_str(input);
    computer.run()
}

fn part_2(input: &str) -> usize {
    unimplemented!()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("../example.txt");

    #[test]
    fn example_1() {
        assert_eq!(part_1(EXAMPLE), "4,6,3,5,6,3,5,2,1,0");
    }

    #[test]
    fn example_2() {
        assert_eq!(part_2(EXAMPLE), 9021);
    }
}
