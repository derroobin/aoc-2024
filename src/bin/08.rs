use std::collections::HashSet;

use glam::IVec2;
use itertools::Itertools;
use nom::{
    bytes::complete::take_till, character::complete::satisfy, multi::many1, sequence::preceded,
    AsChar, IResult,
};
use nom_locate::{position, LocatedSpan};

advent_of_code::solution!(8);

pub type Span<'a> = LocatedSpan<&'a str>;

fn token(input: Span) -> IResult<Span, (IVec2, char)> {
    let (input, pos) = position(input)?;

    let (input, char) = satisfy(|c| c.is_alphanum())(input)?;

    Ok((
        input,
        (
            IVec2::new(pos.get_column() as i32 - 1, pos.location_line() as i32 - 1),
            char,
        ),
    ))
}

fn parse(input: Span) -> IResult<Span, Vec<(IVec2, char)>> {
    many1(preceded(take_till(|c: char| c.is_alphanum()), token))(input)
}

pub fn part_one(input: &str) -> Option<u32> {
    let len = input.lines().take(1).map(|x| x.len() as i32).sum();

    let range = 0..len;

    let span = Span::from(input);

    let (_, mut input) = parse(span).expect("parse");

    input.sort_by(|a, b| a.1.cmp(&b.1));

    let q = input
        .chunk_by(|a, b| a.1 == b.1)
        .into_iter()
        .flat_map(|chunk| {
            chunk
                .iter()
                .tuple_combinations()
                .flat_map(|(a, b)| {
                    let distance = a.0 - b.0;

                    vec![a.0 + distance, b.0 - distance]
                })
                .filter(|x| range.contains(&x.x) && range.contains(&x.y))
        })
        .collect::<HashSet<_>>();

    Some(q.len() as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let len = input.lines().take(1).map(|x| x.len() as i32).sum();

    let range = 0..len;

    let span = Span::from(input);

    let (_, mut input) = parse(span).expect("parse");

    input.sort_by(|a, b| a.1.cmp(&b.1));

    let q = input
        .chunk_by(|a, b| a.1 == b.1)
        .into_iter()
        .flat_map(|chunk| {
            chunk.iter().tuple_combinations().flat_map(|(a, b)| {
                let distance = a.0 - b.0;

                let mut container = Vec::new();

                let mut current = a.0 - distance;

                while range.contains(&current.x) && range.contains(&current.y) {
                    container.push(current);
                    current += distance;
                }
                current = b.0 + distance;

                while range.contains(&current.x) && range.contains(&current.y) {
                    container.push(current);
                    current -= distance;
                }
                container
            })
        })
        .unique()
        .count();

    Some(q as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(14));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(34));
    }
}
