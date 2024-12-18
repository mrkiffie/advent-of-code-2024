use grid::{Direction, Grid};
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use std::{collections::BTreeSet, ops::Add};

const INPUT: &str = include_str!("input.txt");

#[tracing::instrument(level = "trace", skip())]
pub fn run() -> String {
    process(INPUT).to_string()
}

#[tracing::instrument(level = "trace", skip(input))]
fn process(input: &str) -> u32 {
    let get_direction = |i: u8| match i {
        x if x % 4 == 0 => Direction::N,
        x if x % 4 == 1 => Direction::E,
        x if x % 4 == 2 => Direction::S,
        x if x % 4 == 3 => Direction::W,
        _ => unreachable!(),
    };

    let grid = Grid::new(input);

    let start_index = input.find('^').expect("start point should exist");

    let start = grid.index_to_vec2(start_index);

    let mut stepper = start.clone();
    let mut stepper_direction = 0;

    let mut visited: BTreeSet<u32> = BTreeSet::new();

    loop {
        // step
        let next = stepper.add(get_direction(stepper_direction));
        let Some(c) = grid.get(&next) else {
            break;
        };

        if c == '#' {
            // turn
            stepper_direction += 1;
            stepper_direction &= 3;
        } else {
            // update map
            visited.insert(grid.point_to_index(&next).expect("point should be valid") as u32);
            stepper = next;
        }
    }

    visited
        .par_iter()
        .map(|&obstacle_index| {
            let obstacle = grid.index_to_vec2(obstacle_index as usize);

            // Resume from previous fork or start at the beginning
            let mut double_stepper = start.clone();
            let mut single_stepper = start.clone();
            // Direction index
            let mut double_stepper_direction = 0;
            let mut single_stepper_direction = 0;

            // loop detection makes use of 2 walkers following the path. The one
            // walks twice as fast as the other. If they meet on the same spot and
            // are facing the same direction, we've hit a loop. If the fast walker
            // leaves the map, there is no loop.
            loop {
                // double stepper step 1
                {
                    let next = double_stepper.add(get_direction(double_stepper_direction));
                    let Some(c) = grid.get(&next) else {
                        return 0;
                    };

                    if c == '#' || obstacle == next {
                        // turn
                        double_stepper_direction += 1;
                        double_stepper_direction &= 3;
                    } else {
                        double_stepper = next;
                    }
                }

                // double stepper step 2
                {
                    let next = double_stepper.add(get_direction(double_stepper_direction));
                    let Some(c) = grid.get(&next) else {
                        return 0;
                    };

                    if c == '#' || obstacle == next {
                        // turn
                        double_stepper_direction += 1;
                        double_stepper_direction &= 3;
                    } else {
                        double_stepper = next;
                    }
                }

                // single stepper
                {
                    let next = single_stepper.add(get_direction(single_stepper_direction));
                    let Some(c) = grid.get(&next) else {
                        return 0;
                    };

                    if c == '#' || obstacle == next {
                        // turn
                        single_stepper_direction += 1;
                        single_stepper_direction &= 3;
                    } else {
                        single_stepper = next;
                    }
                }

                // has double stepper caught up with single stepper?
                if single_stepper == double_stepper
                    && single_stepper_direction == double_stepper_direction
                {
                    return 1;
                }
            }
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = process(
            "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...",
        );
        assert_eq!(result, 6);
    }
}

#[cfg(feature = "bench")]
pub mod benchmarks {
    use super::INPUT;

    pub fn main() {
        divan::main();
    }

    #[divan::bench()]
    fn bench_process() {
        super::process(INPUT);
    }
}
