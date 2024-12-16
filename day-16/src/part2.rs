use std::collections::{HashMap, HashSet, VecDeque};

use grid::{Direction, Grid, Vec2};

const INPUT: &str = include_str!("input.txt");

#[tracing::instrument(level = "trace", skip())]
pub fn run() -> usize {
    process(INPUT)
}

#[derive(Debug, Clone)]
struct Node {
    score: usize,
    direction: Direction,
    point: Vec2,
    parents: Vec<(Direction, Vec2)>,
}
impl Node {
    fn new(
        score: usize,
        direction: Direction,
        point: Vec2,
        parents: Vec<(Direction, Vec2)>,
    ) -> Self {
        Self {
            score,
            direction,
            point,
            parents,
        }
    }
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

    let mut visited: HashMap<(Direction, Vec2), Node> = HashMap::new();
    visited.insert(
        (direction, start.clone()),
        Node::new(0, direction, start.clone(), vec![]),
    );

    // let mut counter = 0;

    while let Some((current_direction, current_point)) = queue.pop_front() {
        // println!("counter: {}", counter);
        // println!(
        //     "processing queue item: {:?} [{}, {}]",
        //     current_direction, current_point.x, current_point.y
        // );
        // counter += 1;
        let current = visited
            .get(&(current_direction, current_point.clone()))
            .expect("node should exist")
            .clone();

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
            let next_score = current.score + score_delta;

            // track scores and parents
            visited
                .entry((next_direction, next_point.clone()))
                .and_modify(|previous| {
                    match next_score.cmp(&previous.score) {
                        std::cmp::Ordering::Less => {
                            // println!(
                            //     "replacing previous score {} with {} for {:?} [{}, {}]",
                            //     previous.score,
                            //     next_score,
                            //     next_direction,
                            //     next_point.x,
                            //     next_point.y
                            // );
                            previous.score = next_score;
                            previous.parents.clear();
                            previous
                                .parents
                                .push((current_direction, current_point.clone()));
                            queue.push_back((next_direction, next_point.clone()));
                        }
                        std::cmp::Ordering::Equal => {
                            previous
                                .parents
                                .push((current_direction, current_point.clone()));
                        }
                        std::cmp::Ordering::Greater => {
                            // Nothing to do
                        }
                    }
                })
                .or_insert_with(|| {
                    // println!(
                    //     "adding item to queue: {:?} [{}, {}], score: {}",
                    //     next_direction, next_point.x, next_point.y, next_score
                    // );
                    // add to queue it it hasn't been explored
                    queue.push_back((next_direction, next_point.clone()));
                    Node::new(
                        next_score,
                        next_direction,
                        next_point,
                        vec![(current_direction, current_point.clone())],
                    )
                });
        }
    }

    let end = visited
        .iter()
        .filter_map(
            |((_, point), node)| {
                if *point == end {
                    Some(node)
                } else {
                    None
                }
            },
        )
        .reduce(|min, node| if min.score > node.score { node } else { min })
        .expect("end node should exist");

    let mut paths: HashSet<Vec2> = HashSet::new();
    let mut queue = VecDeque::new();

    queue.push_back((end.direction, end.point.clone()));

    while let Some(node) = queue.pop_front() {
        if let Some(node) = visited.get(&node) {
            paths.insert(node.point.clone());
            for parent in node.parents.clone() {
                queue.push_back(parent);
            }
        }
    }

    paths.len()
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
