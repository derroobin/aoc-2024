use nom::{
    character::complete::{self, space1},
    multi::separated_list1,
    IResult,
};
use num_traits::Euclid;
use std::collections::HashMap;

advent_of_code::solution!(11);

fn parser(input: &str) -> IResult<&str, Vec<u64>> {
    separated_list1(space1, complete::u64)(input)
}

fn num_digits(input: &u64) -> bool {
    input.ilog10() % 2 == 1
}

#[inline]
fn split_digit(input: &u64) -> (u64, u64) {
    let num_digits = 10u64.pow((input.ilog10() + 1) / 2);

    input.div_rem_euclid(&num_digits)
}

pub fn process(input: &str, cicles: i32) -> u64 {
    let (_, digits) = parser(input).unwrap();

    let mut cache: HashMap<u64, u64> = digits.iter().map(|x| (*x, 1u64)).collect();

    for _ in 0..cicles {
        let mut new_cache = HashMap::default();

        for (num, count) in cache.into_iter() {
            match num {
                0 => {
                    *new_cache.entry(1).or_insert(0) += count;
                }

                n if num_digits(&n) => {
                    let (left, right) = split_digit(&n);

                    *new_cache.entry(left).or_insert(0) += count;
                    *new_cache.entry(right).or_insert(0) += count;
                }

                n => {
                    *new_cache.entry(n * 2024).or_insert(0) += count;
                }
            }
        }

        cache = new_cache;
    }

    cache.values().sum()
}

pub fn part_one(input: &str) -> Option<u64> {
    let sum = process(input, 25);
    Some(sum)
}

pub fn part_two(input: &str) -> Option<u64> {
    let sum = process(input, 75);

    Some(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(55312));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(65601038650482));
    }
}
