#![feature(test)]
#![feature(box_syntax)]

use itertools::Itertools;

extern crate test;
const INPUT: &str = include_str!("../inputs/input11.txt");

#[derive(Debug)]
struct Monkey {
    items: Vec<usize>,
    operation: fn(usize, usize) -> usize,
    operand: usize,
    div: usize,
    true_target: usize,
    false_target: usize,
    inspections: usize,
}

fn add(x: usize, y: usize) -> usize {
    x + y
}

fn mult(x: usize, y: usize) -> usize {
    x * y
}

fn add2(x: usize, _: usize) -> usize {
    2 * x
}

fn mult2(x: usize, _: usize) -> usize {
    x * x
}

fn parseMonkeys(input: &str) -> Vec<Monkey> {
    let mut monkeys: Vec<Monkey> = Vec::new();
    for (count, m) in INPUT.split("\n\n").enumerate() {
        // skip line "Monkey x:"
        let mut lines = m.lines().skip(1);
        let items: Vec<usize> = lines
            .next()
            .unwrap()
            .split(':')
            .skip(1)
            .next()
            .unwrap()
            .split(",")
            .map(|s| s.trim())
            .map(|i| i.parse::<usize>().unwrap())
            .collect_vec();
        // println!("Monkey {} items:{:?}", count, items);
        let mut opskip = lines
            .next()
            .unwrap()
            .split(':')
            .skip(1)
            .next()
            .unwrap()
            .split(' ')
            .skip(4);
        let op = opskip.next().unwrap();
        // println!("Monkey {} opcode:{}", count, op);
        let op_str = opskip.next().unwrap();
        let mut operand = 0usize;
        // println!("Monkey {} opval:{}", count, op_str);
        let operation = match op {
            "+" => match op_str {
                "old" => add2,
                _ => {
                    operand = op_str.parse::<usize>().unwrap();
                    add
                }
            },
            "*" => match op_str {
                "old" => mult2,
                _ => {
                    operand = op_str.parse::<usize>().unwrap();
                    mult
                }
            },
            _ => panic!("unexpected opcode"),
        };
        let div = lines
            .next()
            .unwrap()
            .split(" ")
            .last()
            .unwrap()
            .parse::<usize>()
            .unwrap();
        let true_target = lines
            .next()
            .unwrap()
            .split(" ")
            .last()
            .unwrap()
            .parse::<usize>()
            .unwrap();
        let false_target = lines
            .next()
            .unwrap()
            .split(" ")
            .last()
            .unwrap()
            .parse::<usize>()
            .unwrap();
        monkeys.push(Monkey {
            items,
            operation,
            operand,
            div,
            true_target,
            false_target,
            inspections: 0,
        })
    }
    monkeys
}

fn part1() -> usize {
    let mut monkeys = parseMonkeys(INPUT);
    // println!("Monkeys {:?}", monkeys);
    // rounds
    for round in 0..20 {
        // turns
        for i in 0..monkeys.len() {
            let mut targets: Vec<(usize, usize)> = Vec::new();
            let monkey = &monkeys[i];
            // items
            for item in &monkey.items {
                let worrylevel = (monkey.operation)(*item, monkey.operand) / 3;
                let target = match worrylevel % monkey.div == 0 {
                    true => monkey.true_target,
                    false => monkey.false_target,
                };
                targets.push((target, worrylevel));
            }
            monkeys[i].inspections += targets.len();
            monkeys[i].items.clear();
            for (target, worrylevel) in targets {
                monkeys[target].items.push(worrylevel);
            }
        }
    }
    let mut inspections = monkeys.into_iter().map(|m| m.inspections).sorted().rev();
    inspections.next().unwrap() * inspections.next().unwrap()
}

fn part2() -> usize {
    let mut monkeys = parseMonkeys(INPUT);
    // println!("Monkeys {:?}", monkeys);
    // rounds
    for round in 0..10000 {
        // turns
        for i in 0..monkeys.len() {
            let mut targets: Vec<(usize, usize)> = Vec::new();
            let monkey = &monkeys[i];
            // items
            for item in &monkey.items {
                let worrylevel = (monkey.operation)(*item, monkey.operand) / 3;
                let target = match worrylevel % monkey.div == 0 {
                    true => monkey.true_target,
                    false => monkey.false_target,
                };
                targets.push((target, worrylevel));
            }
            monkeys[i].inspections += targets.len();
            monkeys[i].items.clear();
            for (target, worrylevel) in targets {
                monkeys[target].items.push(worrylevel);
            }
        }
    }
    let mut inspections = monkeys.into_iter().map(|m| m.inspections).sorted().rev();
    inspections.next().unwrap() * inspections.next().unwrap()
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
        assert_eq!(part1(), 58786);
    }
    #[test]
    fn part2_test() {
        assert_eq!(part2(), 2713310158);
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
