use nom::{bytes::complete::{tag, take, take_until}, character::complete::{alpha0, alphanumeric0}, multi::separated_list1, sequence::separated_pair, IResult};

advent_of_code::solution!(3);


fn read_mul(input: &str) -> IResult<&str, (u32, u32)> {
    let (input, _) = tag("mul(")(input)?;
    let (input,out) = separated_pair(nom::character::complete::u32, tag(","), nom::character::complete::u32)(input)?;
    let (input,  _) = tag(")")(input)?;


    Ok((input, out))
}b

fn read_input(input: &str) -> IResult<&str, Vec<(u32, u32)>> {
    separated_list1(alphanumeric0, read_mul)(input)
}


pub fn part_one(input: &str) -> Option<u32> {
    let (_, list) =  read_input(input).unwrap();

    println!("{:#?}", list);

    Some(0)
}

pub fn part_two(input: &str) -> Option<u32> {
    None
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
        assert_eq!(result, None);
    }
}
