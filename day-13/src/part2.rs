use glam::I64Vec2;

const INPUT: &str = include_str!("input.txt");

#[tracing::instrument(level = "trace", skip())]
pub fn run() -> usize {
    process(INPUT)
}

#[derive(Debug, Default)]
struct ClawMachine {
    a: I64Vec2,
    b: I64Vec2,
    p: I64Vec2,
}

const BUTTON_PREFIX: usize = "Button _: X+".len();
const PRIZE_PREFIX: usize = "Prize: X=".len();

#[tracing::instrument(level = "trace", skip(input))]
fn process(input: &str) -> usize {
    input
        .trim()
        .split("\n\n")
        .map(|chunk| {
            chunk
                .lines()
                .enumerate()
                .filter_map(|(i, line)| -> Option<(usize, I64Vec2)> {
                    match i {
                        0 | 1 => line[BUTTON_PREFIX..].split_once(", Y+"),
                        2 => line[PRIZE_PREFIX..].split_once(", Y="),
                        _ => None,
                    }
                    .map(|(x, y)| (i, I64Vec2::new(int(x), int(y))))
                })
                .fold(ClawMachine::default(), |mut claw, (i, point)| {
                    match i {
                        0 => claw.a = point,
                        1 => claw.b = point,
                        2 => claw.p = point + I64Vec2::splat(10000000000000),
                        _ => {}
                    }
                    claw
                })
        })
        .map(|ClawMachine { a, b, p }| {
            // Cramer's rule

            // { A * a.x + B * b.x = p.x
            // { A * a.y + B * b.y = p.y

            // The given equations in the form of AX = B

            // A = [ a.x  b.x ]
            //     [ a.y  b.y ],
            // B = [ p.x ]
            //     [ p.y ],
            // X = [ A ]
            //     [ B ]

            // Then, the determinant D of matrix A
            // D = | a.x  b.x | = a.x * b.y - a.y * b.x
            //     | a.y  b.y |

            let determinant = a.x * b.y - a.y * b.x;

            // Now, find Da and Db
            // Da = | p.x b.x | = p.x * b.y - p.y * b.x
            //      | p.y b.y |
            // Db = | a.x p.x | = a.x * p.y - a.y * p.x
            //      | a.y p.y |

            let da = p.x * b.y - p.y * b.x;
            let db = a.x * p.y - a.y * p.x;

            // Now, find a_count = Da/D, b_count = Db/D

            let a_count = da / determinant;
            let b_count = db / determinant;

            if a.x * a_count + b.x * b_count == p.x && a.y * a_count + b.y * b_count == p.y {
                (a_count * 3 + b_count) as usize
            } else {
                0
            }
        })
        .sum()
}

fn int(s: &str) -> i64 {
    s.parse::<i64>().unwrap()
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
