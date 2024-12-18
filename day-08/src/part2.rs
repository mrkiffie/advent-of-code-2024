use glam::IVec2;
use itertools::Itertools;
use std::collections::{HashMap, HashSet};

const INPUT: &str = include_str!("input.txt");

#[tracing::instrument(level = "trace", skip())]
pub fn run() -> String {
    process(INPUT).to_string()
}

#[tracing::instrument(level = "trace", skip(input))]
fn process(input: &str) -> usize {
    let (dimensions, map) = parse(input);

    let mut set: HashSet<IVec2> = HashSet::new();

    for (_frequency, towers) in map {
        towers.iter().permutations(2).for_each(|towers| {
            let mut iter = towers.iter();
            if let (Some(&a), Some(&b)) = (iter.next(), iter.next()) {
                set.insert(*b);
                let offset = a - b;
                let mut antinode = b - offset;
                while (0..=dimensions.x).contains(&antinode.x)
                    && (0..=dimensions.y).contains(&antinode.y)
                {
                    set.insert(antinode);

                    antinode -= offset;
                }
            }
        });
    }
    set.len()
}

#[inline]
fn parse(input: &str) -> (IVec2, HashMap<char, Vec<IVec2>>) {
    let mut hashmap: HashMap<char, Vec<IVec2>> = HashMap::new();
    let mut cols = 0;
    let mut rows = 0;
    input.lines().enumerate().for_each(|(y, line)| {
        rows = y;
        line.chars().enumerate().for_each(|(x, c)| {
            cols = x;
            if c != '.' {
                let tower = IVec2::new(x as i32, y as i32);
                hashmap
                    .entry(c)
                    .and_modify(|entry| entry.push(tower))
                    .or_insert(vec![tower]);
            }
        })
    });
    (IVec2::new(cols as i32, rows as i32), hashmap)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = process(
            "............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............",
        );
        assert_eq!(result, 34);
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
