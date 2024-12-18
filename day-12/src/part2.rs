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

    fn corners(&self, point: &Vec2) -> usize;
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

    fn corners(&self, point: &Vec2) -> usize {
        let center = &self.get(point);
        let nw = self.get(&(point + Direction::NW));
        let n = self.get(&(point + Direction::N));
        let ne = self.get(&(point + Direction::NE));
        let e = self.get(&(point + Direction::E));
        let se = self.get(&(point + Direction::SE));
        let s = self.get(&(point + Direction::S));
        let sw = self.get(&(point + Direction::SW));
        let w = self.get(&(point + Direction::W));

        [(nw, n, w), (ne, n, e), (se, s, e), (sw, s, w)]
            .iter()
            .map(|(a, b, c)| {
                let concave = center != a && center == b && center == c;
                let convex = center != a && center != b && center != c;
                let mirrored_convex = center == a && center != b && center != c;
                if concave || convex || mirrored_convex {
                    1
                } else {
                    0
                }
            })
            .sum()
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
        while self.direction_index < DIRECTIONS.len() {
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
                        queue.extend(grid.get_neighbours(&point).filter_map(
                            |(point, neighbour)| match c == neighbour {
                                true => Some(point),
                                false => None,
                            },
                        ));

                        fences += grid.corners(&point);

                        plots += 1;
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
        assert_eq!(result, 80);
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
        assert_eq!(result, 436);
    }

    #[test]
    fn example_3() {
        let result = process(
            "EEEEE
EXXXX
EEEEE
EXXXX
EEEEE",
        );
        assert_eq!(result, 236);
    }

    #[test]
    fn example_4() {
        let result = process(
            "AAAAAA
AAABBA
AAABBA
ABBAAA
ABBAAA
AAAAAA",
        );
        assert_eq!(result, 368);
    }

    #[test]
    fn example_5() {
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
        assert_eq!(result, 1206);
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
