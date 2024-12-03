use nom::{
    branch::alt,
    bytes::complete::{tag, take},
    character::complete::{char, u32},
    combinator::{map, opt},
    multi::many0,
    sequence::{delimited, preceded, separated_pair},
    IResult,
};

const INPUT: &str = include_str!("input.txt");

pub fn run() -> String {
    process(INPUT).to_string()
    // process_with_nom(INPUT).to_string()
}

fn process(input: &str) -> u32 {
    let mut input = input;
    let mut total = 0;

    loop {
        let do_index = input.find("do()");
        let dont_index = input.find("don't()");
        let mul_index = input.find("mul(");

        match (do_index, dont_index, mul_index) {
            (_, _, None) => return total,
            (None, None, Some(mul)) => {
                input = &input[mul + 4..];
                let end = input.find(')');

                let Some(end) = end else {
                    return total;
                };

                if end > 7 {
                    // too far away - must be invalid.
                    continue;
                }

                let Some((left, right)) = &input[..end].split_once(',') else {
                    // no `,` before `)`
                    continue;
                };

                if !(1..=3).contains(&left.len()) {
                    continue;
                }
                if !(1..=3).contains(&right.len()) {
                    continue;
                }

                if !left.chars().all(|c| c.is_ascii_digit()) {
                    continue;
                }
                if !right.chars().all(|c| c.is_ascii_digit()) {
                    continue;
                }

                let left = left.parse::<u32>().unwrap_or_default();
                let right = right.parse::<u32>().unwrap_or_default();

                total += left * right;
                continue;
            }
            (None, Some(disable), Some(mul)) => {
                if disable < mul {
                    return total;
                }
            }
            (Some(enable), None, Some(mul)) => {
                if enable < mul {
                    // Nothing to do if enable is the smallest
                    //
                    input = &input[enable + 4..];
                    continue;
                }

                // When mul is smallest, calculate multiplication if it is valid
                if mul < enable {
                    input = &input[mul + 4..];
                    let end = input.find(')');

                    let Some(end) = end else {
                        return total;
                    };

                    if end > 7 {
                        // too far away - must be invalid.
                        continue;
                    }

                    let Some((left, right)) = &input[..end].split_once(',') else {
                        // no `,` before `)`
                        continue;
                    };

                    if !(1..=3).contains(&left.len()) {
                        continue;
                    }
                    if !(1..=3).contains(&right.len()) {
                        continue;
                    }

                    if !left.chars().all(|c| c.is_ascii_digit()) {
                        continue;
                    }
                    if !right.chars().all(|c| c.is_ascii_digit()) {
                        continue;
                    }

                    let left = left.parse::<u32>().unwrap_or_default();
                    let right = right.parse::<u32>().unwrap_or_default();

                    total += left * right;
                    continue;
                }

                unreachable!("shouldn't be able to reach here");
            }
            (Some(enable), Some(disable), Some(mul)) => {
                // Disabled is the smallest
                if disable < enable && disable < mul {
                    // Skip into the next enabled spot - skipping disabled mul
                    input = &input[enable + 4..];
                    continue;
                }

                if enable < mul && enable < disable {
                    // Nothing to do if enable is the smallest
                    //
                    input = &input[enable + 4..];
                    continue;
                }

                // When mul is smallest, calculate multiplication if it is valid
                if mul < enable && mul < disable {
                    input = &input[mul + 4..];
                    let end = input.find(')');

                    let Some(end) = end else {
                        return total;
                    };

                    if end > 7 {
                        // too far away - must be invalid.
                        continue;
                    }

                    let Some((left, right)) = &input[..end].split_once(',') else {
                        // no `,` before `)`
                        continue;
                    };

                    if !(1..=3).contains(&left.len()) {
                        continue;
                    }
                    if !(1..=3).contains(&right.len()) {
                        continue;
                    }

                    if !left.chars().all(|c| c.is_ascii_digit()) {
                        continue;
                    }
                    if !right.chars().all(|c| c.is_ascii_digit()) {
                        continue;
                    }

                    let left = left.parse::<u32>().unwrap_or_default();
                    let right = right.parse::<u32>().unwrap_or_default();

                    total += left * right;
                    continue;
                }

                unreachable!("shouldn't be able to reach here");
            }
        }
    }
}

