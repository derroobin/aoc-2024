use std::{
    cmp::Ordering,
    collections::{HashMap, HashSet},
};

use itertools::Itertools;
use nom::{
    bytes::complete::{tag, take_while1},
    character::complete::{self, newline},
    multi::{separated_list0, separated_list1},
    sequence::separated_pair,
    IResult,
};

advent_of_code::solution!(5);

fn parse(input: &str) -> IResult<&str, (Vec<(u32, u32)>, Vec<Vec<u32>>)> {
    let (input, order) = separated_list1(
        newline,
        separated_pair(complete::u32, tag("|"), complete::u32),
    )(input)?;

    let (input, _) = newline(input)?;
    let (input, _) = newline(input)?;

    let (input, pages) = separated_list1(newline, separated_list1(tag(","), complete::u32))(input)?;

    Ok((input, (order, pages)))
}

fn is_ordered(orders: &HashMap<u32, Vec<u32>>, page: &Vec<u32>) -> bool {
    let mut stash = HashSet::new();

    for p in page {
        let order = orders.get(&p);

        match order {
            None => {
                stash.insert(p);
            }
            Some(x) => {
                for value in x {
                    if stash.contains(value) {
                        return false;
                    }
                    stash.insert(p);
                }
            }
        }
    }

    true
}

pub fn part_one(input: &str) -> Option<u32> {
    let (_, (parsed_order, pages)) = parse(input).unwrap();

    let mut orders: HashMap<u32, Vec<u32>> = HashMap::new();

    for (key, value) in parsed_order {
        orders.entry(key).or_insert_with(Vec::new).push(value);
    }

    let q = pages
        .into_iter()
        .filter_map(|page| {
            let len = page.len();

            let center = page[len / 2];

            match is_ordered(&orders, &page) {
                true => Some(center as u32),
                false => None,
            }
        })
        .sum();

    Some(q)
}

pub fn part_two(input: &str) -> Option<u32> {
    let (_, (parsed_order, pages)) = parse(input).unwrap();

    let mut orders: HashMap<u32, Vec<u32>> = HashMap::new();

    for (key, value) in parsed_order {
        orders.entry(key).or_insert_with(Vec::new).push(value);
    }

    let q = pages
        .into_iter()
        .filter(|page| !is_ordered(&orders, &page))
        .map(|mut x| {
            x.sort_by(|a, b| {
                let order: Option<&Vec<u32>> = orders.get(a);

                if order.is_some() {
                    let q = order.unwrap().contains(b);

                    if q {
                        return Ordering::Less;
                    }
                }

                let order = orders.get(b);

                if order.is_some() {
                    let q = order.unwrap().contains(a);

                    if q {
                        return Ordering::Greater;
                    }
                }

                Ordering::Equal
            });
            x[x.len() / 2]
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
        assert_eq!(result, Some(143));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(123));
    }
}
