use std::collections::HashSet;

const INPUT: &str = include_str!("input.txt");

#[tracing::instrument(level = "trace", skip())]
pub fn run() -> String {
    process(INPUT).to_string()
}

#[tracing::instrument(level = "trace", skip(input))]
fn process(input: &str) -> String {
    let (_wires, gates) = input.split_once("\n\n").unwrap();

    let mut problematic = HashSet::with_capacity(8);
    let mut final_carry = "z00";

    let gates = gates
        .lines()
        .fold(Vec::with_capacity(222), |mut gates, line| {
            let mut it = line.split_ascii_whitespace();
            match (it.next(), it.next(), it.next(), it.next(), it.next()) {
                (Some(a), Some(gate), Some(b), Some(_), Some(c)) => {
                    final_carry = final_carry.max(c);
                    if a > b {
                        gates.push((b, gate, a, c));
                    } else {
                        gates.push((a, gate, b, c));
                    }
                }
                _ => unreachable!(),
            }
            gates
        });

    for (a, gate, b, c) in &gates {
        let az = a.starts_with("z");
        let bz = b.starts_with("z");
        let cz = c.starts_with("z");
        let ax = a.starts_with("x");
        let bx = b.starts_with("x");
        let cx = c.starts_with("x");
        let ay = a.starts_with("y");
        let by = b.starts_with("y");
        let cy = c.starts_with("y");

        // Ensure that z__ bits are preceeded by AND or OR - except the final carry bit.
        if cz && gate != &"XOR" && c != &final_carry {
            problematic.insert(c);
        }

        // Ensure all XORs have at least either an x__ bit or y__ bit as input or z__ bit as output.
        if gate == &"XOR" && !az && !bz && !cz && !ax && !bx && !cx && !ay && !by && !cy {
            problematic.insert(c);
        }

        // Ensure that the gate following an AND is an OR - expect for input bits _00
        if gate == &"AND" && ![a, b].contains(&&"x00") {
            for (sa, sgate, sb, _) in &gates {
                if [sa, sb].contains(&c) && sgate != &"OR" {
                    problematic.insert(c);
                }
            }
        }

        // Ensure that XOR cannot be followed by OR
        if gate == &"XOR" {
            for (sa, sgate, sb, _) in &gates {
                if [sa, sb].contains(&c) && sgate == &"OR" {
                    problematic.insert(c);
                }
            }
        }
    }

    let mut problematic = problematic.into_iter().cloned().collect::<Vec<_>>();

    problematic.sort();
    problematic.join(",")
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
