use itertools::Itertools;
use std::collections::HashSet;

const INPUT: &str = include_str!("input.txt");

#[tracing::instrument(level = "trace", skip())]
pub fn run() -> String {
    process(INPUT).to_string()
}

// aa-zz is unique pairs 676
#[tracing::instrument(level = "trace", skip(input))]
fn process(input: &str) -> usize {
    let mut connections: Vec<Vec<u8>> = vec![vec![0; 676]; 676];
    let mut ts = vec![0; 676];
    for (a, b) in input.lines().map(|line| line.split_once('-').unwrap()) {
        let a_index = to_index(a);
        let b_index = to_index(b);
        connections[a_index][b_index] = 1;
        connections[b_index][a_index] = 1;
        if a.starts_with('t') {
            ts[a_index] = 1;
        }
        if b.starts_with('t') {
            ts[b_index] = 1;
        }
    }

    let triples = ts
        .neighbours()
        .flat_map(|t| {
            connections[t]
                .neighbours()
                .tuple_combinations()
                .filter(|(a, b)| connections[*a][*b] == 1)
                .map(move |(a, b)| {
                    let mut triple = [a, b, t];
                    triple.sort();
                    (triple[0], triple[1], triple[2])
                })
        })
        .collect::<HashSet<_>>();

    triples.len()
}

trait Neighbours {
    fn neighbours(&self) -> NeighboursIterator;
}

impl Neighbours for [u8] {
    fn neighbours(&self) -> NeighboursIterator {
        NeighboursIterator { list: self, i: 0 }
    }
}

#[derive(Clone)]
struct NeighboursIterator<'a> {
    list: &'a [u8],
    i: usize,
}

impl Iterator for NeighboursIterator<'_> {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        while self.i < self.list.len() {
            if self.list[self.i] == 1 {
                let i = self.i;
                self.i += 1;
                return Some(i);
            }
            self.i += 1;
        }
        None
    }
}

fn to_index(code: &str) -> usize {
    let code = code.as_bytes();
    (code[0] - b'a') as usize * 26 + (code[1] - b'a') as usize
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = process(
            "kh-tc
qp-kh
de-cg
ka-co
yn-aq
qp-ub
cg-tb
vc-aq
tb-ka
wh-tc
yn-cg
kh-ub
ta-co
de-co
tc-td
tb-wq
wh-td
ta-ka
td-qp
aq-cg
wq-ub
ub-vc
de-ta
wq-aq
wq-vc
wh-yn
ka-de
kh-ta
co-tc
wh-qp
tb-vc
td-yn",
        );
        assert_eq!(result, 7);
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
