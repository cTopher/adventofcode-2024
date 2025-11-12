#![feature(never_type)]
#![feature(exhaustive_patterns)]

use crate::computer::Computer;
use std::fs;

mod computer;

fn main() {
    let input = fs::read_to_string("day_03/input.txt").unwrap();
    println!("Answer 1: {}", part_1(&input));
    println!("Answer 2: {}", part_2(&input));
}

fn part_1(input: &str) -> u32 {
    let Ok(computer) = input.parse::<Computer>();
    computer.calculate_sum_of_multiplications()
}

fn part_2(input: &str) -> u32 {
    let Ok(computer) = input.parse::<Computer>();
    computer.calculate()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_1: &str =
        "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
    const EXAMPLE_2: &str =
        "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";

    #[test]
    fn example_1() {
        assert_eq!(part_1(EXAMPLE_1), 161);
    }

    #[test]
    fn example_2() {
        assert_eq!(part_2(EXAMPLE_2), 48);
    }
}
