const INPUT: &str = include_str!("input.txt");

#[tracing::instrument(level = "trace", skip())]
pub fn run() -> usize {
    process(INPUT) as usize
}

#[tracing::instrument(level = "trace", skip(input))]
fn process(input: &str) -> u32 {
    let (ordering, pages) = input.split_once("\n\n").unwrap();

    let lookup: [u128; 100] = ordering
        .lines()
        .map(|line| {
            line.split_once('|').map(|(a, b)| {
                (
                    a.parse::<u8>().expect("should be valid"),
                    b.parse::<u8>().expect("should be valid"),
                )
            })
        })
        .fold([0; 100], |mut accumulator, pair| {
            if let Some((key, value)) = pair {
                accumulator[key as usize] += 1 << value;
            }
            accumulator
        });

    let result = pages
        .lines()
        .filter_map(|line| {
            let pages: Vec<u8> = line
                .split(',')
                .map(|page| page.parse::<u8>().expect("should be valid"))
                .collect();

            let is_sorted = pages.is_sorted_by(|key, b| lookup[*key as usize] & (1 << b) != 0);

            if is_sorted {
                return None;
            }

            let mut pages = pages;

            pages.sort_by(|a, b| {
                let value = lookup[*a as usize];
                match value & (1 << b) != 0 {
                    true => std::cmp::Ordering::Less,
                    false => std::cmp::Ordering::Greater,
                }
            });

            // find middle pages.
            let len = pages.len();
            let index = len / 2;
            let middle_page = pages.get(index).expect("index exists");
            Some(*middle_page as u32)
        })
        .sum();

    result
}

#[cfg(test)]
mod tests {
    use super::*;

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
