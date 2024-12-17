use std::fmt::Display;
use std::ops::{BitXor, Rem};

const INPUT: &str = include_str!("input.txt");

#[tracing::instrument(level = "trace", skip())]
pub fn run() -> usize {
    println!("{}", process(INPUT));
    0
}

#[tracing::instrument(level = "trace", skip(input))]
fn process(input: &str) -> String {
    let mut program = Program::new(input);

    program.run();

    program.to_string()
}

#[derive(Debug, PartialEq, Eq)]
struct Program {
    a: usize,
    b: usize,
    c: usize,
    program: Vec<usize>,
    i: usize,
    out: Vec<usize>,
}

impl Program {
    fn new(input: &str) -> Self {
        let mut lines = input.trim().lines();

        let init_register = |line: Option<&str>| {
            line.and_then(|line| line[12..].parse::<usize>().ok())
                .expect("register should have a value")
        };
        // register a
        let a = init_register(lines.next());
        // register b
        let b = init_register(lines.next());
        // register c
        let c = init_register(lines.next());
        let _ = lines.next();
        let program = lines
            .next()
            .map(|line| {
                line[9..]
                    .split(',')
                    .filter_map(|x| x.parse::<usize>().ok())
                    .collect::<Vec<_>>()
            })
            .expect("there to be a program");

        Self {
            a,
            b,
            c,
            program,
            i: 0,
            out: vec![],
        }
    }

    fn combo_operand(&self, x: usize) -> usize {
        match x {
            4 => self.a,
            5 => self.b,
            6 => self.c,
            7 => unreachable!("invalid combo operand"),
            _ => x,
        }
    }
    fn run(&mut self) {
        while self.step().is_some() {}
    }

    fn step(&mut self) -> Option<()> {
        let opcode = self.program.get(self.i);
        let operand = self.program.get(self.i + 1);
        match (opcode, operand) {
            // The adv instruction (opcode 0) performs division.
            // The numerator is the value in the A register.
            // The denominator is found by raising 2 to the power of the instruction's combo operand.
            // (So, an operand of 2 would divide A by 4 (2^2);
            // an operand of 5 would divide A by 2^B.)
            // The result of the division operation is truncated to an integer and then written to the A register.
            (Some(0), Some(x)) => {
                self.a /= 2_u32.pow(self.combo_operand(*x) as u32) as usize;
                self.i += 2;
                Some(())
            }
            // The bxl instruction (opcode 1) calculates the bitwise XOR of register B and
            // the instruction's literal operand, then stores the result in register B.
            (Some(1), Some(x)) => {
                self.b = self.b.bitxor(x);
                self.i += 2;
                Some(())
            }
            // The bst instruction (opcode 2) calculates the value of its combo operand modulo 8
            // (thereby keeping only its lowest 3 bits), then writes that value to the B register.
            (Some(2), Some(x)) => {
                self.b = self.combo_operand(*x).rem(8);
                self.i += 2;
                Some(())
            }
            // The jnz instruction (opcode 3) does nothing if the A register is 0. However,
            // if the A register is not zero, it jumps by setting the instruction pointer to
            // the value of its literal operand; if this instruction jumps, the instruction
            // pointer is not increased by 2 after this instruction.
            (Some(3), Some(x)) => {
                if self.a == 0 {
                    self.i += 2;
                } else {
                    self.i = *x;
                }
                Some(())
            }
            // The bxc instruction (opcode 4) calculates the bitwise XOR of register B and register C,
            // then stores the result in register B.
            // (For legacy reasons, this instruction reads an operand but ignores it.)
            (Some(4), Some(_)) => {
                self.b = self.b.bitxor(self.c);
                self.i += 2;
                Some(())
            }
            // The out instruction (opcode 5) calculates the value of its combo operand modulo 8,
            // then outputs that value. (If a program outputs multiple values, they are separated by commas.)
            (Some(5), Some(x)) => {
                self.out.push(self.combo_operand(*x).rem(8));
                self.i += 2;
                Some(())
            }
            // The bdv instruction (opcode 6) works exactly like the adv instruction except that the
            // result is stored in the B register. (The numerator is still read from the A register.)
            (Some(6), Some(x)) => {
                self.b = self.a / (2_u32.pow(self.combo_operand(*x) as u32) as usize);
                self.i += 2;
                Some(())
            }
            // The cdv instruction (opcode 7) works exactly like the adv instruction except that the
            // result is stored in the C register. (The numerator is still read from the A register.)
            (Some(7), Some(x)) => {
                self.c = self.a / (2_u32.pow(self.combo_operand(*x) as u32) as usize);
                self.i += 2;
                Some(())
            }
            _ => None,
        }
    }
}

impl Display for Program {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut it = self.out.iter();
        let first = it.next().expect("there should be outout");
        write!(f, "{}", first)?;
        for x in it {
            write!(f, ",{}", x)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_1() {
        // If register C contains 9, the program 2,6 would set register B to 1.
        let mut p = Program {
            a: 0,
            b: 0,
            c: 9,
            program: vec![2, 6],
            i: 0,
            out: vec![],
        };
        p.step();

        assert_eq!(p.b, 1);
        assert_eq!(p.i, 2);
    }

    #[test]
    fn test_case_2() {
        // If register A contains 10, the program 5,0,5,1,5,4 would output 0,1,2.
        let mut p = Program {
            a: 10,
            b: 0,
            c: 0,
            program: vec![5, 0, 5, 1, 5, 4],
            i: 0,
            out: vec![],
        };
        p.run();
        assert_eq!(p.out, vec![0, 1, 2]);
        assert_eq!(p.i, 6);
    }

    #[test]
    fn test_case_3() {
        // If register A contains 2024, the program 0,1,5,4,3,0 would output 4,2,5,6,7,7,7,7,3,1,0 and leave 0 in register A.
        let mut p = Program {
            a: 2024,
            b: 0,
            c: 0,
            program: vec![0, 1, 5, 4, 3, 0],
            i: 0,
            out: vec![],
        };
        p.run();
        assert_eq!(p.out, vec![4, 2, 5, 6, 7, 7, 7, 7, 3, 1, 0]);
        assert_eq!(p.a, 0);
        assert_eq!(p.i, 6);
    }

    #[test]
    fn test_case_4() {
        // If register B contains 29, the program 1,7 would set register B to 26.
        let mut p = Program {
            a: 0,
            b: 29,
            c: 0,
            program: vec![1, 7],
            i: 0,
            out: vec![],
        };
        p.run();
        assert_eq!(p.b, 26);
        assert_eq!(p.i, 2);
    }

    #[test]
    fn test_case_5() {
        // If register B contains 2024 and register C contains 43690, the program 4,0 would set register B to 44354.
        let mut p = Program {
            a: 0,
            b: 2024,
            c: 43690,
            program: vec![4, 0],
            i: 0,
            out: vec![],
        };
        p.run();
        assert_eq!(p.b, 44354);
        assert_eq!(p.i, 2);
    }

    #[test]
    fn test_input_parsing() {
        let input = "Register A: 729
Register B: 0
Register C: 0

Program: 0,1,5,4,3,0
";
        let parsed = Program::new(input);

        let expected = Program {
            a: 729,
            b: 0,
            c: 0,
            program: vec![0, 1, 5, 4, 3, 0],
            i: 0,
            out: vec![],
        };
        assert_eq!(parsed, expected);
    }

    #[test]
    fn it_works() {
        let result = process(
            "Register A: 729
Register B: 0
Register C: 0

Program: 0,1,5,4,3,0
",
        );
        assert_eq!(result, "4,6,3,5,6,3,5,2,1,0");
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
