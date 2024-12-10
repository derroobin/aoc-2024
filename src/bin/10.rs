use std::collections::{HashMap, HashSet};

advent_of_code::solution!(10);

type x = u32;

fn rec_search(
    list: &Vec<Vec<x>>,
    count: u32,
    y: usize,
    x: usize,
    part_two: bool,
    mut cache: &mut HashMap<(usize, usize), u32>,
) -> x {
    if y >= list.len() || x >= list[0].len() {
        return 0;
    }

    if list[y][x] != count {
        return 0;
    }

    if count == 9 {
        if !part_two {
            cache.insert((y, x), 0);
        }
        return 1;
    }

    let cached = cache.get(&(y, x));
    if cached.is_some() {
        return *cached.unwrap();
    }

    let mut sum = 0;

    if x > 0 {
        sum += rec_search(list, count + 1, y, x - 1, part_two, &mut cache);
    }
    if y > 0 {
        sum += rec_search(list, count + 1, y - 1, x, part_two, &mut cache);
    }
    sum += rec_search(list, count + 1, y + 1, x, part_two, &mut cache);
    sum += rec_search(list, count + 1, y, x + 1, part_two, &mut cache);

    if part_two {
        cache.insert((y, x), sum);
    }

    sum
}

pub fn run_search(list: &Vec<Vec<x>>) -> x {
    let mut part_1 = 0;

    for y in 0..list.len() {
        for x in 0..list[y].len() {
            if list[y][x] == 0 {
                let mut cache: HashMap<(usize, usize), u32> = HashMap::new();
                rec_search(list, 0, y, x, false, &mut cache);

                part_1 += cache.len() as x;
            }
        }
    }

    part_1
}

pub fn run_search_part2(list: &Vec<Vec<x>>) -> x {
    let mut sum = 0;
    let mut cache: HashMap<(usize, usize), u32> = HashMap::new();

    for y in 0..list.len() {
        for x in 0..list[y].len() {
            if list[y][x] == 0 {
                sum += rec_search(list, 0, y, x, true, &mut cache);
            }
        }
    }

    sum
}

pub fn part_one(input: &str) -> Option<x> {
    let list: Vec<Vec<x>> = input
        .trim()
        .lines()
        .map(|line| line.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect();

    let sum = run_search(&list);
    Some(sum)
}

pub fn part_two(input: &str) -> Option<u32> {
    let list: Vec<Vec<x>> = input
        .trim()
        .lines()
        .map(|line| line.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect();

    let sum = run_search_part2(&list);
    Some(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(36));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(81));
    }
}
