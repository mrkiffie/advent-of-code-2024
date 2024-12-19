use std::collections::HashMap;

const INPUT: &str = include_str!("input.txt");

#[tracing::instrument(level = "trace", skip())]
pub fn run() -> String {
    process(INPUT).to_string()
}

fn is_composable(
    design: &str,
    patterns: &[&str],
    memo: &mut HashMap<String, bool>,
    ignore_self: bool,
) -> bool {
    if design.is_empty() {
        return true;
    }

    if let Some(b) = memo.get(design) {
        return *b;
    }

    memo.insert(design.to_string(), false);

    for pattern in patterns {
        if ignore_self && *pattern == design {
            continue;
        }
        let len = pattern.len().min(design.len());
        let start = &design[0..len];
        let end = &design[len..];
        if start == *pattern && is_composable(end, patterns, memo, ignore_self) {
            memo.insert(design.to_string(), true);
        }
    }

    *memo.get(design).expect("there should be an entry")
}

#[tracing::instrument(level = "trace", skip(input))]
fn process(input: &str) -> usize {
    let mut lines = input.trim().lines();

    let patterns = lines.next().expect("should have valid input");
    let patterns = patterns.split(", ").collect::<Vec<_>>();

    let _ = lines.next();

    let mut memo = HashMap::<String, bool>::new();

    lines
        .map(|design| {
            memo.clear();
            is_composable(design, &patterns, &mut memo, false)
        })
        .filter(|x| *x)
        .count()
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
        assert_eq!(result, 6);
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
