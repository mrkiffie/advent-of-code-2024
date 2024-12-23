use itertools::Itertools;

const INPUT: &str = include_str!("input.txt");

#[tracing::instrument(level = "trace", skip())]
pub fn run() -> String {
    process(INPUT)
}

// aa-zz is unique pairs 676
#[tracing::instrument(level = "trace", skip(input))]
fn process(input: &str) -> String {
    let mut connections: Vec<Vec<u8>> = vec![vec![0; 676]; 676];
    let mut nodes = vec![0; 676];
    for (a, b) in input.lines().map(|line| line.split_once('-').unwrap()) {
        let a = to_index(a);
        let b = to_index(b);
        connections[a][b] = 1;
        connections[b][a] = 1;
        nodes[a] = 1;
        nodes[b] = 1;
    }

    let lan_party = nodes
        .neighbours()
        .flat_map(|i| {
            connections[i]
                .neighbours()
                // each node has 13 neighbours
                .combinations(12)
                .filter(|neighbours_subset| {
                    neighbours_subset
                        .iter()
                        .tuple_combinations()
                        .all(|(a, b)| connections[*a][*b] == 1)
                })
                .map(move |mut subset| {
                    subset.push(i);
                    subset.sort();
                    subset
                })
        })
        .unique()
        .max_by_key(|a| a.len())
        .expect("there should be a lan party")
        .iter()
        .map(to_code)
        .join(",");

    lan_party
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

fn to_code(index: &usize) -> String {
    let a = b'a' + (index / 26) as u8;
    let b = b'a' + (index % 26) as u8;
    String::from_utf8(vec![a, b]).expect("invalid node index")
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
        assert_eq!(result, "co,de,ka,ta");
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
