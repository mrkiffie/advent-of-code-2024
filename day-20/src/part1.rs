use grid::{Direction, Grid, Vec2};
use pathfinding::prelude::*;

const INPUT: &str = include_str!("input.txt");

#[tracing::instrument(level = "trace", skip())]
pub fn run() -> String {
    process(INPUT, 100).to_string()
}

#[tracing::instrument(level = "trace", skip(input))]
fn process(input: &str, at_least: usize) -> usize {
    let grid = Grid::new(input);
    let start = grid.index_to_vec2(input.find('S').expect("There should a start position"));
    let end = grid.index_to_vec2(input.find('E').expect("There should an end position"));

    let (path, _) = dijkstra(
        &start,
        |node| {
            grid.get_neighbours(node)
                .filter(|(_, c)| c != &'#')
                .map(|(node, _)| (node, 1))
        },
        |node| node == &end,
    )
    .expect("there should be a valid path");

    let mut cheats = 0;

    for i in 0..path.len() - 2 {
        for k in i + 4..path.len() {
            if path[i].cheatable(&path[k]) {
                // count nodes between positions
                let saved = k - i - 2;
                if saved >= at_least {
                    #[cfg(test)]
                    println!("saves: {}, from: {:?}, to {:?}", saved, &path[i], &path[k]);
                    cheats += 1;
                }
            }
        }
    }

    cheats
}

const DIRECTIONS: [grid::Direction; 4] = [Direction::N, Direction::E, Direction::S, Direction::W];
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

trait Vec2Ext {
    fn cheatable(&self, rhs: &Vec2) -> bool;
}

impl Vec2Ext for Vec2 {
    #[inline]
    fn cheatable(&self, rhs: &Vec2) -> bool {
        self.x.abs_diff(rhs.x) == 2 && self.y == rhs.y
            || self.y.abs_diff(rhs.y) == 2 && self.x == rhs.x
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = process(
            "###############
#...#...#.....#
#.#.#.#.#.###.#
#S#...#.#.#...#
#######.#.#.###
#######.#.#...#
#######.#.###.#
###..E#...#...#
###.#######.###
#...###...#...#
#.#####.#.###.#
#.#...#.#.#...#
#.#.#.#.#.#.###
#...#...#...###
###############",
            2,
        );
        assert_eq!(result, 44);
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
        super::process(INPUT, 100);
    }
}
