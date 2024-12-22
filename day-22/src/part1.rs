const INPUT: &str = include_str!("input.txt");

#[tracing::instrument(level = "trace", skip())]
pub fn run() -> String {
    process(INPUT).to_string()
}

#[tracing::instrument(level = "trace", skip(input))]
fn process(input: &str) -> usize {
    input
        .lines()
        .filter_map(|line| line.parse::<usize>().ok())
        .filter_map(|seed| generate_secret(seed).nth(1999))
        .sum()
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
        //Finally, prune the secret number.
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
    fn basic_iteraton() {
        let mut it = generate_secret(123);
        assert_eq!(it.next(), Some(15887950));
        assert_eq!(it.next(), Some(16495136));
        assert_eq!(it.next(), Some(527345));
        assert_eq!(it.next(), Some(704524));
        assert_eq!(it.next(), Some(1553684));
        assert_eq!(it.next(), Some(12683156));
        assert_eq!(it.next(), Some(11100544));
        assert_eq!(it.next(), Some(12249484));
        assert_eq!(it.next(), Some(7753432));
        assert_eq!(it.next(), Some(5908254));
    }

    #[test]
    fn case_1() {
        let result = generate_secret(1).nth(1999);
        assert_eq!(result, Some(8685429));
    }
    #[test]
    fn case_10() {
        let result = generate_secret(10).nth(1999);
        assert_eq!(result, Some(4700978));
    }
    #[test]
    fn case_100() {
        let result = generate_secret(100).nth(1999);
        assert_eq!(result, Some(15273692));
    }
    #[test]
    fn case_2024() {
        let result = generate_secret(2024).nth(1999);
        assert_eq!(result, Some(8667524));
    }

    #[test]
    fn it_works() {
        let result = process(
            "1
10
100
2024",
        );
        assert_eq!(result, 37327623);
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
