# Advent of Code 2022
My [Advent of Code 2022](https://adventofcode.com) solutions in the Rust programming language.

Structure of solutions modeled after https://github.com/ahmadkaouk/advent-of-code-2021

## Usage
```sh
# Run all the days
cargo run --release

# Test a specific day
cargo test --bin day01

#Run a specific day
cargo run --release --bin day01
```
## Benchmark
### Run

```sh
cargo bench
```



### Timing

|                       | Problem                                            | Part 1   | Part 2   |   
|-----------------------|----------------------------------------------------|----------|----------|
| [Day 1](src/day01.rs) | [Problem 1](https://adventofcode.com/2022/day/1)   | 0.069 ms | 0.079 ms |   
| [Day 2](src/day02.rs) | [Problem 2](https://adventofcode.com/2022/day/2)   | 0.105 ms | 0.106 ms |   
| [Day 3](src/day03.rs) | [Problem 3](https://adventofcode.com/2022/day/3)   | 0.054 ms | 0.026 ms | 
| [Day 4](src/day04.rs) | [Problem 4](https://adventofcode.com/2022/day/4)   | 0.569 ms | 0.563 ms | 
| [Day 5](src/day05.rs) | [Problem 5](https://adventofcode.com/2022/day/5)   | 0.340 ms | 0.423 ms | 
| [Day 6](src/day06.rs) | [Problem 6](https://adventofcode.com/2022/day/6)   | 0.208 ms | 0.928 ms | 
| [Day 7](src/day07.rs) | [Problem 7](https://adventofcode.com/2022/day/7)   | 0.116 ms | 0.130 ms |
| [Day 8](src/day08.rs) | [Problem 8](https://adventofcode.com/2022/day/8)   | 0.706 ms | 0.644 ms |
| [Day 9](src/day09.rs) | [Problem 9](https://adventofcode.com/2022/day/9)   | 0.785 ms | 1.230 ms |
| [Day 10](src/day10.rs)| [Problem 10](https://adventofcode.com/2022/day/10) | 0.011 ms | 0.013 ms |
| [Day 11](src/day10.rs)| [Problem 11](https://adventofcode.com/2022/day/11) |   N/A ms |   N/A ms |
| [Day 12](src/day10.rs)| [Problem 12](https://adventofcode.com/2022/day/12) |   N/A ms |   N/A ms |
| [Day 13](src/day10.rs)| [Problem 13](https://adventofcode.com/2022/day/13) |   N/A ms |   N/A ms |
| [Day 14](src/day10.rs)| [Problem 14](https://adventofcode.com/2022/day/14) |   N/A ms |   N/A ms |
| [Day 15](src/day10.rs)| [Problem 15](https://adventofcode.com/2022/day/15) |   N/A ms |   N/A ms |
| [Day 16](src/day10.rs)| [Problem 16](https://adventofcode.com/2022/day/16) |   N/A ms |   N/A ms |
| [Day 17](src/day10.rs)| [Problem 17](https://adventofcode.com/2022/day/17) |   N/A ms |   N/A ms |
| [Day 18](src/day10.rs)| [Problem 18](https://adventofcode.com/2022/day/18) |   N/A ms |   N/A ms |
| [Day 19](src/day10.rs)| [Problem 19](https://adventofcode.com/2022/day/19) |   N/A ms |   N/A ms |
| [Day 20](src/day10.rs)| [Problem 20](https://adventofcode.com/2022/day/20) |   N/A ms |   N/A ms |
| [Day 21](src/day10.rs)| [Problem 21](https://adventofcode.com/2022/day/21) |   N/A ms |   N/A ms |
| [Day 22](src/day10.rs)| [Problem 22](https://adventofcode.com/2022/day/22) |   N/A ms |   N/A ms |
| [Day 23](src/day10.rs)| [Problem 23](https://adventofcode.com/2022/day/23) |   N/A ms |   N/A ms |
| [Day 24](src/day10.rs)| [Problem 24](https://adventofcode.com/2022/day/24) |   N/A ms |   N/A ms |
| [Day 25](src/day10.rs)| [Problem 25](https://adventofcode.com/2022/day/25) |   N/A ms |   N/A ms |


> The benchmarks are measured with the unstable bench feature of Rust using my Macbook Pro 13' mid 2019
