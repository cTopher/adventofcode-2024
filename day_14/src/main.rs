#![feature(never_type)]
#![feature(array_repeat)]
#![feature(iter_map_windows)]

use std::fs;
mod model;
mod tiles;

pub use model::*;

fn main() {
    let input = fs::read_to_string("day_14/input.txt").unwrap();
    println!("Answer 1: {}", &part_1::<101, 103>(&input));
    println!("Answer 2: {}", &part_2::<101, 103>(&input));
}

fn part_1<const W: usize, const H: usize>(input: &str) -> usize {
    let Ok(mut list) = input.parse::<RobotList<W, H>>();
    list.progress(100);
    list.safety_factor()
}

fn part_2<const W: usize, const H: usize>(input: &str) -> usize {
    let Ok(mut list) = input.parse::<RobotList<W, H>>();
    list.progress_until_christmas_tree();
    println!("{list}");
    list.seconds_elapsed
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tiles::Position;

    const EXAMPLE: &str = include_str!("../example.txt");

    #[test]
    fn single_robot() {
        let Ok(mut robot) = "p=2,4 v=2,-3".parse::<Robot<11, 7>>();
        assert_eq!(robot.position, Position { x: 2, y: 4 });
        robot.progress(5);
        assert_eq!(robot.position, Position { x: 1, y: 3 });
        assert_eq!(robot.position.quadrant(), None);
    }

    #[test]
    fn full_example() {
        assert_eq!(part_1::<11, 7>(EXAMPLE), 12);
    }
}
