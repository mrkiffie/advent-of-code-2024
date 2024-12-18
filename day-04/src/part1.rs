use rayon::prelude::*;
use std::ops::Add;

use grid::{Direction, Grid, Vec2};

const INPUT: &str = include_str!("input.txt");

#[tracing::instrument(level = "trace", skip())]
pub fn run() -> String {
    process(INPUT).to_string()
}

#[tracing::instrument(level = "trace", skip(input))]
fn process(input: &str) -> usize {
    Searcher::new(input).count()
}

#[derive(Debug)]
struct Searcher<'a> {
    grid: Grid<'a>,
}

impl<'a> Searcher<'a> {
    fn new(input: &'a str) -> Self {
        Self {
            grid: Grid::new(input),
        }
    }
}

impl Searcher<'_> {
    fn count(&mut self) -> usize {
        [Direction::E, Direction::S, Direction::SE, Direction::SW]
            .par_iter()
            .map(|direction| {
                let mut counter = 0;
                for x in 0..self.grid.cols {
                    for y in 0..self.grid.rows {
                        if let Some(increment) =
                            self.check(Vec2::new(x as i32, y as i32), direction)
                        {
                            counter += increment
                        }
                    }
                }
                counter
            })
            .sum()
    }

    fn check(&self, index: Vec2, direction: &Direction) -> Option<usize> {
        let x = self.grid.get(&index)?;
        let index = index.add(direction);
        let m = self.grid.get(&index)?;
        let index = index.add(direction);
        let a = self.grid.get(&index)?;
        let index = index.add(direction);
        let s = self.grid.get(&index)?;

        match (x, m, a, s) {
            ('X', 'M', 'A', 'S') | ('S', 'A', 'M', 'X') => Some(1),
            _ => Some(0),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = process(
            "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX",
        );
        assert_eq!(result, 18);
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
