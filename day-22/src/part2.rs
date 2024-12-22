use itertools::Itertools;

const INPUT: &str = include_str!("input.txt");

#[tracing::instrument(level = "trace", skip())]
pub fn run() -> String {
    process(INPUT, 2000).to_string()
}

#[tracing::instrument(level = "trace", skip(input))]
fn process(input: &str, count: usize) -> usize {
    let indices = 19 * 19 * 19 * 19;
    let mut prices: Vec<u16> = vec![0; indices];
    let mut seen: Vec<u8> = vec![0; indices];
    for seed in input.lines().map(|line| line.parse::<usize>().unwrap()) {
        seen.fill(0);
        for (key, value) in generate_secret(seed)
            // ones digit
            .map(|secret| secret % 10)
            .tuple_windows()
            // calculate price delta - shifted into the positive range
            .map(|(a, b)| (b, 9 + b - a))
            .take(count)
            .tuple_windows()
            // compress price deltas into an index
            .map(|((_, a), (_, b), (_, c), (price, d))| {
                (
                    ((a * 19 * 19 * 19) + (b * 19 * 19) + (c * 19) + d),
                    price as u16,
                )
            })
        {
            if seen[key] == 0 {
                seen[key] = 1;
                prices[key] += value;
            }
        }
    }

    let result = *prices.iter().max().unwrap() as usize;
    result
}

struct SecretIterator {
    seed: usize,
}

impl Iterator for SecretIterator {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        // Calculate the result of multiplying the secret number by 64.
        let result = self.seed << 6;
        // Then, mix this result into the secret number.
        self.seed ^= result;
        // Finally, prune the secret number.
        self.seed &= 0xFF_FFFF;

        // Calculate the result of dividing the secret number by 32. Round the result down to the nearest integer.
        let result = self.seed >> 5;
        // Then, mix this result into the secret number.
        self.seed ^= result;
        // Finally, prune the secret number.
        self.seed &= 0xFF_FFFF;

        // Calculate the result of multiplying the secret number by 2048.
        let result = self.seed << 11;
        // Then, mix this result into the secret number.
        self.seed ^= result;
        // Finally, prune the secret number.
        self.seed &= 0xFF_FFFF;

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

    #[divan::bench()]
    fn bench_process() {
        super::process(INPUT, 2000);
    }
}
