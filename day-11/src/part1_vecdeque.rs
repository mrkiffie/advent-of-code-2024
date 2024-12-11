use std::collections::VecDeque;

const INPUT: &str = include_str!("input.txt");

#[tracing::instrument(level = "trace", skip())]
pub fn run() -> usize {
    process(INPUT, 25)
}

type BlinkItem = (u64, usize);

#[tracing::instrument(level = "trace", skip(input))]
fn process(input: &str, blinks: usize) -> usize {
    let pebbles = input.trim().split_ascii_whitespace();

    let mut queue = pebbles
        .flat_map(|pebble| pebble.parse::<u64>())
        .map(|pebble| (pebble, blinks))
        .collect::<VecDeque<BlinkItem>>();

    let mut count = 0;

    while let Some((pebble, remaining)) = queue.pop_front() {
        if remaining == 0 {
            count += 1;
        } else {
            match pebble {
                // If the stone is engraved with the number 0, it is replaced by a stone engraved with the number 1.
                0 => {
                    queue.push_front((1, remaining - 1));
                }
                // If the stone is engraved with a number that has an even number of digits, it is replaced by two stones. The left half of the digits are engraved on the new left stone, and the right half of the digits are engraved on the new right stone. (The new numbers don't keep extra leading zeroes: 1000 would become stones 10 and 0.)
                x if x.ilog10() % 2 == 1 => {
                    let (a, b) = split_digits(x);
                    queue.push_front((a, remaining - 1));
                    queue.push_front((b, remaining - 1));
                }
                // If none of the other rules apply, the stone is replaced by a new stone; the old stone's number multiplied by 2024 is engraved on the new stone.
                x => {
                    queue.push_front((x * 2024, remaining - 1));
                }
            }
        }
    }

    count
}

fn split_digits(value: u64) -> (u64, u64) {
    let order = 10_u64.pow(value.ilog10() / 2 + 1);
    (value / order, value % order)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_split_digits() {
        assert_eq!(split_digits(123456), (123, 456));
        assert_eq!(split_digits(1234), (12, 34));
        assert_eq!(split_digits(12), (1, 2));
        assert_eq!(split_digits(1001), (10, 1));
    }

    #[test]
    fn example_1() {
        let result = process("125 17", 6);
        assert_eq!(result, 22);
    }

    #[test]
    fn example_2() {
        let result = process("125 17", 25);
        assert_eq!(result, 55312);
    }
}

#[cfg(feature = "bench")]
pub mod benchmarks {
    use super::INPUT;

    pub fn main() {
        divan::main();
    }

    #[divan::bench()]
    fn bench_process_25() {
        super::process(INPUT, 25);
    }
}
