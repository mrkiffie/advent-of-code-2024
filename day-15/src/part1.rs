use glam::IVec2;
use std::collections::HashMap;

const INPUT: &str = include_str!("input.txt");

#[tracing::instrument(level = "trace", skip())]
pub fn run() -> usize {
    process(INPUT)
}

#[tracing::instrument(level = "trace", skip(input))]
fn process(input: &str) -> usize {
    let (mut map, moves) = parse(input);

    let mut robot = *map
        .iter()
        .filter_map(|(pos, entity)| match entity {
            Entity::Robot => Some(pos),
            _ => None,
        })
        .next()
        .expect("there should be a robot");

    for c in moves.chars() {
        if c == '\n' {
            continue;
        }

        let direction = Direction::from_char(c);
        let mut next_pos = robot + direction;
        loop {
            if let Some(entity) = map.get(&next_pos) {
                match entity {
                    // Can't perform movement
                    Entity::Wall => {
                        break;
                    }
                    Entity::Box => next_pos += &direction,
                    _ => unreachable!(),
                }
            } else {
                // space available - perform swap
                map.insert(next_pos, Entity::Box);
                map.remove(&robot);
                robot += direction;
                map.insert(robot, Entity::Robot);
                break;
            }
        }
    }

    map.iter()
        .filter_map(|(pos, entity)| match entity {
            Entity::Wall => None,
            Entity::Box => Some((pos.y * 100 + pos.x) as usize),
            Entity::Robot => None,
        })
        .sum()
}

struct Direction;

impl Direction {
    const NORTH: IVec2 = IVec2::NEG_Y;
    const SOUTH: IVec2 = IVec2::Y;
    const EAST: IVec2 = IVec2::X;
    const WEST: IVec2 = IVec2::NEG_X;

    fn from_char(c: char) -> IVec2 {
        match c {
            '^' => Direction::NORTH,
            'v' => Direction::SOUTH,
            '<' => Direction::WEST,
            '>' => Direction::EAST,
            _ => unimplemented!("This character is not expected {}", c),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
enum Entity {
    Wall,
    Box,
    Robot,
}

fn parse(input: &str) -> (HashMap<IVec2, Entity>, &str) {
    let (map, moves) = input.split_once("\n\n").unwrap();

    let map = map
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars().enumerate().filter_map(move |(x, c)| {
                let x = x as i32;
                let y = y as i32;
                let pos = IVec2::new(x, y);
                match c {
                    '#' => Some((pos, Entity::Wall)),
                    'O' => Some((pos, Entity::Box)),
                    '@' => Some((pos, Entity::Robot)),
                    _ => None,
                }
            })
        })
        .collect::<HashMap<IVec2, Entity>>();

    (map, moves)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn small_example() {
        let result = process(
            "########
#..O.O.#
##@.O..#
#...O..#
#.#.O..#
#...O..#
#......#
########

<^^>>>vv<v>>v<<",
        );
        assert_eq!(result, 2028);
    }

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
        assert_eq!(result, 10092);
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
