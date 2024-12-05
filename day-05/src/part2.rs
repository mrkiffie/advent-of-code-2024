use std::collections::HashMap;

const INPUT: &str = include_str!("input.txt");

#[tracing::instrument(level = "trace", skip())]
pub fn run() -> String {
    process(INPUT).to_string()
}

#[tracing::instrument(level = "trace", skip(input))]
fn process(input: &str) -> u32 {
    let (lookup, mut pages) = parse(input).unwrap_or_default();

    let result = pages
        .iter_mut()
        .filter(|pages| {
            !pages.is_sorted_by(|key, b| {
                let value = lookup.get(key).unwrap_or(&0);
                value & (1 << b) != 0
            })
        })
        .map(|pages| {
            pages.sort_by(|a, b| {
                let value = lookup.get(a).unwrap_or(&0);
                match value & (1 << b) != 0 {
                    true => std::cmp::Ordering::Less,
                    false => std::cmp::Ordering::Greater,
                }
            });
            pages
        })
        .map(|pages| {
            // find middle pages.
            let len = pages.len();
            let index = len / 2;
            let middle_page = pages.get(index).expect("index exists");
            *middle_page as u32
        })
        .sum();

    result
}

type OrderingLookup = HashMap<u8, u128>;
type Pages = Vec<Vec<u8>>;

fn parse(input: &str) -> Option<(OrderingLookup, Pages)> {
    let (ordering, pages) = input.split_once("\n\n")?;

    let ordering_lookup: HashMap<u8, u128> = ordering
        .lines()
        .map(|line| {
            line.split_once('|').map(|(a, b)| {
                (
                    a.parse::<u8>().expect("should be valid"),
                    b.parse::<u8>().expect("should be valid"),
                )
            })
        })
        .fold(HashMap::with_capacity(64), |mut accumulator, pair| {
            if let Some((key, value)) = pair {
                accumulator
                    .entry(key)
                    .and_modify(|e| *e += 1 << value)
                    .or_insert(1 << value);
            }

            accumulator
        });

    let pages: Pages = pages
        .lines()
        .map(|line| {
            line.split(',')
                .map(|page| page.parse::<u8>().expect("should be valid"))
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    Some((ordering_lookup, pages))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        parse(INPUT).expect("Something");
    }

    #[test]
    fn it_works() {
        let result = process(
            "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47",
        );
        assert_eq!(result, 123);
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
