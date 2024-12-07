use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{self, anychar},
    combinator::value,
    multi::{many1, many_till},
    sequence::{delimited, separated_pair},
    IResult, Parser,
};

advent_of_code::solution!(3);

#[derive(Debug, Clone)]
enum Instruction {
    Mul(u32, u32),
    Do,
    Dont,
}

fn mul(input: &str) -> IResult<&str, Instruction> {
    let (input, _) = tag("mul")(input)?;

    let (input, (a, b)) = delimited(
        tag("("),
        separated_pair(complete::u32, tag(","), complete::u32),
        tag(")"),
    )(input)?;

    Ok((input, Instruction::Mul(a, b)))
}

fn read_input(input: &str) -> IResult<&str, Vec<Instruction>> {
    many1(many_till(anychar, mul).map(|(_, a)| a))(input)
}

fn read_input2(input: &str) -> IResult<&str, Vec<Instruction>> {
    many1(
        many_till(
            anychar,
            alt((
                mul,
                value(Instruction::Do, tag("do()")),
                value(Instruction::Dont, tag("don't()")),
            )),
        )
        .map(|(_, a)| a),
    )(input)
}

pub fn part_one(input: &str) -> Option<u32> {
    let (_, list) = read_input(input).unwrap();

    let q = list
        .into_iter()
        .map(|y| match y {
            Instruction::Mul(a, b) => a * b,
            Instruction::Do => 0,
            Instruction::Dont => 0,
        })
        .sum();
    Some(q)
}

pub fn part_two(input: &str) -> Option<u32> {
    let (_, list) = read_input2(input).unwrap();
    let mut allow = 1;
    let q = list
        .into_iter()
        .map(|y| match y {
            Instruction::Mul(a, b) => allow * a * b,
            Instruction::Do => {
                allow = 1;
                0
            }
            Instruction::Dont => {
                allow = 0;
                0
            }
        })
        .sum();
    Some(q)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(161));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(48));
    }
}
