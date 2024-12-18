use glam::IVec2;
use pathfinding::prelude::dijkstra;
use std::fmt::Write;

const INPUT: &str = include_str!("input.txt");

#[tracing::instrument(level = "trace", skip())]
pub fn run() -> usize {
    process(INPUT, 71, 1024)
}

#[tracing::instrument(level = "trace", skip(input))]
fn process(input: &str, size: i32, n: usize) -> usize {
    let points = input
        .lines()
        .map(|line| {
            let (x, y) = line.split_once(',').expect("should be valid x,y pair");
            IVec2 {
                x: x.parse::<i32>().expect("should be valid integer"),
                y: y.parse::<i32>().expect("should be valid integer"),
            }
        })
        .take(n)
        .collect::<Vec<_>>();

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

    let (path, _cost) = dijkstra(&IVec2::new(0, 0), successors, |point| {
        *point == IVec2::new(size - 1, size - 1)
    })
    .expect("there should be a valid path");

    // debug_grid(&points, &path, size);

    path.len() - 1
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
        assert_eq!(result, 22);
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
