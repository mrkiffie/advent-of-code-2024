use grid::{Direction, Grid, Vec2};

const INPUT: &str = include_str!("input.txt");

#[tracing::instrument(level = "trace", skip())]
pub fn run() -> usize {
    process(INPUT)
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

    fn get_neighbours(&self, point: &Vec2) -> Neighbours;
}

impl GridExt for Grid<'_> {
    #[inline]
    fn get_digit(&self, point: &Vec2) -> Option<u32> {
        self.get(point).and_then(|c| c.to_digit(10))
    }

    #[inline]
    fn get_neighbours(&self, point: &Vec2) -> Neighbours {
        Neighbours {
            grid: self,
            point: point.clone(),
            direction_index: 0,
        }
    }
}

struct Neighbours<'a> {
    grid: &'a Grid<'a>,
    point: Vec2,
    direction_index: usize,
}

impl Iterator for Neighbours<'_> {
    type Item = (Vec2, char);

    fn next(&mut self) -> Option<Self::Item> {
        while self.direction_index < 4 {
            let point = &self.point + &DIRECTIONS[self.direction_index];
            self.direction_index += 1;
            if let Some(c) = self.grid.get(&point) {
                return Some((point, c));
            }
        }
        None
    }
}

#[tracing::instrument(level = "trace", skip(input))]
fn process(input: &str) -> usize {
    let grid = Grid::new(input);
    let mut counter = 0;
    let mut queue: Vec<Vec2> = Vec::with_capacity(16);
    for y in 0..grid.rows {
        for x in 0..grid.cols {
            let current = Vec2::at(x, y);
            if let Some(0) = grid.get_digit(&current) {
                queue.push(current);
            }
            while let Some(point) = queue.pop() {
                match grid.get_digit(&point) {
                    Some(9) => {
                        counter += 1;
                    }
                    Some(previous) => {
                        let neighbours = grid
                            .get_neighbours(&point)
                            .filter(|(_, c)| {
                                c.to_digit(10)
                                    .map(|height| height == previous + 1)
                                    .unwrap_or_default()
                            })
                            .map(|(neighbour, _)| neighbour);
                        for neighbour in neighbours {
                            queue.push(neighbour);
                        }
                    }
                    _ => {}
                }
            }
        }
    }
    counter
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_neighbours() {
        let grid = Grid::new(
            "abc
def
ghi",
        );

        let mut neighbours = grid.get_neighbours(&Vec2::new(1, 1));

        assert_eq!(neighbours.next(), Some((Vec2::new(1, 0), 'b')));
        assert_eq!(neighbours.next(), Some((Vec2::new(2, 1), 'f')));
        assert_eq!(neighbours.next(), Some((Vec2::new(1, 2), 'h')));
        assert_eq!(neighbours.next(), Some((Vec2::new(0, 1), 'd')));
        assert_eq!(neighbours.next(), None);

        let mut neighbours = grid.get_neighbours(&Vec2::new(0, 0));

        assert_eq!(neighbours.next(), Some((Vec2::new(1, 0), 'b')));
        assert_eq!(neighbours.next(), Some((Vec2::new(0, 1), 'd')));
        assert_eq!(neighbours.next(), None);
    }

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
