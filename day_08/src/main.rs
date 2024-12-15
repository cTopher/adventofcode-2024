#![feature(never_type)]

mod antenna;

use crate::antenna::City;
use std::fs;

fn main() {
    let input = fs::read_to_string("day_08/input.txt").unwrap();
    println!("Answer 1: {}", &part_1(&input));
    println!("Answer 2: {}", &part_2(&input));
}

fn part_1(input: &str) -> usize {
    let Ok(city) = input.parse::<City>();
    city.antinodes()
        .enumerate()
        .filter(|&(_, is_antinode)| is_antinode)
        .count()
}

fn part_2(input: &str) -> usize {
    let Ok(city) = input.parse::<City>();
    city.resonant_harmonic_antinodes()
        .enumerate()
        .filter(|&(_, is_antinode)| is_antinode)
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("../example.txt");

    #[test]
    fn example_1() {
        assert_eq!(part_1(EXAMPLE), 14);
    }

    #[test]
    fn example_2() {
        assert_eq!(part_2(EXAMPLE), 34);
    }
}
