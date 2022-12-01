use my_aoc2022::{day01, day02, day03, day04, day05, day06, day07, day08};
fn main() {
    let solutions = [
        day01::main,
        day02::main,
        day03::main,
        day04::main,
        day05::main,
        day06::main,
        day07::main,
        day08::main,
    ];
    for (day, solution) in solutions.iter().enumerate() {
        println!("------ Day {} ------", day + 1);
        solution()
    }
}
