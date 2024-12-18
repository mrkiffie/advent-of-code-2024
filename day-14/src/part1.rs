use glam::IVec2;

const INPUT: &str = include_str!("input.txt");

#[tracing::instrument(level = "trace", skip())]
pub fn run() -> String {
    let grid = IVec2::new(101, 103);
    let ticks = 100;
    process(INPUT, grid, ticks).to_string()
}

struct Robot {
    position: IVec2,
    velocity: IVec2,
}

impl Robot {
    fn simulate(&mut self, grid: IVec2) {
        self.position += self.velocity;
        if self.position.x < 0 {
            self.position.x += grid.x;
        }
        if self.position.y < 0 {
            self.position.y += grid.y;
        }
        if self.position.x >= grid.x {
            self.position.x -= grid.x;
        }
        if self.position.y >= grid.y {
            self.position.y -= grid.y;
        }
    }

    fn quadrant(&self, grid: IVec2) -> usize {
        let mid_x = grid.x / 2;
        let mid_y = grid.y / 2;

        match (self.position.x, self.position.y) {
            (x, y) if x < mid_x && y < mid_y => 1,
            (x, y) if x > mid_x && y < mid_y => 2,
            (x, y) if x < mid_x && y > mid_y => 3,
            (x, y) if x > mid_x && y > mid_y => 4,
            _ => 0,
        }
    }
}

#[tracing::instrument(level = "trace", skip(input))]
fn process(input: &str, grid: IVec2, ticks: usize) -> usize {
    let quadrants = input
        .lines()
        .map(|line| {
            let line = line.strip_prefix("p=").unwrap();
            let (position, velocity) = line.split_once(" v=").unwrap();
            let (x, y) = position.split_once(',').unwrap();
            let x = x.parse::<i32>().unwrap();
            let y = y.parse::<i32>().unwrap();
            let position = IVec2::new(x, y);
            let (x, y) = velocity.split_once(',').unwrap();
            let x = x.parse::<i32>().unwrap();
            let y = y.parse::<i32>().unwrap();
            let velocity = IVec2::new(x, y);
            let mut robot = Robot { position, velocity };

            for _ in 0..ticks {
                robot.simulate(grid);
            }

            robot.quadrant(grid)
        })
        .fold((0, 0, 0, 0), |mut quadrants, quadrant| {
            match quadrant {
                1 => quadrants.0 += 1,
                2 => quadrants.1 += 1,
                3 => quadrants.2 += 1,
                4 => quadrants.3 += 1,
                _ => {}
            };
            quadrants
        });

    quadrants.0 * quadrants.1 * quadrants.2 * quadrants.3
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_robot_simulation() {
        let mut robot = Robot {
            position: IVec2::new(2, 4),
            velocity: IVec2::new(2, -3),
        };

        let grid = IVec2 { x: 11, y: 7 };

        robot.simulate(grid);
        assert!(robot.position == IVec2::new(4, 1));
        robot.simulate(grid);
        assert!(robot.position == IVec2::new(6, 5));
        robot.simulate(grid);
        assert!(robot.position == IVec2::new(8, 2));
        robot.simulate(grid);
        assert!(robot.position == IVec2::new(10, 6));
        robot.simulate(grid);
        assert!(robot.position == IVec2::new(1, 3));

        let mut robot = Robot {
            position: IVec2::new(10, 6),
            velocity: IVec2::new(1, 1),
        };
        robot.simulate(grid);
        assert!(robot.position == IVec2::new(0, 0));

        let mut robot = Robot {
            position: IVec2::new(0, 0),
            velocity: IVec2::new(-1, -1),
        };
        robot.simulate(grid);
        assert!(robot.position == IVec2::new(10, 6));
    }

    #[test]
    fn it_works() {
        let grid = IVec2::new(11, 7);
        let ticks = 100;

        let result = process(
            "p=0,4 v=3,-3
p=6,3 v=-1,-3
p=10,3 v=-1,2
p=2,0 v=2,-1
p=0,0 v=1,3
p=3,0 v=-2,-2
p=7,6 v=-1,-3
p=3,0 v=-1,-2
p=9,3 v=2,3
p=7,3 v=-1,2
p=2,4 v=2,-3
p=9,5 v=-3,-3",
            grid,
            ticks,
        );
        assert_eq!(result, 12);
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
        use glam::IVec2;
        let grid = IVec2::new(101, 103);
        let ticks = 100;
        super::process(INPUT, grid, ticks);
    }
}