fn process_with_nom(input: &str) -> u32 {
    parse_line(input)
        .iter()
        .fold(
            (true, 0),
            |(enabled, total), instruction| match instruction {
                Instruction::Multiply(a, b) if enabled => (enabled, total + a * b),
                Instruction::Do => (true, total),
                Instruction::Dont => (false, total),
                _ => (enabled, total),
            },
        )
        .1
}

#[derive(Debug, PartialEq, Clone)]
enum Instruction {
    Multiply(u32, u32),
    Do,
    Dont,
}

impl Instruction {
    fn from_tuple(input: (u32, u32)) -> Self {
        Instruction::Multiply(input.0, input.1)
    }
}

fn extract_mul(input: &str) -> IResult<&str, Option<Instruction>> {
    let (input, maybe_numbers) = opt(delimited(
        tag("mul("),
        map(separated_pair(u32, char(','), u32), Instruction::from_tuple),
        char(')'),
    ))(input)?;

    if maybe_numbers.is_some() {
        Ok((input, maybe_numbers))
    } else {
        // consume "mul(" so that the parser can continue
        let (input, _) = tag("mul(")(input)?;
        Ok((input, None))
    }
}

fn extract_do(input: &str) -> IResult<&str, Option<Instruction>> {
    let (input, _tag) = tag("do()")(input)?;
    Ok((input, Some(Instruction::Do)))
}

fn extract_dont(input: &str) -> IResult<&str, Option<Instruction>> {
    let (input, _tag) = tag("don't()")(input)?;
    Ok((input, Some(Instruction::Dont)))
}

fn take_until_keyword(input: &str) -> IResult<&str, ()> {
    let next_indices = [
        input.find("do()"),
        input.find("don't()"),
        input.find("mul("),
    ];
    let index = next_indices.iter().flatten().min();

    match index {
        Some(&index) => {
            let (input, _) = take(index)(input)?;
            Ok((input, ()))
        }
        None => Ok((&input[input.len()..], ())),
    }
}

