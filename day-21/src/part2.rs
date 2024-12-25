use glam::IVec2;
use std::collections::{HashMap, VecDeque};
const INPUT: &str = include_str!("input.txt");

#[tracing::instrument(level = "trace", skip())]
pub fn run() -> String {
    process(INPUT).to_string()
}

type Keypad = Vec<[char; 3]>;
#[tracing::instrument(level = "trace", skip(input))]
fn process(input: &str) -> usize {
    let numeric_keypad: Keypad = vec![
        ['7', '8', '9'],
        ['4', '5', '6'],
        ['1', '2', '3'],
        [' ', '0', 'A'],
    ];
    let arrow_keypad: Keypad = vec![[' ', '^', 'A'], ['<', 'v', '>']];

    let mut cache: Cache = HashMap::new();

    let total = input
        .lines()
        .map(|code| {
            let max_depth = 25;
            let mut state = vec!['A'; max_depth + 1];

            let num = code
                .strip_suffix('A')
                .and_then(|s| s.parse::<usize>().ok())
                .expect("number should be parseable");

            let sequence = code.chars().collect::<Vec<_>>();

            let count = get_shortest_sequence(
                &sequence,
                max_depth,
                &mut cache,
                &numeric_keypad,
                &arrow_keypad,
                true,
                &mut state,
            );

            count * num
        })
        .sum();

    total
}

const DIRECTIONS: [IVec2; 4] = [IVec2::X, IVec2::Y, IVec2::NEG_X, IVec2::NEG_Y];

fn get_paths(keypad: &Keypad, from: char, to: char) -> Vec<Vec<char>> {
    let mut start = IVec2::new(0, 0);
    let mut end = IVec2::new(0, 0);
    for (y, row) in keypad.iter().enumerate() {
        for (x, key) in row.iter().enumerate() {
            if key == &from {
                start = IVec2::new(x as i32, y as i32);
            }
            if key == &to {
                end = IVec2::new(x as i32, y as i32)
            }
        }
    }

    if from == to {
        return vec![vec!['A']];
    }

    // determine distances from start
    let mut distances = vec![[usize::MAX; 3]; keypad.len()];
    let mut queue = VecDeque::new();
    queue.push_back((start, 0));
    while let Some((current, distance)) = queue.pop_front() {
        distances[current.y as usize][current.x as usize] = distance;

        for direction in DIRECTIONS {
            let next = current + direction;

            if within_bounds(keypad, &next)
                && get(keypad, &next) != &' '
                && get(&distances, &next) == &usize::MAX
            {
                queue.push_back((next, distance + 1));
            }
        }
    }

    // build paths from end to start
    let mut paths = vec![];
    let mut stack = vec![];

    stack.push((end, vec!['A']));

    while let Some((current, path)) = stack.pop() {
        if current == start {
            paths.push(path);
            continue;
        }

        for (i, direction) in DIRECTIONS.iter().enumerate() {
            let next = current + direction;

            if within_bounds(keypad, &next) && get(&distances, &next) < get(&distances, &current) {
                let key = match i {
                    0 => '<',
                    1 => '^',
                    2 => '>',
                    3 => 'v',
                    _ => unreachable!(),
                };

                let mut next_path = vec![key];
                next_path.extend(&path);
                stack.push((next, next_path));
            }
        }
    }

    paths
}

#[inline]
fn within_bounds(keypad: &Keypad, point: &IVec2) -> bool {
    (0..3).contains(&point.x) && (0..keypad.len() as i32).contains(&point.y)
}

#[inline]
fn get<'a, T>(keypad: &'a [[T; 3]], point: &'a IVec2) -> &'a T {
    &keypad[point.y as usize][point.x as usize]
}

type Cache = HashMap<(Vec<char>, usize), usize>;

fn get_shortest_sequence(
    sequence: &[char],
    depth: usize,
    cache: &mut Cache,
    numeric_keypad: &Keypad,
    arrow_keypad: &Keypad,
    is_door: bool,
    state: &mut [char],
) -> usize {
    let cache_key = (sequence.to_vec(), depth);
    if let Some(&cached) = cache.get(&cache_key) {
        return cached;
    }

    let mut count = 0;

    for &key in sequence {
        // get paths
        let keypad = if is_door {
            numeric_keypad
        } else {
            arrow_keypad
        };
        let paths = get_paths(keypad, state[depth], key);

        if depth == 0 {
            count += paths
                .iter()
                .map(|path| path.len())
                .min()
                .expect("should have a min path");
        } else {
            count += paths
                .iter()
                .map(|sequence| {
                    get_shortest_sequence(
                        sequence,
                        depth - 1,
                        cache,
                        numeric_keypad,
                        arrow_keypad,
                        false,
                        state,
                    )
                })
                .min()
                .expect("should have a min path");
        }
        state[depth] = key;
    }

    cache.insert(cache_key, count);
    count
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
