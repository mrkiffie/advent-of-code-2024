use std::collections::{HashMap, VecDeque};

const INPUT: &str = include_str!("input.txt");

#[tracing::instrument(level = "trace", skip())]
pub fn run() -> String {
    process(INPUT).to_string()
}

#[tracing::instrument(level = "trace", skip(input))]
fn process(input: &str) -> usize {
    let (wires, gates) = input.split_once("\n\n").unwrap();
    let mut wires = wires
        .lines()
        .map(|line| line.split_once(": ").unwrap())
        .map(|(key, value)| (key, value.parse::<u8>().unwrap()))
        .collect::<HashMap<&str, u8>>();

    let mut queue = gates
        .lines()
        .map(|line| {
            let mut it = line.split_ascii_whitespace();
            match (it.next(), it.next(), it.next(), it.next(), it.next()) {
                (Some(a), Some(gate), Some(b), Some(_), Some(c)) => {
                    let gate = match gate {
                        "AND" => Gate::And,
                        "OR" => Gate::Or,
                        "XOR" => Gate::Xor,
                        _ => unreachable!(),
                    };

                    (a, gate, b, c)
                }
                _ => unreachable!(),
            }
        })
        .collect::<VecDeque<_>>();

    let mut number = 0;

    while let Some((a, gate, b, c)) = queue.pop_front() {
        match (wires.get(a), wires.get(b)) {
            (Some(a), Some(b)) => {
                let value = match gate {
                    Gate::And => a & b,
                    Gate::Or => a | b,
                    Gate::Xor => a ^ b,
                };
                if let Some(shift) = c.strip_prefix("z") {
                    let shift = shift.parse::<usize>().unwrap();
                    number |= (value as usize) << shift
                } else {
                    wires.insert(c, value);
                }
            }
            _ => {
                queue.push_back((a, gate, b, c));
            }
        }
    }
    number
}

enum Gate {
    And,
    Or,
    Xor,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_test() {
        let result = process(
            "x00: 1
x01: 1
x02: 1
y00: 0
y01: 1
y02: 0

x00 AND y00 -> z00
x01 XOR y01 -> z01
x02 OR y02 -> z02",
        );
        assert_eq!(result, 4);
    }

    #[test]
    fn it_works() {
        let result = process(
            "x00: 1
x01: 0
x02: 1
x03: 1
x04: 0
y00: 1
y01: 1
y02: 1
y03: 1
y04: 1

ntg XOR fgs -> mjb
y02 OR x01 -> tnw
kwq OR kpj -> z05
x00 OR x03 -> fst
tgd XOR rvg -> z01
vdt OR tnw -> bfw
bfw AND frj -> z10
ffh OR nrd -> bqk
y00 AND y03 -> djm
y03 OR y00 -> psh
bqk OR frj -> z08
tnw OR fst -> frj
gnj AND tgd -> z11
bfw XOR mjb -> z00
x03 OR x00 -> vdt
gnj AND wpb -> z02
x04 AND y00 -> kjc
djm OR pbm -> qhw
nrd AND vdt -> hwm
kjc AND fst -> rvg
y04 OR y02 -> fgs
y01 AND x02 -> pbm
ntg OR kjc -> kwq
psh XOR fgs -> tgd
qhw XOR tgd -> z09
pbm OR djm -> kpj
x03 XOR y03 -> ffh
x00 XOR y04 -> ntg
bfw OR bqk -> z06
nrd XOR fgs -> wpb
frj XOR qhw -> z04
bqk OR frj -> z07
y03 OR x01 -> nrd
hwm AND bqk -> z03
tgd XOR rvg -> z12
tnw OR pbm -> gnj",
        );
        assert_eq!(result, 2024);
    }
}

#[cfg(feature = "bench")]
pub mod benchmarks {
    use super::INPUT;

    pub fn main() {
        divan::main();
    }

    #[divan::bench(sample_count = 1000)]
    fn bench_process() {
        super::process(INPUT);
    }
}
