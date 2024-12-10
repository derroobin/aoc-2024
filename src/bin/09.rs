advent_of_code::solution!(9);

pub fn part_one(input: &str) -> Option<u64> {
    let input = input.trim();
    let high_index: u32 = input.chars().map(|c| c.to_digit(10).unwrap()).sum();

    let mut reverse = ((0..input.len()).rev().zip(input.chars().rev()))
        .scan(high_index - 1, |base_index, (compressed_index, c)| {
            let num_indices = c.to_digit(10).unwrap();
            *base_index -= num_indices;

            Some(
                (*base_index..(*base_index + num_indices))
                    .rev()
                    .filter_map(move |i| {
                        (compressed_index % 2 == 0).then_some((i, compressed_index / 2))
                    }),
            )
        })
        .flatten();

    let mut base_index = 0;
    let mut sum = 0;
    let mut last_uncompressed_index = u32::MAX;

    for (compressed_index, c) in input.chars().enumerate() {
        let num_indices = c.to_digit(10).unwrap() as usize;
        let file_id = compressed_index / 2;

        for uncompressed_index in base_index..(base_index + num_indices) {
            if uncompressed_index >= last_uncompressed_index as usize {
                break;
            }
            if compressed_index % 2 == 0 {
                sum += uncompressed_index * file_id;
            } else {
                let (rev_uncompressed_index, file_id) = reverse.next().unwrap();

                sum += uncompressed_index * file_id;
                last_uncompressed_index = rev_uncompressed_index;
            }
        }

        base_index += num_indices;
    }

    Some(sum as u64)
}

pub fn part_two(input: &str) -> Option<u32> {
    let q: u32 = input
        .trim()
        .chars()
        .map(|c| c.to_digit(10).unwrap() as u32)
        .sum();
    dbg!(q);
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1928));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
