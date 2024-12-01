use std::cmp::Ordering;

#[tracing::instrument(level = "trace", skip())]
pub fn run() -> String {
    let input = include_str!("input.txt");
    process(input).to_string()
}

#[tracing::instrument(level = "trace", skip(input))]
fn process(input: &str) -> u64 {
    let mut left: Vec<u64> = Vec::with_capacity(1000);
    let mut right: Vec<u64> = Vec::with_capacity(1000);
    for line in input.lines() {
        let mut split = line.split("   ");
        if let Some(val) = split.next() {
            left.push(val.parse::<u64>().expect("Should be valid number"))
        }
        if let Some(val) = split.next() {
            right.push(val.parse::<u64>().expect("Should be valid number"))
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
                let increment = count as u64 * value;
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
        let result = process("3   4\n4   3\n2   5\n1   3\n3   9\n3   3");
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
        let result = process("1   0\n2   2\n2   2\n4   4\n9   3\n3   3");
        assert_eq!(result, 18);
    }
}
