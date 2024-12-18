const INPUT: &str = include_str!("input.txt");

#[tracing::instrument(level = "trace", skip())]
pub fn run() -> String {
    process(INPUT).to_string()
}

#[tracing::instrument(level = "trace", skip(input))]
fn process(input: &str) -> usize {
    input
        .lines()
        .map(parse_levels)
        .filter(|report| report.is_sorted_by(|a, b| a < b) || report.is_sorted_by(|a, b| a > b))
        .filter(|report| {
            report.windows(2).all(|window| {
                let diff = window[0].abs_diff(window[1]);
                (1..=3).contains(&diff)
            })
        })
        .count()
}

#[inline]
fn parse_levels(levels: &str) -> Vec<u8> {
    levels
        .split_ascii_whitespace()
        .map(|n| n.parse::<u8>().expect("should be valid input"))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let result = process("7 6 4 2 1\n1 2 7 8 9\n9 7 6 2 1\n1 3 2 4 5\n8 6 4 4 1\n1 3 6 7 9");
        assert_eq!(result, 2);
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
