use glam::IVec2;
use pathfinding::prelude::dijkstra;
use std::fmt::Write;

const INPUT: &str = include_str!("input.txt");

#[tracing::instrument(level = "trace", skip())]
pub fn run() -> usize {
    println!("{}", process(INPUT, 71, 1024));

    0
}

#[tracing::instrument(level = "trace", skip(input))]
fn process(input: &str, size: i32, n: usize) -> String {
    let points = input
        .lines()
        .map(|line| {
            let (x, y) = line.split_once(',').expect("should be valid x,y pair");
            IVec2 {
                x: x.parse::<i32>().expect("should be valid integer"),
                y: y.parse::<i32>().expect("should be valid integer"),
            }
        })
        .collect::<Vec<_>>();

    let mut low = n;
    let mut high = points.len() - 1;
    let mut mid = (low + high) / 2;

    loop {
        if has_a_path(&points[0..=mid], size) {
            low = mid + 1;
        } else {
            high = mid;
        }

        mid = (low + high) / 2;

        if low == mid || mid == high {
            break;
        }
    }

    let mut count = low;

    while has_a_path(&points[0..count], size) {
        count += 1;
    }
    let last_point = points.get(count - 1).expect("there should be a blocker");

    format!("{},{}", last_point.x, last_point.y)
}

fn has_a_path(points: &[IVec2], size: i32) -> bool {
    let directions = [IVec2::NEG_Y, IVec2::X, IVec2::Y, IVec2::NEG_X];

    let successors = |point: &IVec2| {
        directions
            .iter()
            .map(|direction| point + direction)
            .filter(|IVec2 { x, y }| *x >= 0 && *x < size && *y >= 0 && *y < size)
            .filter(|point| !points.contains(point))
            .map(|point| (point, 1))
            .collect::<Vec<_>>()
    };

    let path = dijkstra(&IVec2::new(0, 0), successors, |point| {
        *point == IVec2::new(size - 1, size - 1)
    });
    path.is_some()
}

#[allow(dead_code)]
fn debug_grid(points: &[IVec2], path: &[IVec2], size: i32) {
    let mut out = String::with_capacity((size * size + size) as usize);
    for y in 0..size {
        for x in 0..size {
            let point = IVec2::new(x, y);
            match (path.contains(&point), points.contains(&point)) {
                (true, true) => write!(out, "X"),
                (true, false) => write!(out, "O"),
                (false, true) => write!(out, "#"),
                (false, false) => write!(out, "."),
            }
            .ok();
        }
        writeln!(out).ok();
    }

    println!("{}", out);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = process(
            "5,4
4,2
4,5
3,0
2,1
6,3
2,4
1,5
0,6
3,3
2,6
5,1
1,2
5,5
2,5
6,5
1,4
0,4
6,4
1,1
6,1
1,0
0,5
1,6
2,0",
            7,
            12,
        );
        assert_eq!(result, "6,1");
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
        super::process(INPUT, 71, 1024);
    }
}
