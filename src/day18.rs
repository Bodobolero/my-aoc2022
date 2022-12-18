#![feature(test)]

use std::collections::HashSet;

extern crate test;
const INPUT: &str = include_str!("../inputs/input18.txt");

fn scan_cubes() -> Vec<[i8; 3]> {
    INPUT
        .lines()
        .map(|line| {
            let mut num_iterator = line.split(',').map(|s| s.parse::<i8>().unwrap());
            [
                num_iterator.next().unwrap(),
                num_iterator.next().unwrap(),
                num_iterator.next().unwrap(),
            ]
        })
        .collect()
}

fn part1() -> usize {
    let cubes = scan_cubes();
    let mut free_sides: usize = 0;
    for c1 in &cubes {
        let mut c1_free = 6;
        for c2 in &cubes {
            let distance = (c1[0] - c2[0]).abs() + (c1[1] - c2[1]).abs() + (c1[2] - c2[2]).abs();
            if distance == 1 {
                c1_free -= 1;
            }
        }
        free_sides += c1_free;
    }
    free_sides
}

fn traverse(
    cube: usize,
    cubes: &Vec<[i8; 3]>,
    reached: &mut Vec<bool>,
    sides: &Vec<Vec<Option<usize>>>,
    steam: &mut HashSet<[i8; 3]>,
) -> usize {
    if reached[cube] {
        return 0;
    };
    let mut free_sides: usize = 0;
    reached[cube] = true;
    let mut cube_sides: usize = 6;
    for k in 0..6 {
        let side = sides[cube][k];
        if side.is_none() {
            // check if side reachable by steam
            let mut space = cubes[cube];
            let val = if k > 3 { 1 } else { -1 };
            let index = k % 3;
            space[index] += val;
            if !steam.contains(&space) {}
        }
    }
    for k in 0..6 {
        let side = sides[cube][k];
        if side.is_some() {
            cube_sides -= 1;
            if (!reached[side.unwrap()]) {
                free_sides += traverse(side.unwrap(), cubes, reached, sides, steam);
            }
        } else {
            // check if side reachable by steam
        }
    }
    free_sides + cube_sides
}

fn part2() -> usize {
    let cubes = scan_cubes();
    // sides: -x, +x, -y, +y, -z, +z
    let mut sides: Vec<Vec<Option<usize>>> = vec![vec![None; 6]; cubes.len()];
    // build grid
    for (i, c1) in cubes.iter().enumerate() {
        for (j, c2) in cubes.iter().enumerate() {
            let distance = (c1[0] - c2[0]).abs() + (c1[1] - c2[1]).abs() + (c1[2] - c2[2]).abs();
            if distance == 1 {
                for k in 0..3 {
                    if c1[k] < c2[k] {
                        sides[i][k * 2] = Some(j);
                    }
                    if c1[k] > c2[k] {
                        sides[i][k * 2 + 1] = Some(j);
                    }
                }
            }
        }
    }
    let mut reached: Vec<bool> = vec![false; cubes.len()];
    let mut free_sides: usize = 0;
    // collect free sides reachable from the outside
    while reached.iter().filter(|b| true).count() < reached.len() {
        let mut steam: HashSet<[i8; 3]> = HashSet::new();
        // find a cube at the outside of connected graph with minimum x
        let minx = cubes
            .iter()
            .enumerate()
            .filter(|&(i, c)| !reached[i])
            .map(|(i, c)| c[0])
            .min()
            .unwrap();

        let (ci, c) = cubes
            .iter()
            .enumerate()
            .find(|&(i, c)| c[0] == minx)
            .unwrap();
        steam.insert([minx - 1, c[1], c[2]]);
        free_sides += traverse(ci, &cubes, &mut reached, &mut sides, &mut steam);
    }
    free_sides
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
        assert_eq!(part1(), 3564);
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
