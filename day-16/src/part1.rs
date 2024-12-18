use std::collections::{HashMap, VecDeque};

use grid::{Direction, Grid, Vec2};

const INPUT: &str = include_str!("input.txt");

#[tracing::instrument(level = "trace", skip())]
pub fn run() -> String {
    process(INPUT).to_string()
}

#[tracing::instrument(level = "trace", skip(input))]
fn process(input: &str) -> usize {
    let grid = Grid::new(input.trim());
    let start = Vec2::at(1, grid.rows - 2);
    let end = Vec2::at(grid.cols - 2, 1);

    assert!(grid.get(&start) == Some('S'));
    assert!(grid.get(&end) == Some('E'));

    let direction = Direction::E;

    let mut queue: VecDeque<(Direction, Vec2)> = VecDeque::new();
    queue.push_back((direction, start.clone()));

    let mut visited: HashMap<(Direction, Vec2), usize> = HashMap::new();
    visited.insert((direction, start.clone()), 0);

    // let mut counter = 0;

    while let Some((current_direction, current_point)) = queue.pop_front() {
        // println!("counter: {}", counter);
        // println!(
        //     "processing queue item: {:?} [{}, {}]",
        //     current_direction, current_point.x, current_point.y
        // );
        // counter += 1;
        let current_score = *visited
            .get(&(current_direction, current_point.clone()))
            .unwrap_or(&usize::MAX);

        for (next_point, next_direction) in grid.get_neighbours(&current_point.clone()) {
            let score_delta = match (&current_direction, &next_direction) {
                (Direction::N, Direction::N) => Scores::FORWARD,
                (Direction::N, Direction::E) => Scores::TURN,
                (Direction::N, Direction::S) => Scores::BACK,
                (Direction::N, Direction::W) => Scores::TURN,
                (Direction::E, Direction::N) => Scores::TURN,
                (Direction::E, Direction::E) => Scores::FORWARD,
                (Direction::E, Direction::S) => Scores::TURN,
                (Direction::E, Direction::W) => Scores::BACK,
                (Direction::S, Direction::N) => Scores::BACK,
                (Direction::S, Direction::E) => Scores::TURN,
                (Direction::S, Direction::S) => Scores::FORWARD,
                (Direction::S, Direction::W) => Scores::TURN,
                (Direction::W, Direction::N) => Scores::TURN,
                (Direction::W, Direction::E) => Scores::BACK,
                (Direction::W, Direction::S) => Scores::TURN,
                (Direction::W, Direction::W) => Scores::FORWARD,
                _ => unreachable!("invalid directions"),
            };
            let next_score = current_score + score_delta;
            // track scores
            visited
                .entry((next_direction, next_point.clone()))
                .and_modify(|previous_score| {
                    if *previous_score > next_score {
                        // println!(
                        //     "replacing previous score {} with {} for {:?} [{}, {}]",
                        //     previous_score, next_score, next_direction, next_point.x, next_point.y
                        // );
                        *previous_score = next_score;
                        queue.push_back((next_direction, next_point.clone()));
                    }
                })
                .or_insert_with(|| {
                    // println!(
                    //     "adding item to queue: {:?} [{}, {}], score: {}",
                    //     next_direction, next_point.x, next_point.y, next_score
                    // );
                    // add to queue it it hasn't been explored
                    queue.push_back((next_direction, next_point));
                    next_score
                });
        }
    }

    let min = *visited
        .iter()
        .filter_map(
            |((_, point), score)| {
                if *point == end {
                    Some(score)
                } else {
                    None
                }
            },
        )
        .min()
        .expect("there should be a min score");

    min
}

const DIRECTIONS: [grid::Direction; 4] = [Direction::N, Direction::E, Direction::S, Direction::W];

struct Scores;

impl Scores {
    const FORWARD: usize = 1;
    const TURN: usize = 1001;
    const BACK: usize = 2001;
}

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
    type Item = (Vec2, Direction);

    fn next(&mut self) -> Option<Self::Item> {
        while self.direction_index < 4 {
            let direction = DIRECTIONS[self.direction_index];
            let point = &self.point + direction;
            self.direction_index += 1;
            if let Some(c) = self.grid.get(&point) {
                if c != '#' {
                    return Some((point, direction));
                }
            }
        }
        None
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
        assert_eq!(result, 7036);
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
        assert_eq!(result, 11048);
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
