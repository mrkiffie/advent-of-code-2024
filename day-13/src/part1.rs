use std::cmp::Ordering;

use glam::IVec2;

const INPUT: &str = include_str!("input.txt");

#[tracing::instrument(level = "trace", skip())]
pub fn run() -> usize {
    process(INPUT)
}

#[derive(Debug, Default)]
struct ClawMachine {
    a: IVec2,
    b: IVec2,
    p: IVec2,
}

const BUTTON_PREFIX: usize = "Button _: X+".len();
const PRIZE_PREFIX: usize = "Prize: X=".len();

#[tracing::instrument(level = "trace", skip(input))]
fn process(input: &str) -> usize {
    let tokens = input
        .trim()
        .split("\n\n")
        .map(|chunk| {
            chunk
                .lines()
                .enumerate()
                .filter_map(|(i, line)| -> Option<(usize, IVec2)> {
                    match i {
                        0 | 1 => line[BUTTON_PREFIX..].split_once(", Y+"),
                        2 => line[PRIZE_PREFIX..].split_once(", Y="),
                        _ => None,
                    }
                    .map(|(x, y)| (i, IVec2::new(int(x), int(y))))
                })
                .fold(ClawMachine::default(), |mut claw, (i, point)| {
                    match i {
                        0 => claw.a = point,
                        1 => claw.b = point,
                        2 => claw.p = point,
                        _ => {}
                    }
                    claw
                })
        })
        .filter(|ClawMachine { a, b, p }| {
            // Remove items that can never go far enough
            (a.x + b.x) * 100 > p.x && (a.y + b.y) * 100 > p.y
        })
        .map(|ClawMachine { a, b, p }| {
            let mut claw = a;

            let slope = |IVec2 { x, y }| x as f32 / y as f32;
            let p_slope = slope(p);
            let a_slope = slope(a);
            let b_slope = slope(b);
            assert!(a_slope != b_slope);

            let mut a_count = 1;
            let mut b_count = 0;
            let mut count = 3;

            #[cfg(debug_assertions)]
            println!();
            #[cfg(debug_assertions)]
            println!();
            #[cfg(debug_assertions)]
            println!();

            #[cfg(debug_assertions)]
            println!("prize: {:?}, a: {:?}, b: {:?}", p, a, b);

            #[cfg(debug_assertions)]
            print!("🌕");
            while claw != p && b_count < 100 && a_count < 100 {
                let c_slope = slope(claw);

                match c_slope.partial_cmp(&p_slope) {
                    Some(Ordering::Equal) => {
                        #[cfg(debug_assertions)]
                        print!("");
                        // vector is the same - so be able to multiply the current count by N to get to the point
                        if claw == p {
                            #[cfg(debug_assertions)]
                            print!("🟢");

                            return count;
                        } else {
                            let n = p.x / claw.x;
                            let m = p.y / claw.y;

                            if n == m && n * claw.x == p.x && m * claw.y == p.y {
                                #[cfg(debug_assertions)]
                                print!("🟣");
                                return count * n as usize;
                            } else {
                                return 0;
                            }
                        }
                    }
                    Some(Ordering::Greater) => {
                        if a_slope > b_slope {
                            #[cfg(debug_assertions)]
                            print!("🔴");
                            claw += b;
                            count += 1;
                            b_count += 1;
                        } else {
                            #[cfg(debug_assertions)]
                            print!("🌕");
                            claw += a;
                            count += 3;
                            a_count += 1;
                        }
                    }
                    Some(Ordering::Less) => {
                        if a_slope > b_slope {
                            #[cfg(debug_assertions)]
                            print!("🌕");
                            claw += a;
                            count += 3;
                            a_count += 1;
                        } else {
                            #[cfg(debug_assertions)]
                            print!("🔴");
                            claw += b;
                            count += 1;
                            b_count += 1;
                        }
                    }
                    None => todo!(),
                }

                if claw.x > p.x || claw.y > p.y {
                    break;
                }
            }

            if claw == p {
                count
            } else {
                0
            }
        })
        .sum();
    #[cfg(debug_assertions)]
    println!();
    #[cfg(debug_assertions)]
    println!();

    tokens
}

fn int(s: &str) -> i32 {
    s.parse::<i32>().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = process(
            "Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279",
        );
        assert_eq!(result, 480);
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
