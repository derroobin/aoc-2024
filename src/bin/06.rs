use std::{
    borrow::Borrow,
    collections::{HashMap, HashSet},
};

use glam::{ivec2, IVec2, IVec3};
use itertools::Itertools;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{char, line_ending},
    combinator::value,
    multi::{many1, separated_list1},
    IResult, InputIter, Parser,
};
use nom_locate::{position, LocatedSpan};
advent_of_code::solution!(6);

type Span<'a> = LocatedSpan<&'a str>;

#[derive(Debug, Clone, PartialEq)]
enum Direction {
    Up(),
    Down(),
    Left(),
    Right(),
}

#[derive(Debug, Clone, PartialEq)]
enum Token {
    Guard(Direction),
    Pillar(),
    Tile(),
}

impl Direction {
    fn rotate(&self) -> Direction {
        match self {
            Direction::Up() => Direction::Right(),
            Direction::Down() => Direction::Left(),
            Direction::Left() => Direction::Up(),
            Direction::Right() => Direction::Down(),
        }
    }
    fn to_ivec2(&self) -> IVec2 {
        match self {
            Direction::Up() => IVec2::NEG_Y,
            Direction::Down() => IVec2::Y,
            Direction::Right() => IVec2::X,
            Direction::Left() => IVec2::NEG_X,
        }
    }
}

fn token(input: Span) -> IResult<Span, (IVec2, Token)> {
    let (s, pos) = position(input)?;

    let (input, token) = alt((
        value(Token::Tile(), tag(".")),
        value(Token::Pillar(), tag("#")),
        value(Token::Guard(Direction::Down()), tag("v")),
        value(Token::Guard(Direction::Up()), tag("^")),
        value(Token::Guard(Direction::Right()), tag(">")),
        value(Token::Guard(Direction::Left()), tag("<")),
    ))(input)?;

    Ok((
        input,
        (
            IVec2::new(pos.get_column() as i32 - 1, pos.location_line() as i32 - 1),
            token,
        ),
    ))
}

fn parser(input: Span) -> IResult<Span, (HashMap<IVec2, Token>, IVec2, Direction, IVec2)> {
    let (input, x) = separated_list1(line_ending, many1(token))(input)?;

    let start = x
        .iter()
        .flatten()
        .find(|(_, token)| match token {
            Token::Guard(x) => true,
            _ => false,
        })
        .cloned()
        .expect("to find a start");

    let dir: Direction = match start.1 {
        Token::Guard(dir) => Some(dir),
        _ => None,
    }
    .expect("we need a guard to have a direction");

    let last_x = x
        .iter()
        .flatten()
        .map(|(vec, _)| vec.x)
        .max()
        .expect("need size");

    let last_y: i32 = x
        .iter()
        .flatten()
        .map(|(vec, _)| vec.x)
        .max()
        .expect("need size");

    let map: HashMap<IVec2, Token> = x
        .into_iter()
        .flatten()
        .filter(|(_, token)| match token {
            Token::Pillar() => true,
            _ => false,
        })
        .collect();

    Ok((input, (map, start.0, dir, IVec2::new(last_x, last_y))))
}

pub fn part_one(input: &str) -> Option<u32> {
    let span = Span::new(input);

    let (_, (map, mut pos, mut dir, size)) = parser(span).expect("failed parsing input");

    let range_x = 0..=size.x;
    let range_y = 0..=size.y;

    let mut path = HashSet::from([]);
    while range_x.contains(&pos.x) && range_y.contains(&pos.y) {
        let new_pos = pos + dir.to_ivec2();

        if map.get(&new_pos).is_some() {
            dir = dir.rotate();
            continue;
        }

        path.insert(pos);
        pos = new_pos;
    }

    Some(path.len() as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let span = Span::new(input);

    let (_, (mut map, start, start_dir, size)) = parser(span).expect("failed parsing input");

    let range_x = 0..=size.x;
    let range_y = 0..=size.y;

    let possibles = vec![
        IVec2::new(-1, -1),
        IVec2::new(-1, 1),
        IVec2::new(1, -1),
        IVec2::new(1, 1),
    ];

    let mut found = HashSet::new();

    let mut path = HashSet::from([]);
    let mut pos = start;
    let mut dir = start_dir.clone();
    while range_x.contains(&pos.x) && range_y.contains(&pos.y) {
        let new_pos = pos + dir.to_ivec2();

        if map.get(&new_pos).is_some() {
            dir = dir.rotate();
            continue;
        }

        path.insert(pos);
        pos = new_pos;
    }

    let permutations = map.clone();

    let pillar_positions: HashSet<IVec2> = map
        .keys()
        .permutations(2)
        .flat_map(|perm| {
            let a = perm[0];
            let b = perm[1];

            possibles
                .iter()
                .map(|x| x + IVec2::new(b.x, a.y))
                .filter(|x| !permutations.contains_key(x) && path.contains(x))
        })
        .collect();

    for pillar in pillar_positions {
        map.insert(pillar, Token::Pillar());

        let mut path = HashSet::new();

        let mut pos = start;
        let mut dir = start_dir.clone();

        while range_x.contains(&pos.x) && range_y.contains(&pos.y) {
            let dir_vec = dir.to_ivec2();
            let new_pos = pos + dir_vec;

            let key = (pos, dir_vec);

            if path.contains(&key) {
                found.insert(pillar);
                break;
            }

            if map.get(&new_pos).is_some() {
                dir = dir.rotate();
                continue;
            }

            path.insert(key);
            pos = new_pos;
        }

        map.remove(&pillar);
    }

    Some(found.len() as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(41));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }
}
