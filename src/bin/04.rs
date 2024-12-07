use itertools::Itertools;

advent_of_code::solution!(4);

fn check_add(index: usize, dir: i32) -> Option<usize> {
    match dir.is_positive() {
        true => index.checked_add(dir as usize),
        false => index.checked_sub(dir.abs() as usize),
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let lines = input
        .lines()
        .into_iter()
        .map(|x| x.chars().collect_vec())
        .collect_vec();

    let directions = vec![
        // W
        vec![(0, 1), (0, 2), (0, 3)],
        // O
        vec![(0, -1), (0, -2), (0, -3)],
        // N
        vec![(-1, 0), (-2, 0), (-3, 0)],
        //S
        vec![(1, 0), (2, 0), (3, 0)],
        // NO
        vec![(-1, -1), (-2, -2), (-3, -3)],
        // NW
        vec![(-1, 1), (-2, 2), (-3, 3)],
        // SO
        vec![(1, -1), (2, -2), (3, -3)],
        // SW
        vec![(1, 1), (2, 2), (3, 3)],
    ];

    let chars = vec!['M', 'A', 'S'];
    let cy = lines.len();
    let cx = lines[0].len();

    let mut res = 0;

    for y in 0..cy {
        for x in 0..cx {
            if lines[y][x] == 'X' {
                for dir in &directions {
                    let mut ok = true;
                    let last_y = check_add(y, dir[2].0);
                    let last_x = check_add(x, dir[2].1);

                    if last_x.is_none() || last_x.unwrap() >= cx {
                        continue;
                    }

                    if last_y.is_none() || last_y.unwrap() >= cy {
                        continue;
                    }

                    for (i, cur) in dir.iter().enumerate() {
                        let cury = check_add(y, cur.0).unwrap();
                        let curx = check_add(x, cur.1).unwrap();

                        if lines[cury][curx] != chars[i] {
                            ok = false;
                            break;
                        }
                    }

                    if ok {
                        res += 1;
                    }
                }
            }
        }
    }

    Some(res)
}

pub fn check_line(lines: &Vec<Vec<char>>, y: usize, x: usize, a: bool) -> bool {
    let directions = vec![
        // SO A
        vec![(-1, -1), (1, 1)],
        // SO Bc
        vec![(1, 1), (-1, -1)],
        // NO A
        vec![(1, -1), (-1, 1)],
        // NO B
        vec![(-1, 1), (1, -1)],
    ];
    let chars = vec!['M', 'S'];

    let cy = lines.len();
    let cx = lines[0].len();

    let dir_range = match a {
        true => 0..=1,
        false => 2..=3,
    };

    for dir_index in dir_range {
        let dir = &directions[dir_index];

        let a_y = check_add(y, dir[0].0);
        let a_x = check_add(x, dir[0].1);

        let b_y = check_add(y, dir[1].0);
        let b_x = check_add(x, dir[1].1);

        if a_y.is_none()
            || a_x.is_none()
            || b_y.is_none()
            || b_x.is_none()
            || a_y.unwrap() >= cy
            || a_x.unwrap() >= cx
            || b_y.unwrap() >= cy
            || b_x.unwrap() >= cx
        {
            return false;
        }

        let (one_y, one_x, two_y, two_x) = (a_y.unwrap(), a_x.unwrap(), b_y.unwrap(), b_x.unwrap());

        if lines[one_y][one_x] == chars[0] && lines[two_y][two_x] == chars[1] {
            return true;
        }
    }

    false
}

pub fn part_two(input: &str) -> Option<u32> {
    let lines: Vec<Vec<char>> = input
        .lines()
        .into_iter()
        .map(|x| x.chars().collect_vec())
        .collect_vec();

    let cy = lines.len();
    let cx = lines[0].len();

    let mut res = 0;

    for y in 0..cy {
        for x in 0..cx {
            if lines[y][x] == 'A' {
                if check_line(&lines, y, x, false) && check_line(&lines, y, x, true) {
                    res += 1;
                }
            }
        }
    }

    Some(res)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(18));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(9));
    }
}
