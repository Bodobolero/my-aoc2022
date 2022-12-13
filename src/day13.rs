#![feature(test)]

use itertools::Itertools;

extern crate test;
const INPUT: &str = include_str!("../inputs/input13.txt");

#[derive(Debug)]
enum elem {
    val(usize),
    list(Vec<elem>),
}

fn parse(list_str: &str) -> (Option<elem>, &str) {
    match list_str.chars().nth(0).unwrap() {
        '[' => {
            let mut list: Vec<elem> = Vec::new();
            let (mut element, mut rest) = parse(&list_str[1..]);
            while element.is_some() {
                list.push(element.unwrap());
                if rest.chars().nth(0).unwrap() != ',' {
                    break;
                }
                (element, rest) = parse(&rest[1..]);
            }
            (Some(elem::list(list)), &rest[1..])
        }
        ']' => (None, &list_str),
        _ => {
            // parse integer part
            // find first non-digit
            let (pos, _) = list_str.chars().find_position(|c| !c.is_digit(10)).unwrap();
            let val = list_str[0..pos].parse::<usize>().unwrap();
            (Some(elem::val(val)), &list_str[pos..])
        }
    }
}

fn compare(e1: &elem, e2: &elem) -> bool {
    match (e1, e2) {
        (elem::val(val1), elem::val(val2)) => {
            println!("Compare [{}] vs [{}] is {}", val1, val2, val1 < val2);
            val1 < val2
        }
        (elem::list(l1), elem::list(l2)) => {
            for (index, l1_item) in l1.into_iter().enumerate() {
                let l2_item = l2.into_iter().nth(index);
                if l2_item.is_none() {
                    return false;
                }
                if compare(l1_item, l2_item.unwrap()) {
                    return true;
                }
                if compare(l2_item.unwrap(), l1_item) {
                    return false;
                }
            }
            if l1.len() < l2.len() {
                return true;
            }
            false
        }
        (elem::val(val1), elem::list(_)) => {
            let mut list: Vec<elem> = Vec::new();
            list.push(elem::val(*val1));
            compare(&elem::list(list), e2)
        }
        (elem::list(l1), elem::val(val2)) => {
            let mut list: Vec<elem> = Vec::new();
            list.push(elem::val(*val2));
            compare(e1, &elem::list(list))
        }
        _ => panic!("case not considered"),
    }
}
fn part1() -> usize {
    INPUT
        .split("\n\n")
        .enumerate()
        .map(|(i, pair)| {
            let index = i + 1;
            let mut pair_lines = pair.lines();
            let (left, _) = parse(pair_lines.next().unwrap());
            let (right, _) = parse(pair_lines.next().unwrap());
            let left = left.unwrap();
            let right = right.unwrap();
            println!("left: {:?}", left);
            println!("right: {:?}\n", right);
            (index, left, right)
        })
        .filter(|(index, left, right)| {
            let result = compare(left, right);
            println!("Index {} result {}", index, result);
            result
        })
        .map(|(index, _, _)| index)
        .sum::<usize>()
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
        assert_eq!(part1(), 5393);
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
