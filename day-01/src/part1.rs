const INPUT: &str = include_str!("input.txt");

#[tracing::instrument(level = "trace", skip())]
pub fn run() -> String {
    process(INPUT).to_string()
}

#[tracing::instrument(level = "trace", skip(input))]
fn process(input: &str) -> u64 {
    let mut left: Vec<u64> = Vec::with_capacity(1000);
    let mut right: Vec<u64> = Vec::with_capacity(1000);
    for line in input.lines() {
        let mut split = line.split("   ");
        if let Some(val) = split.next() {
            left.push(val.parse::<u64>().expect("Should be valid number"))
        }
        if let Some(val) = split.next() {
            right.push(val.parse::<u64>().expect("Should be valid number"))
        }
    }

    left.sort();
    right.sort();

    let result: u64 = left
        .iter()
        .zip(right)
        .map(|(a, b)| if a > &b { a - b } else { b - a })
        .sum();

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = process("3   4\n4   3\n2   5\n1   3\n3   9\n3   3");
        assert_eq!(result, 11);
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
