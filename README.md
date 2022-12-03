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
| [Day 1](src/day01.rs) | [Problem 1](https://adventofcode.com/2022/day/1)   |    69 ms |    79 ms |   
| [Day 2](src/day02.rs) | [Problem 2](https://adventofcode.com/2022/day/2)   |   105 ms |   106 ms |   
| [Day 3](src/day03.rs) | [Problem 3](https://adventofcode.com/2022/day/3)   |    50 ms |    62 ms | 
| [Day 4](src/day04.rs) | [Problem 4](https://adventofcode.com/2022/day/4)   |  N/A  ms |  N/A  ms | 
| [Day 5](src/day05.rs) | [Problem 5](https://adventofcode.com/2022/day/5)   |  N/A  ms |  N/A  ms | 
| [Day 6](src/day06.rs) | [Problem 6](https://adventofcode.com/2022/day/6)   |  N/A  ms |  N/A  ms | 
| [Day 7](src/day07.rs) | [Problem 7](https://adventofcode.com/2022/day/7)   |  N/A  ms |  N/A  ms |
| [Day 8](src/day08.rs) | [Problem 8](https://adventofcode.com/2022/day/8)   |  N/A  ms |  N/A  ms |
| [Day 9](src/day09.rs) | [Problem 9](https://adventofcode.com/2022/day/9)   |  N/A  ms |  N/A  ms |

> The benchmarks are measured with the unstable bench feature of Rust using my Macbook Pro 13' mid 2019
