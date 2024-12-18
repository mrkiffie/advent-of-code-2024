const INPUT: &str = include_str!("input.txt");

#[tracing::instrument(level = "trace", skip())]
pub fn run() -> String {
    process(INPUT).to_string()
}

#[tracing::instrument(level = "trace", skip(input))]
fn process(input: &str) -> usize {
    let mut disk = parse(input);

    let mut head: usize = 0;
    let mut tail: usize = disk.len() - 1;

    while head < tail {
        if disk[head].is_none() && disk[tail].is_some() {
            disk.swap(head, tail);
        }
        while disk[head].is_some() {
            head += 1;
        }
        while disk[tail].is_none() {
            tail -= 1;
        }
    }

    disk.iter()
        .enumerate()
        .filter_map(|(i, v)| v.map(|v| v * i))
        .sum()
}

fn parse(input: &str) -> Vec<Option<usize>> {
    input
        .trim()
        .chars()
        .enumerate()
        .fold(Vec::new(), |mut acc, (i, x)| {
            let item = if i % 2 == 0 { Some(i / 2) } else { None };
            let n = x.to_digit(10).unwrap();
            (0..n).for_each(|_| {
                acc.push(item);
            });
            acc
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = process("2333133121414131402");
        assert_eq!(result, 1928);
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
