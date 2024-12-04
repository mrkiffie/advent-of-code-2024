use std::ops::Add;

use grid::{Direction, Grid, Vec2};

const INPUT: &str = include_str!("input.txt");

#[tracing::instrument(level = "trace", skip())]
pub fn run() -> String {
    process(INPUT).to_string()
}

#[tracing::instrument(level = "trace", skip(input))]
fn process(input: &str) -> usize {
    let mut searcher = Searcher::new(input);

    let mut xmases = 0;
    while let Some(xmas) = searcher.next() {
        if xmas.is_some() {
            xmases += 1
        }
    }

    xmases
}

#[derive(Debug)]
struct Searcher<'a> {
    row_index: usize,
    col_index: usize,
    direction_index: usize,
    grid: Grid<'a>,
}

impl<'a> Searcher<'a> {
    fn new(input: &'a str) -> Self {
        Self {
            row_index: 0,
            col_index: 0,
            direction_index: 0,
            grid: Grid::new(input),
        }
    }
}

const DIRECTIONS: [Direction; 8] = [
    Direction::N,
    Direction::NE,
    Direction::E,
    Direction::SE,
    Direction::S,
    Direction::SW,
    Direction::W,
    Direction::NW,
];

type Xmas = ();

impl Searcher<'_> {
    fn next(&mut self) -> Option<Option<Xmas>> {
        if self.row_index == self.grid.rows {
            return None;
        }
        let direction = DIRECTIONS[self.direction_index].clone();
        let result = self.check_direction(direction);
        self.direction_index += 1;

        if self.direction_index == DIRECTIONS.len() {
            self.direction_index = 0;
            self.col_index += 1;
        }

        if self.col_index == self.grid.cols {
            self.col_index = 0;
            self.row_index += 1;
        }

        Some(result)
    }

    fn check_direction(&self, direction: Direction) -> Option<()> {
        let index = Vec2::new(self.col_index as i32, self.row_index as i32);
        let x = self.grid.get(&index)?;
        if x != 'X' {
            return None;
        }

        // Search M
        let index = &index.add(&direction);
        let m = self.grid.get(index)?;
        if m != 'M' {
            return None;
        }

        // Search A
        let index = index.add(&direction);
        let a = self.grid.get(&index)?;
        if a != 'A' {
            return None;
        }

        // Search S
        let index = index.add(&direction);
        let s = self.grid.get(&index)?;
        if s != 'S' {
            return None;
        }

        Some(())
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
