use crate::trie::Trie;

const INPUT: &str = include_str!("input.txt");

#[tracing::instrument(level = "trace", skip())]
pub fn run() -> String {
    process(INPUT).to_string()
}

fn dfs(design: &str, patterns: &Trie, cache: &mut [usize]) -> usize {
    if design.is_empty() {
        return 1;
    }

    let c = cache[design.len() - 1];
    if c != usize::MAX {
        return c;
    }

    let mut count = 0;
    for i in patterns.common_prefix_lengths(design.as_bytes()) {
        count += dfs(&design[i..], patterns, cache);
    }

    cache[design.len() - 1] = count;
    count
}

#[tracing::instrument(level = "trace", skip(input))]
fn process(input: &str) -> usize {
    let mut lines = input.trim().lines();

    let patterns = lines.next().expect("should have valid input");
    let patterns = patterns.split(", ").collect::<Vec<_>>();
    let _ = lines.next();

    let mut trie = Trie::new();
    for p in &patterns {
        trie.insert(p);
    }

    lines
        .map(|design| {
            let mut cache = vec![usize::MAX; design.len()];
            dfs(design, &trie, &mut cache)
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = process(
            "r, wr, b, g, bwu, rb, gb, br

brwrr
bggr
gbbr
rrbgbr
ubwu
bwurrg
brgr
bbrgwb",
        );
        assert_eq!(result, 16);
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
