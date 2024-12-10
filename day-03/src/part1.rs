use nom::{
    bytes::complete::{tag, take_until},
    character::complete::{char, u32},
    combinator::opt,
    multi::many0,
    sequence::{delimited, separated_pair},
    IResult,
};

const INPUT: &str = include_str!("input.txt");

#[tracing::instrument(level = "trace", skip())]
pub fn run() -> usize {
    process(INPUT) as usize
}

#[tracing::instrument(level = "trace", skip(input))]
fn process(input: &str) -> u32 {
    input
        .lines()
        .map(parse_line)
        .map(|numbers| numbers.into_iter().map(|(a, b)| a * b).sum::<u32>())
        .sum()
}

#[tracing::instrument(level = "trace", skip(input))]
fn extract_mul(input: &str) -> IResult<&str, Option<(u32, u32)>> {
    let (input, _leading) = take_until("mul")(input)?;
    let (input, maybe_numbers) = opt(delimited(
        tag("mul("),
        separated_pair(u32, char(','), u32),
        char(')'),
    ))(input)?;

    if maybe_numbers.is_some() {
        Ok((input, maybe_numbers))
    } else {
        // consume "mul" so that the parser can continue
        let (input, _) = tag("mul")(input)?;
        Ok((input, None))
    }
}

#[tracing::instrument(level = "trace", skip(input))]
fn parse_line(input: &str) -> Vec<(u32, u32)> {
    let (_input, result) = many0(extract_mul)(input).unwrap();

    result
        .iter()
        .flatten()
        .map(|a| a.to_owned())
        .collect::<Vec<(u32, u32)>>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_mul() {
        let (tail, numbers) = extract_mul("then(mul(11,8)m").unwrap();
        assert_eq!(tail, "m");
        assert_eq!(numbers, Some((11, 8)));

        let (tail, numbers) = extract_mul("then(mul(11,8!m").unwrap();
        assert_eq!(tail, "(11,8!m");
        assert_eq!(numbers, None);
    }

    #[test]
    fn test_parse_line() {
        let result =
            process("xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))");
        assert_eq!(result, 161);
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
