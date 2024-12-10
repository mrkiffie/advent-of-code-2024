use std::{cmp::Ordering, collections::HashMap};

const INPUT: &str = include_str!("input.txt");

#[tracing::instrument(level = "trace", skip())]
pub fn run() -> usize {
    sort_and_indices(INPUT) as usize
    // hashmap_counting(INPUT) as usize
    // filter_iteration(INPUT) as usize
}

#[tracing::instrument(level = "trace", skip(input))]
fn sort_and_indices(input: &str) -> u32 {
    let mut left: Vec<u32> = Vec::with_capacity(1000);
    let mut right: Vec<u32> = Vec::with_capacity(1000);
    for line in input.lines() {
        let mut split = line.split("   ");
        if let Some(val) = split.next() {
            left.push(val.parse::<u32>().expect("Should be valid number"))
        }
        if let Some(val) = split.next() {
            right.push(val.parse::<u32>().expect("Should be valid number"))
        }
    }

    left.sort();
    right.sort();

    let mut left_index: usize = 0;
    let mut right_index: usize = 0;
    let mut result = 0;
    let len = left.len();

    while left_index < len && right_index < len {
        match left[left_index].cmp(&right[right_index]) {
            Ordering::Less => {
                left_index += 1;
            }
            Ordering::Equal => {
                let mut count = 1;
                while right_index < len - 1 && right[right_index] == right[right_index + 1] {
                    count += 1;
                    right_index += 1;
                }
                let value = right[right_index];
                let increment = count as u32 * value;
                result += increment;

                while left_index < len - 1 && left[left_index] == left[left_index + 1] {
                    result += increment;
                    left_index += 1;
                }
                left_index += 1;
                right_index += 1;
            }
            Ordering::Greater => {
                right_index += 1;
            }
        }
    }

    result
}

#[tracing::instrument(level = "trace", skip(input))]
fn hashmap_counting(input: &str) -> u32 {
    let mut left: Vec<u32> = Vec::with_capacity(1000);
    let mut right: HashMap<u32, usize> = HashMap::new();
    for line in input.lines() {
        let mut split = line.split("   ");
        if let Some(val) = split.next() {
            left.push(val.parse::<u32>().expect("Should be valid number"))
        }
        if let Some(val) = split.next() {
            let val = val.parse::<u32>().expect("Should be valid number");
            right.entry(val).and_modify(|x| *x += 1).or_insert(1);
        }
    }

    let result = left
        .iter()
        .filter_map(|x| right.get(x).map(|c| *c as u32 * *x))
        .sum();

    result
}

#[tracing::instrument(level = "trace", skip(input))]
fn filter_iteration(input: &str) -> u32 {
    let mut left: Vec<u32> = Vec::with_capacity(1000);
    let mut right: Vec<u32> = Vec::with_capacity(1000);
    for line in input.lines() {
        let mut split = line.split("   ");
        if let Some(val) = split.next() {
            left.push(val.parse::<u32>().expect("Should be valid number"))
        }
        if let Some(val) = split.next() {
            right.push(val.parse::<u32>().expect("Should be valid number"))
        }
    }

    let result = left
        .iter()
        .map(|l| l * right.iter().filter(|r| r == &l).count() as u32)
        .sum();

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    // 1   3
    // 2   3
    // 3   3
    // 3   4
    // 3   5
    // 4   9
    #[test]
    fn example_1() {
        let result = sort_and_indices("3   4\n4   3\n2   5\n1   3\n3   9\n3   3");
        assert_eq!(result, 31);
    }

    // 1   0
    // 2   2
    // 2   2
    // 4   4
    // 9   3
    // 3   3
    #[test]
    fn example_2() {
        let result = sort_and_indices("1   0\n2   2\n2   2\n4   4\n9   3\n3   3");
        assert_eq!(result, 18);
    }
}

#[cfg(feature = "bench")]
pub mod benchmarks {
    use super::INPUT;

    pub fn main() {
        divan::main();
    }

    #[divan::bench(sample_count = 1000)]
    fn sort_and_indices() {
        super::sort_and_indices(INPUT);
    }

    #[divan::bench(sample_count = 1000)]
    fn hashmap_counting() {
        super::hashmap_counting(INPUT);
    }

    #[divan::bench(sample_count = 1000)]
    fn filter_iteration() {
        super::filter_iteration(INPUT);
    }
}
