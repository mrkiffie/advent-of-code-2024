use grid::{Direction, Grid};
use std::ops::Add;

const INPUT: &str = include_str!("input.txt");

#[tracing::instrument(level = "trace", skip())]
pub fn run() -> String {
    process(INPUT).to_string()
}

#[tracing::instrument(level = "trace", skip(input))]
fn process(input: &str) -> u32 {
    let get_direction = |i: usize| match i {
        x if x % 4 == 0 => Direction::N,
        x if x % 4 == 1 => Direction::E,
        x if x % 4 == 2 => Direction::S,
        x if x % 4 == 3 => Direction::W,
        _ => unreachable!(),
    };

    let grid = Grid::new(input);
    let mut map: Vec<u8> = Vec::with_capacity(grid.cols * grid.rows);
    map.extend(input.lines().flat_map(|line| line.chars().map(|_| 0)));

    let mut current = grid.index_to_vec2(input.find('^').expect("start point should exist"));
    // Direction index
    let mut i = 0;

    loop {
        // update map
        map[current.x as usize + current.y as usize * grid.cols] = 1;

        // step
        let next = current.add(get_direction(i));
        let Some(c) = grid.get(&next) else {
            break;
        };

        if c == '#' {
            // turn
            i += 1;
        } else {
            current = next;
        }
    }

    map.iter().fold(0, |total, x| total + *x as u32)
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
        assert_eq!(result, 41);
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
