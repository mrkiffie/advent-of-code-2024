use itertools::Itertools;
use rayon::prelude::*;

const INPUT: &str = include_str!("input.txt");

#[tracing::instrument(level = "trace", skip())]
pub fn run() -> usize {
    process(INPUT) as usize
}

#[derive(Debug, Clone)]
enum Operator {
    Add,
    Mul,
}

const OPERATORS: [Operator; 2] = [Operator::Add, Operator::Mul];
#[tracing::instrument(level = "trace", skip(input))]
pub fn process(input: &str) -> u64 {
    let total = parse(input)
        .par_bridge()
        .map(|(total, operands)| {
            let operator_count = operands.len() - 1;

            (0..operator_count)
                .map(|_| OPERATORS)
                .multi_cartesian_product()
                .any(|operator_sequence| {
                    let mut operators = operator_sequence.iter();
                    let computed_total: u64 = operands
                        .iter()
                        .copied()
                        .reduce(|total, operand| match operators.next() {
                            Some(Operator::Add) => total + operand,
                            Some(Operator::Mul) => total * operand,
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
        assert_eq!(result, 3749);
    }

    #[test]
    fn edge_case_1() {
        // 1 + 1 + 1 + 1
        assert_eq!(process("4: 1 1 1 1"), 4);
        // 1 + 1 + 1 * 1
        assert_eq!(process("3: 1 1 1 1"), 3);
        // 1 + 1 * 1 + 1
        assert_eq!(process("3: 1 1 1 1"), 3);
        // 1 + 1 * 1 * 1
        assert_eq!(process("2: 1 1 1 1"), 2);
        // 1 * 1 + 1 + 1
        assert_eq!(process("3: 1 1 1 1"), 3);
        // 1 * 1 + 1 * 1
        assert_eq!(process("2: 1 1 1 1"), 2);
        // 1 * 1 * 1 + 1
        assert_eq!(process("2: 1 1 1 1"), 2);
        // 1 * 1 * 1 * 1
        assert_eq!(process("1: 1 1 1 1"), 1);
    }

    #[test]
    fn edge_case_2() {
        // 5 + 5 + 5 + 5
        assert_eq!(process("20: 5 5 5 5"), 20);
        // 5 + 5 + 5 * 5
        assert_eq!(process("75: 5 5 5 5"), 75);
        // 5 + 5 * 5 + 5
        assert_eq!(process("55: 5 5 5 5"), 55);
        // 5 + 5 * 5 * 5
        assert_eq!(process("250: 5 5 5 5"), 250);
        // 5 * 5 + 5 + 5
        assert_eq!(process("35: 5 5 5 5"), 35);
        // 5 * 5 + 5 * 5
        assert_eq!(process("150: 5 5 5 5"), 150);
        // 5 * 5 * 5 + 5
        assert_eq!(process("130: 5 5 5 5"), 130);
        // 5 * 5 * 5 * 5
        assert_eq!(process("625: 5 5 5 5"), 625);
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
