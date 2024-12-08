use itertools::Itertools;
use rayon::prelude::*;

const INPUT: &str = include_str!("input.txt");

#[tracing::instrument(level = "trace", skip())]
pub fn run() -> String {
    process(INPUT).to_string()
}

#[derive(Debug, Clone)]
enum Operator {
    Add,
    Mul,
    Concatenate,
}

pub trait Concatenate<Rhs = Self> {
    type Output;

    #[must_use = "this returns the result of the operation, without modifying the original"]
    fn concatenate(self, rhs: Rhs) -> Self::Output;
}

impl Concatenate<u64> for u64 {
    type Output = u64;

    #[inline]
    fn concatenate(self, rhs: u64) -> Self::Output {
        self * 10_i32.pow(rhs.ilog10() + 1) as u64 + rhs
    }
}

const OPERATORS: [Operator; 3] = [Operator::Add, Operator::Mul, Operator::Concatenate];

#[tracing::instrument(level = "trace", skip(input))]
pub fn process(input: &str) -> u64 {
    let total = parse(input)
        .par_bridge()
        .map(|(total, operands)| {
            (0..operands.len() - 1)
                .map(|_| OPERATORS)
                .multi_cartesian_product()
                .any(|operator_sequence| {
                    let mut operators = operator_sequence.iter();
                    let computed_total = operands
                        .iter()
                        .copied()
                        .reduce(|total, operand| match operators.next() {
                            Some(Operator::Add) => total + operand,
                            Some(Operator::Mul) => total * operand,
                            Some(Operator::Concatenate) => total.concatenate(operand),
                            _ => unreachable!("there should be an operator"),
                        })
                        .unwrap_or(0);

                    computed_total == total
                })
                .then_some(total)
                .unwrap_or_default()
        })
        .sum();

    total
}

#[inline]
fn parse(input: &str) -> impl Iterator<Item = (u64, std::vec::Vec<u64>)> + use<'_> {
    input.lines().filter_map(|line| {
        line.split_once(": ").map(|(total, operands)| {
            let total = total.parse::<u64>().expect("there should be a total");

            let operands = operands
                .split(' ')
                .map(|operand| operand.parse::<u64>().expect("operand should be valid"))
                .collect::<Vec<u64>>();
            (total, operands)
        })
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_concatenate() {
        assert_eq!(12_u64.concatenate(345), 12345);
    }

    #[test]
    fn it_works() {
        let result = process(
            "190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20",
        );
        assert_eq!(result, 11387);
    }
    #[test]
    fn edge_case_1() {
        assert_eq!(process("190: 10 19"), 190);
        assert_eq!(process("3267: 81 40 27"), 3267);
        assert_eq!(process("83: 17 5"), 0);
        assert_eq!(process("156: 15 6"), 156);
        assert_eq!(process("7290: 6 8 6 15"), 7290);
        assert_eq!(process("161011: 16 10 13"), 0);
        assert_eq!(process("192: 17 8 14"), 192);
        assert_eq!(process("21037: 9 7 18 13"), 0);
        assert_eq!(process("292: 11 6 16 20"), 292);
        assert_eq!(process("111111: 1 1 1 1 1 1"), 111111);
    }
}

#[cfg(feature = "bench")]
pub mod benchmarks {
    use super::INPUT;

    pub fn main() {
        divan::main();
    }

    #[divan::bench(sample_count = 100)]
    fn bench_process() {
        super::process(INPUT);
    }
}
