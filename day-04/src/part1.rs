use std::ops::Add;

const INPUT: &str = include_str!("input.txt");

#[tracing::instrument(level = "trace", skip())]
pub fn run() -> String {
    process(INPUT).to_string()
}

#[tracing::instrument(level = "trace", skip(input))]
fn process(input: &str) -> usize {
    let mut searcher = Searcher::new(input);

    let mut xmases = vec![];
    while let Some(xmas) = searcher.next() {
        if let Some(xmas) = xmas {
            xmases.push(xmas);
        }
    }

    xmases.len()
}

#[derive(Debug)]
struct Vec2 {
    x: i32,
    y: i32,
}

impl Vec2 {
    fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}

impl Add<Vec2> for Vec2 {
    type Output = Vec2;

    fn add(self, rhs: Vec2) -> Self::Output {
        Vec2 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

#[derive(Debug, Clone)]
enum Direction {
    N,
    NE,
    E,
    SE,
    S,
    SW,
    W,
    NW,
}

impl From<Direction> for Vec2 {
    fn from(direction: Direction) -> Self {
        match direction {
            Direction::N => Vec2::new(0, -1),
            Direction::NE => Vec2::new(1, -1),
            Direction::E => Vec2::new(1, 0),
            Direction::SE => Vec2::new(1, 1),
            Direction::S => Vec2::new(0, 1),
            Direction::SW => Vec2::new(-1, 1),
            Direction::W => Vec2::new(-1, 0),
            Direction::NW => Vec2::new(-1, -1),
        }
    }
}
#[derive(Debug)]
struct Grid {
    rows: usize,
    cols: usize,
    data: Vec<Vec<char>>,
}

impl Grid {
    fn new(input: &str) -> Self {
        let mut lines = input.lines();
        let cols = lines.next().expect("there should be lines").len();
        let lines = input.lines();
        let rows = lines.count();

        let data = input.lines().map(|row| row.chars().collect()).collect();

        Self { rows, cols, data }
    }
}

#[derive(Debug)]
struct Searcher {
    row_index: usize,
    col_index: usize,
    direction_index: usize,
    grid: Grid,
}

impl Searcher {
    fn new(input: &str) -> Self {
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

type Xmas = (Vec2, Direction);

impl Searcher {
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

    fn check_direction(&self, direction: Direction) -> Option<Xmas> {
        let index = Vec2::new(self.col_index as i32, self.row_index as i32);
        let x = self.get(&index)?;
        if x != 'X' {
            return None;
        }

        // Search M
        let index = index.add(direction.clone().into());
        let m = self.get(&index)?;
        if m != 'M' {
            return None;
        }

        // Search A
        let index = index.add(direction.clone().into());
        let a = self.get(&index)?;
        if a != 'A' {
            return None;
        }

        // Search S
        let index = index.add(direction.clone().into());
        let s = self.get(&index)?;
        if s != 'S' {
            return None;
        }

        Some((
            Vec2::new(self.col_index as i32, self.row_index as i32),
            direction,
        ))
    }

    fn get(&self, index: &Vec2) -> Option<char> {
        if (0..self.grid.cols as i32).contains(&index.x)
            && (0..self.grid.rows as i32).contains(&index.y)
        {
            Some(self.grid.data[index.y as usize][index.x as usize])
        } else {
            None
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
