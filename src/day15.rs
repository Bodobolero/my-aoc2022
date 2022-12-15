#![feature(test)]

extern crate test;
const INPUT: &str = include_str!("../inputs/input15.txt");

const LINE: i32 = 2000000;

use regex::Regex;
use std::collections::HashSet;

lazy_static::lazy_static! {
    static ref RE_RULE: Regex = Regex::new(r"^Sensor at x=([-+]?\d+), y=([-+]?\d+): closest beacon is at x=([-+]?\d+), y=([-+]?\d+)$").unwrap();
}

fn part1() -> usize {
    // compute vector of sensors and the manhattan distance of all points impossible
    let sensors_beacons: Vec<(i32, i32, i32, i32)> = INPUT
        .lines()
        .map(|line| {
            let caps = RE_RULE.captures(line).unwrap();
            (
                caps.get(1).unwrap().as_str().parse::<i32>().unwrap(),
                caps.get(2).unwrap().as_str().parse::<i32>().unwrap(),
                caps.get(3).unwrap().as_str().parse::<i32>().unwrap(),
                caps.get(4).unwrap().as_str().parse::<i32>().unwrap(),
            )
        })
        .collect();
    let sensors_range: Vec<(i32, i32, i32)> = sensors_beacons
        .iter()
        .map(|(sx, sy, bx, by)| {
            // compute manhattan distance of beacon
            (*sx, *sy, (bx - sx).abs() + (by - sy).abs())
        })
        .collect();
    let beacons: HashSet<(i32, i32)> = sensors_beacons
        .iter()
        .map(|(_, _, bx, by)| (*bx, *by))
        .collect();
    // the line at y=LINE potentially is infite - we need to determine
    // where we start to compute the positions covered
    let sx_min = *sensors_range.iter().map(|(x, _, _)| x).min().unwrap();
    let sx_max = *sensors_range.iter().map(|(x, _, _)| x).max().unwrap();
    let max_dist = *sensors_range.iter().map(|(_, _, d)| d).max().unwrap();
    let mut count: usize = 0;
    for x in sx_min - max_dist..sx_max + max_dist {
        if !beacons.contains(&(x, LINE)) {
            // compute manhattan distance of position from all sensors and check if it falls in range
            for (sx, sy, range) in &sensors_range {
                let dist = (x - *sx).abs() + (LINE - *sy).abs();
                if dist <= *range {
                    count += 1;
                    break;
                }
            }
        }
    }
    count
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
        assert_eq!(part1(), 26);
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
