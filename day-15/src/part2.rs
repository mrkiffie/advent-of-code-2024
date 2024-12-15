use std::collections::HashMap;

use glam::I64Vec2;

const INPUT: &str = include_str!("input.txt");

#[tracing::instrument(level = "trace", skip())]
pub fn run() -> usize {
    process(INPUT)
}

#[tracing::instrument(level = "trace", skip(input))]
fn process(input: &str) -> usize {
    let (mut entities, moves) = parse(input);

    let mut stack: Vec<usize> = vec![];
    let mut to_check: Vec<I64Vec2> = vec![];

    for direction in moves.into_iter() {
        let (index, robot) = entities
            .iter()
            .enumerate()
            .find(|(_, entity)| matches!(entity, Entity::Robot(..)))
            .expect("there should be a robot");
        let pos = match robot {
            Entity::Robot(pos) => *pos,
            _ => unreachable!(),
        };
        stack.push(index);
        to_check.push(pos + direction);

        let mut can_move = true;

        while let Some(pos) = to_check.pop() {
            if !can_move {
                stack.clear();
                to_check.clear();
                break;
            }
            let next_pos = entities
                .iter()
                .enumerate()
                .find(|(_, entity)| match entity {
                    Entity::Wall(left, right) | Entity::Box(left, right) => {
                        *left == pos || *right == pos
                    }
                    _ => false,
                });

            if let Some((index, entity)) = next_pos {
                match entity {
                    // Can't perform movement
                    Entity::Wall(..) => {
                        stack.clear();
                        to_check.clear();
                        can_move = false;
                        break;
                    }
                    Entity::Box(left, right) => {
                        stack.push(index);
                        let next_left = *left + direction;
                        if next_left != pos {
                            to_check.push(*left + direction);
                        }
                        let next_right = *right + direction;
                        if next_right != pos {
                            to_check.push(*right + direction);
                        }
                    }
                    _ => unreachable!(),
                }
            }
        }

        if can_move {
            for (_, entity) in entities
                .iter_mut()
                .enumerate()
                .filter(|(index, _)| stack.contains(index))
            {
                entity.translate(direction);
            }
        }
        stack.clear();
        to_check.clear();
    }

    entities
        .iter()
        .filter_map(|entity| match entity {
            Entity::Box(left, _) => Some((left.y * 100 + left.x) as usize),
            _ => None,
        })
        .sum()
}

#[allow(dead_code)]
fn debug_map(map: &[Entity]) {
    let max = map.iter().fold(I64Vec2::ZERO, |max, entity| {
        let pos = match entity {
            Entity::Wall(_, right) => right,
            Entity::Box(_, right) => right,
            Entity::Robot(pos) => pos,
        };
        I64Vec2 {
            x: max.x.max(pos.x),
            y: max.y.max(pos.y),
        }
    });
    println!();

    let map = map.iter().fold(HashMap::new(), |mut acc, entity| {
        match entity {
            Entity::Wall(left, right) => {
                acc.insert(left, '#');
                acc.insert(right, '#');
            }
            Entity::Box(left, right) => {
                acc.insert(left, '[');
                acc.insert(right, ']');
            }
            Entity::Robot(pos) => {
                acc.insert(pos, '@');
            }
        };

        acc
    });

    for y in 0..=max.y {
        for x in 0..=max.x {
            match map.get(&I64Vec2 { x, y }) {
                Some(c) => print!("{}", c),
                None => print!("."),
            };
        }
        println!();
    }
    println!();
}

struct Direction;

impl Direction {
    const NORTH: I64Vec2 = I64Vec2::NEG_Y;
    const SOUTH: I64Vec2 = I64Vec2::Y;
    const EAST: I64Vec2 = I64Vec2::X;
    const WEST: I64Vec2 = I64Vec2::NEG_X;
}

#[derive(Debug, PartialEq, Eq)]
enum Entity {
    Wall(I64Vec2, I64Vec2),
    Box(I64Vec2, I64Vec2),
    Robot(I64Vec2),
}

impl Entity {
    fn translate(&mut self, direction: I64Vec2) {
        match self {
            Entity::Wall(left, right) => {
                *left += direction;
                *right += direction;
            }
            Entity::Box(left, right) => {
                *left += direction;
                *right += direction;
            }
            Entity::Robot(pos) => {
                *pos += direction;
            }
        }
    }
}

fn parse(input: &str) -> (Vec<Entity>, Vec<I64Vec2>) {
    let (map, moves) = input.split_once("\n\n").unwrap();

    let entities = map
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars().enumerate().filter_map(move |(x, c)| {
                let x = x as i64 * 2;
                let y = y as i64;
                let pos = I64Vec2::new(x, y);
                match c {
                    '#' => Some(Entity::Wall(pos, pos + Direction::EAST)),
                    'O' => Some(Entity::Box(pos, pos + Direction::EAST)),
                    '@' => Some(Entity::Robot(pos)),
                    _ => None,
                }
            })
        })
        .collect::<Vec<Entity>>();

    let moves = moves
        .chars()
        .filter_map(|c| match c {
            '^' => Some(Direction::NORTH),
            'v' => Some(Direction::SOUTH),
            '<' => Some(Direction::WEST),
            '>' => Some(Direction::EAST),
            _ => None,
        })
        .collect::<Vec<I64Vec2>>();

    (entities, moves)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn larger_example() {
        let result = process(
            "##########
#..O..O.O#
#......O.#
#.OO..O.O#
#..O@..O.#
#O#..O...#
#O..O..O.#
#.OO.O.OO#
#....O...#
##########

<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
>^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^",
        );
        assert_eq!(result, 9021);
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
