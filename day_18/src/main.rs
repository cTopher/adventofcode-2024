#![feature(never_type)]

use std::fs;
use std::str::FromStr;

mod model;
pub use model::*;

fn main() {
    let input = fs::read_to_string("day_18/input.txt").unwrap();
    println!("Answer 1: {}", part_1::<71>(&input, 1024));
    println!("Answer 2: {}", part_2::<71>(&input));
}

fn part_1<const N: usize>(input: &str, simulation_size: usize) -> u32 {
    let Ok(mut computer) = Computer::<N>::from_str(input);
    computer.simulate(simulation_size);
    computer.shortest_safe_path().steps
}

fn part_2<const N: usize>(_input: &str) -> u32 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("../example.txt");

    #[test]
    fn example_1() {
        assert_eq!(part_1::<7>(EXAMPLE, 12), 22);
    }

    #[test]
    fn example_2() {
        assert_eq!(part_2::<7>(EXAMPLE), 0);
    }
}
