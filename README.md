# Advent of Code 2020
My [Advent of Code 2020](https://adventofcode.com) solutions in the Rust programming language.

Structure of solutions modeled after https://github.com/ahmadkaouk/advent-of-code-2021

## Usage
```sh
# Run all the days
cargo run --release

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
| [Day 1](src/day01.rs) | [Problem 1](https://adventofcode.com/2020/day/1)   |  0.007 ms |  0.376 ms |   
| [Day 2](src/day02.rs) | [Problem 2](https://adventofcode.com/2020/day/2)   |  0.715 ms |  0.642 ms |   
| [Day 3](src/day03.rs) | [Problem 3](https://adventofcode.com/2020/day/3)   |  0.012 ms |  0.073 ms | 
| [Day 4](src/day04.rs) | [Problem 4](https://adventofcode.com/2020/day/4)   |  0.303 ms |  0.701 ms | 
| [Day 5](src/day05.rs) | [Problem 5](https://adventofcode.com/2020/day/5)   |  0.025 ms |  0.050 ms | 
| [Day 6](src/day06.rs) | [Problem 6](https://adventofcode.com/2020/day/6)   |  0.447 ms |  1.111 ms | 
| [Day 6](src/day06.rs) | [Tim Visee](https://adventofcode.com/2020/day/6)   |  N/A   ms |  0.080 ms!|
| [Day 7](src/day07.rs) | [Problem 7](https://adventofcode.com/2020/day/7)   |  2.305 ms |  2.429 ms |
| [Day 8](src/day08.rs) | [Problem 8](https://adventofcode.com/2020/day/8)   |  0.032 ms |  0.087 ms |
| [Day 9](src/day09.rs) | [Problem 9](https://adventofcode.com/2020/day/9)   |  0.008 ms |  0.144 ms |

> The benchmarks are measured with the unstable bench feature of Rust using my Macbook Pro 15' mid 2016
