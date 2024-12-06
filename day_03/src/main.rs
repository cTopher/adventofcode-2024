#![feature(never_type)]

use std::fs;

mod computer;

fn main() {
    let input = fs::read_to_string("day_03/input.txt").unwrap();
    println!("Answer 1: {}", &part_1(&input));
    println!("Answer 2: {}", &part_2(&input));
}

fn part_1(input: &str) -> u32 {
    let computer: computer::Computer = input.parse().unwrap();
    computer.calculate_sum_of_multiplications()
}

fn part_2(input: &str) -> u32 {
    let computer: computer::Computer = input.parse().unwrap();
    computer.calculate_sum_of_multiplications()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";

    #[test]
    fn example_1() {
        assert_eq!(part_1(EXAMPLE), 161);
    }

    #[test]
    fn example_2() {
        assert_eq!(part_2(EXAMPLE), 0);
    }
}
