#![feature(test)]

use std::num;

extern crate test;
const INPUT: &str = include_str!("../inputs/input18.txt");

fn scan_cubes() -> Vec<(i8, i8, i8)> {
    INPUT
        .lines()
        .map(|line| {
            let mut num_iterator = line.split(',').map(|s| s.parse::<i8>().unwrap());
            (
                num_iterator.next().unwrap(),
                num_iterator.next().unwrap(),
                num_iterator.next().unwrap(),
            )
        })
        .collect()
}

fn part1() -> usize {
    let cubes = scan_cubes();
    let mut free_sides: usize = 0;
    for c1 in &cubes {
        let mut c1_free = 6;
        for c2 in &cubes {
            let distance = (c1.0 - c2.0).abs() + (c1.1 - c2.1).abs() + (c1.2 - c2.2).abs();
            if distance == 1 {
                c1_free -= 1;
            }
        }
        free_sides += c1_free;
    }
    free_sides
}

fn part2() -> usize {
    0
}

pub fn main() {
    println!("Part 1: Answer {}", part1());
    println!("Part 2: Answer {} ", part2());
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[test]
    fn part1_test() {
        assert_eq!(part1(), 64);
    }
    #[test]
    fn part2_test() {
        assert_eq!(part2(), 0);
    }
    #[bench]
    fn part1_bench(b: &mut Bencher) {
        b.iter(part1);
    }

    #[bench]
    fn part2_bench(b: &mut Bencher) {
        b.iter(part2);
    }
}
