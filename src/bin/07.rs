use nom::{
    bytes::complete::tag,
    character::complete::{line_ending, space1, u64},
    multi::separated_list1,
    sequence::separated_pair,
    IResult,
};
use rayon::prelude::*;
advent_of_code::solution!(7);

fn equation(input: &str) -> IResult<&str, (u64, Vec<u64>)> {
    separated_pair(u64, tag(": "), separated_list1(space1, u64))(input)
}

fn parser(input: &str) -> IResult<&str, Vec<(u64, Vec<u64>)>> {
    separated_list1(line_ending, equation)(input)
}

fn prepare_concat(a: u64, b: u64) -> u64 {
    format!("{}{}", a, b).parse::<u64>().unwrap()
}

fn check_recursive(
    expected: u64,
    values: &Vec<u64>,
    current: u64,
    index: usize,
    part_two: bool,
) -> u64 {
    if values.len() <= index {
        return current;
    }

    if current > expected {
        return current;
    }

    let multiplication = check_recursive(
        expected,
        values,
        current * values[index],
        index + 1,
        part_two,
    );

    if multiplication == expected {
        return multiplication;
    }

    if part_two {
        let prep = prepare_concat(current, values[index]);

        let concat = check_recursive(expected, values, prep, index + 1, part_two);
        if concat == expected {
            return concat;
        }
    }

    return check_recursive(
        expected,
        values,
        current + values[index],
        index + 1,
        part_two,
    );
}

fn check_result(result: u64, values: &Vec<u64>, part_two: bool) -> bool {
    result == check_recursive(result, values, values[0], 1, part_two)
}

pub fn part_one(input: &str) -> Option<u64> {
    let (_, equations) = parser(input).expect("read input");

    let q = equations
        .par_iter()
        .filter_map(|(res, values)| match check_result(*res, values, false) {
            true => Some(res),
            false => None,
        })
        .sum();

    Some(q)
}

pub fn part_two(input: &str) -> Option<u64> {
    let (_, equations) = parser(input).expect("read input");

    let q = equations
        .par_iter()
        .filter_map(|(res, values)| match check_result(*res, values, true) {
            true => Some(res),
            false => None,
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
        assert_eq!(result, Some(3749));
    }

    #[test]
    fn test_part_two() {
        assert_eq!(prepare_concat(2, 1), 21);

        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11387));
    }
}
