const INPUT: &str = include_str!("input.txt");

#[tracing::instrument(level = "trace", skip())]
pub fn run() -> String {
    process(INPUT).to_string()
}

#[tracing::instrument(level = "trace", skip(input))]
fn process(input: &str) -> usize {
    let (locks, keys) = parse(input);

    let mut count = 0;

    for lock in locks.iter() {
        for key in keys.iter() {
            if lock.iter().zip(key.iter()).all(|(l, k)| l + k < 6) {
                count += 1;
            }
        }
    }

    count
}

type Lock = [u8; 5];
type Key = [u8; 5];

fn parse_item(input: &str) -> [u8; 5] {
    input
        .lines()
        .skip(1)
        .take(5)
        .fold([0; 5], |mut item, line| {
            for i in 0..5 {
                if &line[i..i + 1] == "#" {
                    item[i] += 1;
                }
            }
            item
        })
}

fn parse(input: &str) -> (Vec<Lock>, Vec<Key>) {
    input
        .split("\n\n")
        .fold((Vec::new(), Vec::new()), |(mut locks, mut keys), chunk| {
            if chunk.starts_with("#") {
                locks.push(parse_item(chunk));
            } else {
                keys.push(parse_item(chunk));
            }
            (locks, keys)
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = process(
            "#####
.####
.####
.####
.#.#.
.#...
.....

#####
##.##
.#.##
...##
...#.
...#.
.....

.....
#....
#....
#...#
#.#.#
#.###
#####

.....
.....
#.#..
###..
###.#
###.#
#####

.....
.....
.....
#....
#.#..
#.#.#
#####",
        );
        assert_eq!(result, 3);
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
