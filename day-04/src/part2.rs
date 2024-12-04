use grid::{Direction, Grid, Vec2};
use std::ops::Add;

const INPUT: &str = include_str!("input.txt");

#[tracing::instrument(level = "trace", skip())]
pub fn run() -> String {
    process(INPUT).to_string()
}

#[tracing::instrument(level = "trace", skip(input))]
fn process(input: &str) -> usize {
    let mut searcher = Searcher::new(input);

    let mut counter = 0;
    while let Some(xmas) = searcher.next() {
        if xmas.is_some() {
            counter += 1;
        }
    }

    counter
}

#[derive(Debug)]
struct Searcher<'a> {
    row_index: usize,
    col_index: usize,
    grid: Grid<'a>,
}

impl<'a> Searcher<'a> {
    fn new(input: &'a str) -> Self {
        Self {
            row_index: 1,
            col_index: 1,
            grid: Grid::new(input),
        }
    }
}

type Xmas = ();

impl Searcher<'_> {
    fn next(&mut self) -> Option<Option<Xmas>> {
        if self.row_index == self.grid.rows - 1 {
            return None;
        }
        let result = self.check();
        self.col_index += 1;

        if self.col_index == self.grid.cols - 1 {
            self.col_index = 0;
            self.row_index += 1;
        }

        Some(result)
    }

    fn check(&self) -> Option<Xmas> {
        let a_index = Vec2::new(self.col_index as i32, self.row_index as i32);
        let a = self.grid.get(&a_index)?;
        if a != 'A' {
            return None;
        }

        let index = a_index.add(Direction::NW);
        let nw = self.grid.get(&index)?;

        let index = a_index.add(Direction::SE);
        let se = self.grid.get(&index)?;

        let index = a_index.add(Direction::NE);
        let ne = self.grid.get(&index)?;

        let index = a_index.add(Direction::SW);
        let sw = self.grid.get(&index)?;

        match (nw, se) {
            ('M', 'S') => {}
            ('S', 'M') => {}
            (_, _) => {
                return None;
            }
        }

        match (ne, sw) {
            ('M', 'S') => {}
            ('S', 'M') => {}
            (_, _) => {
                return None;
            }
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
        assert_eq!(result, 9);
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
