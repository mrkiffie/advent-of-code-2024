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
        .filter(|levels| damper(levels))
        .count()
}

#[inline]
fn damper(levels: &[u8]) -> bool {
    (0..levels.len())
        .map(|i| {
            let mut clone = Vec::from(levels);
            clone.remove(i);
            clone
        })
        .any(|levels| validate_levels(&levels))
}

#[inline]
fn validate_levels(levels: &[u8]) -> bool {
    (levels.is_sorted_by(|a, b| a < b) || levels.is_sorted_by(|a, b| a > b))
        && levels.windows(2).all(|window| {
            let diff = window[0].abs_diff(window[1]);
            (1..=3).contains(&diff)
        })
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
        assert_eq!(result, 4);
    }

    #[test]
    fn test_damper() {
        assert!(damper(&[7, 6, 4, 2, 1]));
        assert!(!damper(&[1, 2, 7, 8, 9]));
        assert!(!damper(&[9, 7, 6, 2, 1]));
        assert!(damper(&[1, 3, 2, 4, 5]));
        assert!(damper(&[8, 6, 4, 4, 1]));
        assert!(damper(&[1, 3, 6, 7, 9]));
    }

    #[test]
    fn test_validate_levels() {
        assert!(validate_levels(&[7, 6, 4, 2, 1]));

        assert!(!validate_levels(&[1, 2, 7, 8, 9]));
        assert!(!validate_levels(&[2, 7, 8, 9]));
        assert!(!validate_levels(&[1, 7, 8, 9]));
        assert!(!validate_levels(&[1, 2, 8, 9]));
        assert!(!validate_levels(&[1, 2, 7, 9]));
        assert!(!validate_levels(&[1, 2, 7, 8]));

        assert!(!validate_levels(&[9, 7, 6, 2, 1]));
        assert!(!validate_levels(&[7, 6, 2, 1]));
        assert!(!validate_levels(&[9, 6, 2, 1]));
        assert!(!validate_levels(&[9, 7, 2, 1]));
        assert!(!validate_levels(&[9, 7, 6, 1]));
        assert!(!validate_levels(&[9, 7, 6, 2]));

        assert!(!validate_levels(&[1, 3, 2, 4, 5]));
        assert!(!validate_levels(&[3, 2, 4, 5]));
        assert!(validate_levels(&[1, 2, 4, 5]));
        assert!(validate_levels(&[1, 3, 4, 5]));
        assert!(!validate_levels(&[1, 3, 2, 5]));
        assert!(!validate_levels(&[1, 3, 2, 4]));

        assert!(!validate_levels(&[8, 6, 4, 4, 1]));
        assert!(!validate_levels(&[6, 4, 4, 1]));
        assert!(!validate_levels(&[8, 4, 4, 1]));
        assert!(validate_levels(&[8, 6, 4, 1]));
        assert!(!validate_levels(&[8, 6, 4, 4]));

        assert!(validate_levels(&[1, 3, 6, 7, 9]));
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
