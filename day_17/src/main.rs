#![feature(never_type)]

use std::fs;
use std::str::FromStr;

mod model;
pub use model::*;

fn main() {
    let input = fs::read_to_string("day_17/input.txt").unwrap();
    println!("Answer 1: {}", part_1(&input));
    println!("Answer 2: {}", part_2(&input));
}

fn part_1(input: &str) -> String {
    let Ok(mut computer) = Computer::from_str(input);
    let mut output = computer.run().into_iter();
    let mut output_str = output.next().unwrap().to_string();
    for value in output {
        output_str.push(',');
        output_str.push_str(&value.to_string());
    }
    output_str
}

fn part_2(input: &str) -> usize {
    let Ok(mut computer) = Computer::from_str(input);
    let mut paths: Vec<_> = (0..8)
        .rev()
        .map(|a| (computer.program.len() - 1, a))
        .collect();
    while let Some((index, a)) = paths.pop() {
        computer.reset(a);
        let output = computer.run();
        if output[0] != computer.program[index] {
            continue;
        }
        if index == 0 {
            return a;
        }
        paths.extend((0..8).rev().map(|x| (index - 1, a << 3 | x)));
    }
    panic!("No solution found");
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_1: &str = include_str!("../example-1.txt");
    const EXAMPLE_2: &str = include_str!("../example-2.txt");

    #[test]
    fn example_1() {
        assert_eq!(part_1(EXAMPLE_1), "4,6,3,5,6,3,5,2,1,0");
    }

    #[test]
    fn example_2() {
        assert_eq!(part_2(EXAMPLE_2), 117_440);
    }
}
