use grid::{Direction, Grid, Vec2};

const INPUT: &str = include_str!("input.txt");

#[tracing::instrument(level = "trace", skip())]
pub fn run() -> String {
    process(INPUT).to_string()
}

const DIRECTIONS: [grid::Direction; 4] = [Direction::N, Direction::E, Direction::S, Direction::W];

trait Vec2Ext {
    fn at(x: usize, y: usize) -> Vec2;
}

impl Vec2Ext for Vec2 {
    fn at(x: usize, y: usize) -> Vec2 {
        Vec2::new(x as i32, y as i32)
    }
}

trait GridExt {
    fn get_digit(&self, point: &Vec2) -> Option<u32>;

    fn get_neighbours(&self, point: &Vec2) -> Vec<Vec2>;
}

impl GridExt for Grid<'_> {
    fn get_digit(&self, point: &Vec2) -> Option<u32> {
        self.get(point).and_then(|c| c.to_digit(10))
    }

    fn get_neighbours(&self, point: &Vec2) -> Vec<Vec2> {
        if let Some(digit) = self.get_digit(point) {
            DIRECTIONS
                .iter()
                .map(|direction| point + direction)
                .filter_map(|neighbour| {
                    if let Some(next_digit) = self.get_digit(&neighbour) {
                        if next_digit == digit + 1 {
                            return Some(neighbour);
                        }
                    }
                    None
                })
                .collect::<Vec<Vec2>>()
        } else {
            vec![]
        }
    }
}

#[tracing::instrument(level = "trace", skip(input))]
fn process(input: &str) -> usize {
    let grid = Grid::new(input);
    let mut counter = 0;

    let mut queue: Vec<Vec2> = Vec::with_capacity(100);
    for y in 0..grid.rows {
        for x in 0..grid.cols {
            let mut ends = 0;
            let current = Vec2::at(x, y);
            if let Some(0) = grid.get_digit(&current) {
                queue.push(current.clone());
            }
            while let Some(point) = queue.pop() {
                if let Some(9) = grid.get_digit(&point) {
                    ends += 1;
                }
                for n in grid.get_neighbours(&point) {
                    queue.push(n);
                }
            }
            counter += ends;
        }
    }

    counter
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = process(
            "89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732",
        );
        assert_eq!(result, 81);
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
