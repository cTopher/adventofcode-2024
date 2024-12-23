#![feature(never_type)]

use crate::model::Arcade;
use std::fs;

mod model;

fn main() {
    let input = fs::read_to_string("day_13/input.txt").unwrap();
    println!("Answer 1: {}", &part_1(&input));
    println!("Answer 2: {}", &part_2(&input));
}

fn part_1(input: &str) -> u64 {
    let Ok(arcade) = input.parse::<Arcade>();
    arcade.tokens_to_win_all()
}

fn part_2(input: &str) -> u64 {
    let Ok(mut arcade) = input.parse::<Arcade>();
    arcade.increase_price(10_000_000_000_000);
    arcade.tokens_to_win_all()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("../example.txt");

    #[test]
    fn example_1() {
        assert_eq!(part_1(EXAMPLE), 480);
    }

    #[test]
    fn example_2() {
        assert_eq!(part_2(EXAMPLE), 875_318_608_908);
    }
}
