use std::{cmp, collections::HashMap};

use nom::{
    character::complete::{multispace1, newline},
    multi::separated_list1,
    sequence::separated_pair,
    IResult,
};

advent_of_code::solution!(1);

fn parse_pair(input: &str) -> IResult<&str, (u32, u32)> {
    separated_pair(
        nom::character::complete::u32,
        multispace1,
        nom::character::complete::u32,
    )(input)
}

fn parse_file(input: &str) -> IResult<&str, Vec<(u32, u32)>> {
    separated_list1(newline, parse_pair)(input)
}

fn count<I>(it: I, item: &I::Item) -> u32
where
    I: IntoIterator,
    I::Item: PartialEq,
{
    it.into_iter().filter(|x| x == item).count() as u32
}

pub fn part_two(input: &str) -> Option<u32> {
    match parse_file(input) {
        Ok((_, pairs)) => {
            let (mut x, mut y): (Vec<_>, Vec<_>) = pairs.into_iter().unzip();

            x.sort();
            y.sort();

            let mut hashmap: HashMap<u32, u32> = HashMap::new();

            let mut sum: u32 = 0;
            for x_val in x.iter() {
                match hashmap.get(x_val) {
                    Some(a) => sum += a * x_val,

                    None => {
                        let count = count(&y, &x_val);
                        hashmap.insert(*x_val, count);
                        sum += count * x_val
                    }
                }
            }

            Some(sum)
        }

        Err(_) => Some(0),
    }
}
pub fn part_one(input: &str) -> Option<u32> {
    match parse_file(input) {
        Ok((_, pairs)) => {
            let (mut x, mut y): (Vec<_>, Vec<_>) = pairs.into_iter().unzip();

            x.sort();
            y.sort();

            let res = x
                .into_iter()
                .zip(y.into_iter())
                .fold(0, |acc, (a, b)| acc + cmp::max(a, b) - cmp::min(a, b));

            Some(res)
        }

        Err(_) => Some(0),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(31));
    }
}
