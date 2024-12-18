use std::collections::HashSet;

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
    fn get_neighbours(&self, point: &Vec2) -> Neighbours;
}

impl GridExt for Grid<'_> {
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
            let point = &self.point + DIRECTIONS[self.direction_index];
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

    let mut visited: HashSet<Vec2> = HashSet::new();

    let mut regions: Vec<(usize, usize)> = vec![];

    let mut queue: Vec<Vec2> = vec![];

    for y in 0..grid.cols {
        for x in 0..grid.rows {
            let point = Vec2::at(x, y);
            if visited.contains(&point) {
                continue;
            } else {
                queue.push(point);
            }
            let mut fences = 0;
            let mut plots = 0;
            while let Some(point) = queue.pop() {
                if !visited.contains(&point) {
                    visited.insert(point.clone());
                    if let Some(c) = grid.get(&point) {
                        let before = queue.len();
                        queue.extend(grid.get_neighbours(&point).filter_map(
                            |(point, neighbour)| match c == neighbour {
                                true => Some(point),
                                false => None,
                            },
                        ));
                        let after = queue.len();
                        plots += 1;
                        fences += 4 - (after - before);
                    }
                }
            }

            regions.push((plots, fences));
        }
    }

    regions.iter().map(|(plots, fences)| plots * fences).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let result = process(
            "AAAA
BBCD
BBCC
EEEC",
        );
        assert_eq!(result, 140);
    }

    #[test]
    fn example_2() {
        let result = process(
            "OOOOO
OXOXO
OOOOO
OXOXO
OOOOO",
        );
        assert_eq!(result, 772);
    }

    #[test]
    fn example_3() {
        let result = process(
            "RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE",
        );
        assert_eq!(result, 1930);
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
