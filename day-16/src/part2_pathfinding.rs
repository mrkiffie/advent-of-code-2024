use std::collections::HashSet;

use grid::{Grid, Vec2};
use pathfinding::prelude::astar_bag;

const INPUT: &str = include_str!("input.txt");

#[tracing::instrument(level = "trace", skip())]
pub fn run() -> usize {
    process(INPUT)
}

#[tracing::instrument(level = "trace", skip(input))]
fn process(input: &str) -> usize {
    let grid = Grid::new(input.trim());
    let start = Vec2::new(1, grid.rows as i32 - 2);
    let end = Vec2::new(grid.cols as i32 - 2, 1);

    assert!(grid.get(&start) == Some('S'));
    assert!(grid.get(&end) == Some('E'));

    let direction = Vec2::X;

    let (paths, _cost) = astar_bag(
        &(start, direction),
        |(node, direction): &(Vec2, Vec2)| {
            let mut choices = vec![
                ((node.clone(), direction.rotate()), 1000),
                ((node.clone(), -direction.rotate()), 1000),
            ];
            let next = grid.get(&(node + direction));
            if next == Some('.') || next == Some('E') {
                choices.push(((node + direction, direction.clone()), 1))
            }
            choices
        },
        |_| 0,
        |(node, _)| *node == end,
    )
    .expect("there should be some paths");

    let nodes = paths
        .into_iter()
        .flat_map(|path| path.into_iter().map(|(node, _)| node))
        .collect::<HashSet<_>>();
    nodes.len()
}

trait Vec2Ext {
    const X: Vec2;

    fn rotate(&self) -> Vec2;
}

impl Vec2Ext for Vec2 {
    const X: Vec2 = Vec2 { x: 1, y: 0 };

    fn rotate(&self) -> Self {
        Self {
            x: -self.y,
            y: self.x,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let result = process(
            "###############
#.......#....E#
#.#.###.#.###.#
#.....#.#...#.#
#.###.#####.#.#
#.#.#.......#.#
#.#.#####.###.#
#...........#.#
###.#.#####.#.#
#...#.....#.#.#
#.#.#.###.#.#.#
#.....#...#.#.#
#.###.#.#.#.#.#
#S..#.....#...#
###############",
        );
        assert_eq!(result, 45);
    }

    #[test]
    fn example_2() {
        let result = process(
            "#################
#...#...#...#..E#
#.#.#.#.#.#.#.#.#
#.#.#.#...#...#.#
#.#.#.#.###.#.#.#
#...#.#.#.....#.#
#.#.#.#.#.#####.#
#.#...#.#.#.....#
#.#.#####.#.###.#
#.#.#.......#...#
#.#.###.#####.###
#.#.#...#.....#.#
#.#.#.#####.###.#
#.#.#.........#.#
#.#.#.#########.#
#S#.............#
#################",
        );
        assert_eq!(result, 64);
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
