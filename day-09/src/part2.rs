use std::fmt::Display;

const INPUT: &str = include_str!("input.txt");

#[tracing::instrument(level = "trace", skip())]
pub fn run() -> usize {
    process(INPUT)
}

#[tracing::instrument(level = "trace", skip(input))]
fn process(input: &str) -> usize {
    let mut disk = parse(input);
    let mut tail: usize = disk.len() - 1;
    while 0 < tail {
        // find file
        while disk[tail].is_space() {
            tail -= 1;
        }
        let file_size = disk[tail].len;
        let file_index = disk[tail].index;

        let mut found = false;

        // find space
        let mut space_size: usize = 0;
        for space_index in 0..file_index {
            if disk[space_index].is_space() {
                space_size += 1;
            } else {
                space_size = 0;
            }

            // swap
            if space_size == file_size {
                let space_index = space_index - (space_size - 1);
                (0..file_size).for_each(|offset| {
                    disk.swap(space_index + offset, file_index + offset);
                });

                // update tail
                tail -= file_size;
                found = true;
                break;
            }
        }
        if !found {
            tail -= 1;
        }
        // repeat
    }

    disk.iter()
        .enumerate()
        .filter_map(|(index, Block { id, .. })| id.map(|id| id * index))
        .sum()
}

#[derive(Debug)]
struct Block {
    index: usize,
    len: usize,
    id: Option<usize>,
}

impl Block {
    fn is_space(&self) -> bool {
        self.id.is_none()
    }
}

impl Display for Block {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.id {
            Some(id) => write!(f, "{}", id),
            None => write!(f, "."),
        }
    }
}

fn parse(input: &str) -> Vec<Block> {
    input
        .trim()
        .chars()
        .enumerate()
        .fold(Vec::new(), |mut acc, (sequence_index, len)| {
            let index = acc.len();
            let len = len.to_digit(10).unwrap() as usize;
            (0..len).for_each(|_| {
                acc.push(if sequence_index % 2 == 0 {
                    Block {
                        index,
                        len,
                        id: Some(sequence_index / 2),
                    }
                } else {
                    Block {
                        index,
                        len: 1,
                        id: None,
                    }
                });
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
        assert_eq!(result, 2858);
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
