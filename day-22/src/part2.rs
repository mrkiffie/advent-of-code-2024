use itertools::Itertools;
use std::collections::HashMap;

const INPUT: &str = include_str!("input.txt");

#[tracing::instrument(level = "trace", skip())]
pub fn run() -> String {
    process(INPUT, 2000).to_string()
}

#[tracing::instrument(level = "trace", skip(input))]
fn process(input: &str, count: usize) -> usize {
    *input
        .lines()
        .map(|line| line.parse::<usize>().unwrap())
        .map(|seed| {
            generate_secret(seed)
                .map(|secret| secret % 10)
                .tuple_windows()
                .map(|(a, b)| (b, 10 + b - a))
                .take(count)
                .tuple_windows()
                .fold(HashMap::new(), |mut map, (a, b, c, d)| {
                    let price = d.0;
                    let key =
                        (((a.1 + 10) << 24) | ((b.1 + 10) << 16) | ((c.1 + 10) << 8) | (d.1 + 10))
                            as u32;

                    #[cfg(test)]
                    println!("{:?}: {}", key, price);

                    map.entry(key).or_insert(price);
                    map
                })
        })
        .fold(HashMap::new(), |mut map, prices| {
            for (key, value) in prices {
                map.entry(key)
                    .and_modify(|previous| *previous += value)
                    .or_insert(value);
            }
            map
        })
        .values()
        .max()
        .unwrap()
}

struct SecretIterator {
    seed: usize,
}

impl Iterator for SecretIterator {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        // Calculate the result of multiplying the secret number by 64.
        let result = self.seed * 64;
        // Then, mix this result into the secret number.
        self.seed ^= result;
        // Finally, prune the secret number.
        self.seed %= 16777216;

        // Calculate the result of dividing the secret number by 32. Round the result down to the nearest integer.
        let result = self.seed / 32;
        // Then, mix this result into the secret number.
        self.seed ^= result;
        // Finally, prune the secret number.
        self.seed %= 16777216;

        // Calculate the result of multiplying the secret number by 2048.
        let result = self.seed * 2048;
        // Then, mix this result into the secret number.
        self.seed ^= result;
        // Finally, prune the secret number.
        self.seed %= 16777216;

        Some(self.seed)
    }
}

fn generate_secret(seed: usize) -> SecretIterator {
    SecretIterator { seed }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn monkey_business() {
        assert_eq!(process("123", 10), 6);
    }

    #[test]
    fn it_works() {
        let result = process(
            "1
2
3
2024",
            2000,
        );
        assert_eq!(result, 23);
    }
}

#[cfg(feature = "bench")]
pub mod benchmarks {
    use super::INPUT;

    pub fn main() {
        divan::main();
    }

    #[divan::bench(sample_count = 10)]
    fn bench_process() {
        super::process(INPUT, 2000);
    }
}
