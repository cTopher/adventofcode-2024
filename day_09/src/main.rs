#![feature(never_type)]
#![feature(iter_map_windows)]

mod fast_disk;
mod fragmentable_disk;

use std::fs;

fn main() {
    let input = fs::read_to_string("day_09/input.txt").unwrap();
    println!("Answer 1: {}", part_1(&input));
    println!("Answer 2: {}", part_2(&input));
}

fn part_1(input: &str) -> usize {
    let Ok(mut disk) = input.parse::<fragmentable_disk::DiskMap>();
    disk.compact();
    disk.checksum()
}

fn part_2(input: &str) -> usize {
    let Ok(mut disk) = input.parse::<fast_disk::DiskMap>();
    disk.compact();
    disk.checksum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "2333133121414131402";

    #[test]
    fn example_1() {
        assert_eq!(part_1(EXAMPLE), 1928);
    }

    #[test]
    fn example_2() {
        assert_eq!(part_2(EXAMPLE), 2858);
    }
}
