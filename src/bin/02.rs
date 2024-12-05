use nom::{
    character::complete::{newline, space1},
    multi::separated_list1,
    IResult,
};
advent_of_code::solution!(2);

fn read_file(input: &str) -> IResult<&str, Vec<Vec<i32>>> {
    separated_list1(
        newline,
        separated_list1(space1, nom::character::complete::i32),
    )(input)
}

fn check_safety(list: &Vec<i32>) -> bool {
    let (min, max) = list.windows(2).map(|win| win[0] - win[1]).fold(
        (i32::MAX, i32::MIN),
        |(mut min, mut max), ele| {
            if ele < min {
                min = ele;
            }

            if ele > max {
                max = ele;
            }
            (min, max)
        },
    );

    if min.abs() > 3 || max.abs() > 3 {
        return false;
    }

    if min <= 0 && max >= 0 {
        return false;
    }

    true
}

pub fn part_one(input: &str) -> Option<i32> {
    let (_, list) = read_file(input).unwrap();

    let q = list
        .into_iter()
        .filter_map(|x| match check_safety(&x) {
            true => Some(1),
            false => None,
        })
        .sum();
    Some(q)
}

pub fn part_two(input: &str) -> Option<i32> {
    let (_, list) = read_file(input).unwrap();

    let q = list
        .into_iter()
        .filter_map(|x| {
            let qq = check_safety(&x);
            let test = match qq {
                true => Some(1),
                false => {
                    for index in 0..x.len() {
                        let mut new_list = x.clone();

                        new_list.remove(index);
                        if check_safety(&new_list) {
                            return Some(1);
                        }
                    }
                    return None;
                }
            };
            test
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
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4));
    }
}
