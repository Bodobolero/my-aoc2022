#![feature(test)]

extern crate test;
const INPUT: &str = include_str!("../inputs/input17.txt");

const MAX_ROCKS: usize = 2022;
// const MAX_ROCKS2: usize = 1000000000000;
const MAX_ROCKS2: usize = 100000000;

const ROCKS: &str = r#"####

.#.
###
.#.

..#
..#
###

#
#
#
#

##
##"#;

const WIDTH: usize = 7;

#[derive(Debug)]
struct Rock {
    bits: Vec<u8>,
    width: i64, // bits start at bit 4 until 4-width
}

/** we represent the rock patters as bit patterns
 * for a maximum width of the cave of 7 we need 7 bits
 * so we choose u8 type and use bits 0..7 for the cave positions
 * from right to left.
 * However since each rock starts its fall two positions from the left
 * we always leave bit 5 and 6 (left-most) zero
 */
fn parse_rocks() -> Vec<Rock> {
    let mut rocks: Vec<Rock> = Vec::new();
    for rlines in ROCKS.split("\n\n") {
        let mut rock: Vec<u8> = Vec::new();
        let mut width: i64 = 0;
        for rline in rlines.lines() {
            let mut row: u8 = 0;
            width = rline.bytes().count() as i64;
            for (i, byte) in rline.bytes().enumerate() {
                let bit = match byte {
                    b'.' => 0u8,
                    b'#' => 1u8,
                    _ => panic!("invalid rock"),
                } << (4 - i);
                row = row | bit;
            }
            rock.push(row);
        }
        rocks.push(Rock { bits: rock, width });
    }
    rocks
}

fn moved_bits(x: i64, bits: u8) -> u8 {
    let delta = x - 6;
    if delta > 0 {
        bits << delta
    } else {
        bits >> delta.abs()
    }
}

fn print_cave(cave: &Vec<u8>, rocks: usize) {
    println!("Cave with {rocks} rocks:");
    for line in cave.iter().rev() {
        if *line == 0 {
            continue;
        };
        print!("|");
        for i in (0..7).rev() {
            let mask = 1 << i;
            if mask & *line > 0 {
                print!("#");
            } else {
                print!(".");
            }
        }

        println!("|");
    }
    println!("+-------+");
}

fn run(max: usize) -> i64 {
    let rocks = parse_rocks();
    // println!("Rocks: {:?}", rocks);
    let mut top = -1i64;
    // vector of positions taken, each bit represents one position,
    // index 0 is bottom, 1 grows from bottom to top
    // we leave room for 2022 rocks each max height of 4 stacked on each other
    let mut cave: Vec<u8> = vec![0; max * 4];
    let mut i_push = 0;
    let mut num_pushes = INPUT.as_bytes().len();

    // 2022 iterations
    for i in 0..max {
        // print_cave(&cave, i);
        let r = i % rocks.len();
        let rock = &rocks[r];
        let height = rock.bits.len() as i64;
        // position of rock related to its top left (might be true or false)
        // the two positions to the right are already included in the rock's
        // encoding
        let (mut x, mut y) = (6, top + 3 + height);

        loop {
            let push = INPUT.as_bytes()[i_push % num_pushes];
            i_push += 1;
            // our coordinates go from right to left
            let xdelta = if push == b'>' { -1 } else { 1 };
            let try_x = xdelta + x;
            // println!("Try position of rock {r} for push ({try_x},{y})");
            // check if rock is outside left and right wall
            if try_x <= 8 && try_x > rock.width {
                // check if there is any collision with (try_x, y)
                let mut collision = false;
                for (row, bits) in rock.bits.iter().enumerate() {
                    let overlay = moved_bits(try_x, *bits) & cave[(y - row as i64) as usize];
                    // println!("row {row} overlay {overlay}");
                    collision = collision
                        || ((moved_bits(try_x, *bits) & cave[(y - row as i64) as usize]) != 0u8);
                    // println!("collision after row {row} is {collision}");
                }
                if !collision {
                    x = try_x;
                };
            }
            // println!("Position of rock {r} after push ({x},{y})");

            let try_y = y - 1;
            // println!("Try position of rock {r} for down ({x},{try_y})");
            let mut collision = false;
            if try_y - height + 1 >= 0 {
                // check if there is any collision with (x, try_y)
                for (row, bits) in rock.bits.iter().enumerate() {
                    collision = collision
                        || (moved_bits(x, *bits) & cave[(try_y - row as i64) as usize] != 0u8);
                }
                if !collision {
                    y = try_y;
                };
            } else {
                collision = true; // reached the bottom
            }
            if collision {
                // if there is a collision in moving down:
                // add rock to cave at current position and
                // break loop of pushes
                for (row, bits) in rock.bits.iter().enumerate() {
                    cave[(y - row as i64) as usize] |= moved_bits(x, *bits);
                }
                if y > top {
                    top = y;
                }
                // println!("==>Final position of rock {r} is ({x},{y}) top is {top}");
                break;
            }
        }
    }
    top + 1
}

// our coordinates are x: 0..6 from right to left
// and 0..top from bottom to top
fn part1() -> i64 {
    run(MAX_ROCKS)
}

fn part2() -> i64 {
    run(MAX_ROCKS2)
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
        assert_eq!(part1(), 3179);
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
