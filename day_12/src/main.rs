#![feature(never_type)]
#![feature(let_chains)]

mod model;
pub use model::*;

use std::fs;

fn main() {
    let input = fs::read_to_string("day_12/input.txt").unwrap();
    println!("Answer 1: {}", &part_1(&input));
    println!("Answer 2: {}", &part_2(&input));
}

fn part_1(input: &str) -> usize {
    let Ok(map) = input.parse::<Map>();
    map.plots().map(|plot| plot.price()).sum()
}

fn part_2(input: &str) -> usize {
    let Ok(map) = input.parse::<Map>();
    map.plots().map(|plot| plot.bulk_discount_price()).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("../example.txt");

    #[test]
    fn example_1() {
        assert_eq!(part_1(EXAMPLE), 1930);
    }

    #[test]
    fn example_2() {
        assert_eq!(part_2(EXAMPLE), 1206);
    }
}