fn parse_line(input: &str) -> Vec<Instruction> {
    let (_input, result) = many0(preceded(
        take_until_keyword,
        alt((extract_dont, extract_do, extract_mul)),
    ))(input)
    .unwrap();

    result
        .iter()
        .flatten()
        .map(|a| a.to_owned())
        .collect::<Vec<Instruction>>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_mul() {
        let (tail, numbers) = extract_mul("mul(11,8)m").unwrap();
        assert_eq!(tail, "m");
        assert_eq!(numbers, Some(Instruction::Multiply(11, 8)));

        let (tail, numbers) = extract_mul("mul(11,8!m").unwrap();
        assert_eq!(tail, "11,8!m");
        assert_eq!(numbers, None);
    }

    #[test]
    fn test_extract_do() {
        let (tail, numbers) = extract_do("do()m").unwrap();
        assert_eq!(tail, "m");
        assert_eq!(numbers, Some(Instruction::Do));
    }

    #[test]
    fn test_extract_dont() {
        let (tail, numbers) = extract_dont("don't()m").unwrap();
        assert_eq!(tail, "m");
        assert_eq!(numbers, Some(Instruction::Dont));
    }

    #[test]
    fn test_parse_line() {
        let result =
            process("xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))");
        assert_eq!(result, 48);
    }

    #[test]
    fn example_1() {
        let result =
            process("%how()how():?:mul(766,746)*@mul(364,566)-< who()(*':mul(999,344)*/select()--mul(672,593){how())<mul(73#)()@+mul(83,507)  mul(373,176)who()^'('mul(584,620)what()#//!do()mul(103,225) ~;;'why()*~mul(187,119)+/]mul+(select()%mul(874,888)}when():how()mul(583,992)^~[[what()don't()() {from() ]]((mul(68,200)^?what()who()*[mul(932,283)['$mul(189,932)< ,mul(652,125))$how()where()how()^how()#]mul(501,335)!when():+%])!<mul(551,924)+,#why()) $mul(118,951)@])/'who()mul(858,212) who(){-how()!don't())mul(746,402)/%}where()mul(629,312)];*~#]mul(680,3)what()how()what()'}?'@where();mul(263,427)#$$from()-what()mul(698,847)#(;$!$<+mul;why())what())$select()mul(482,169)-where()mul(546,79)mul(796,632)how()select()when()&$/*mul(749,226)-%what()>(who()'/<when()mul(932,346)?where(),^^>mul(722,627)>-?mul(231,501)~}#!mul(694,751)when()how()- where(202,572)select() }*^mul(17,75):+'what(),&mul(413,505)mul(113,65)[-+{,[mul(83,722)((mul(475,980)mul(588,832):/;)what()/+mul(103,764)?{$:?{{+:select()mul(583,487)mul(757,133)why()??mul(47,54)<]>select()>^?$mul(201,196)$from()]^~#where()mul(494,817)]?//-#select()%+mul(444,319)%?from()mul(316,303)}-~'<<-when()when()mul(350,810)mul(557,674)~##(select()$mul(97,781)who()(>>' >!),mul(473,488)who(290,952)mul(33,630)why()>do():)<select()~ mul(571,144){mul(931,78)mul(200,845)how()#select(403,528)mul(741,613)mul(54,465)@;(<[>mul(267,367)+/who())^select()^from()$!mul(409,900)*what()[)]who())[+where()mul(309,751)~don't()!mul(165,206)mul(113,418)]from(),'&do()select()/*:)]!mul(272,138) mul(211,851)]/$mul(916,846)mul(203,199)mul(40,428){&*from()%mul(305,353)? >}where()<what()(mul(904,794)+$from()-,/{mul(712,685)@ what(628,776)why(){;:;-mul(909,11){<,,mul(287,272),?),>%mul(397,337)]!mul(352,23)@don't()where()^{from()mul(804,392)${<}!mul(392,298),>>mul(572,89)+why()$*;when())#where()$mul(458,495);mul(375,386)~from()mul(429,704),{*%select()$who()]mul(442,21)#why()@?!mul(659,81)when()<($%^&&don't()!mul(934,729)/<[:how(288,214)'mul(971,226);+!%!mul(465,736)/]&%&^what(),+mul(613,544)-/from()what() },<-!mul(906,152)[who()&when()select()mul(612,56)~&<')/!mul(247,423)from()[{&who()mul(979,442)[mul(319,494)~%/+mul(781,251);<>)who()%from()[from()mul(27,381)}+)what()%/select(),,mul(324,64)mul(938,422)how():@>}:%'/&mul(388,707)]@mul(98,712)~who()$%@?(what()from()who()mul(161,906)~where():#mul(198,30)why() ~!>how()['-who()mul(5,68)what()<%%{mul(829,126):,mul(509,883)mul(142,939)do()#>mul(53,112)!(what()/?do()(,how()%mul(523,469) who(){what()'/mul(356,713)~@;!~ ->mul(309,932)where()mul(93,190)where()select()){how()}why()mul(202,888))!,{{:what(),~mul(591,813)select()<&{[&mul(652,199)");
        assert_eq!(result, 9083605);
    }
}

#[cfg(feature = "bench")]
pub mod benchmarks {
    use super::INPUT;

    pub fn main() {
        divan::main();
    }

    #[divan::bench(sample_count = 1000)]
    fn bench_process() {
        super::process(INPUT);
    }

    #[divan::bench(sample_count = 1000)]
    fn bench_process_with_nom() {
        super::process_with_nom(INPUT);
    }
}
