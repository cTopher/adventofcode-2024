#![feature(never_type)]

use fixed_grid::Coordinate;
use std::fs;
use std::str::FromStr;

mod model;
pub use model::*;

fn main() {
    let input = fs::read_to_string("day_18/input.txt").unwrap();
    println!("Answer 1: {}", part_1::<71>(&input, 1024));
    let answer_2 = part_2::<71>(&input);
    println!("Answer 2: {},{}", answer_2.x, answer_2.y);
}

fn part_1<const N: usize>(input: &str, simulation_size: usize) -> u32 {
    let Ok(mut computer) = Computer::<N>::from_str(input);
    for _ in 0..simulation_size {
        computer.simulate();
    }
    computer.shortest_safe_path().unwrap().steps
}

fn part_2<const N: usize>(input: &str) -> Coordinate<N> {
    let Ok(mut computer) = Computer::<N>::from_str(input);
    loop {
        let position = computer.simulate();
        if computer.shortest_safe_path().is_none() {
            return position;
        }
    }
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
        assert_eq!(part_2::<7>(EXAMPLE), Coordinate::new(6, 1));
    }
}
