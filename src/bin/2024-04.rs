#![feature(int_roundings)]
#![feature(let_chains)]

use core::str;

advent_of_code::solution!(2024, 04);

pub fn part_one(input: &str) -> Option<u64> {
    let crosswords = input.lines().collect::<Vec<_>>();
    let mut res = 0;

    for y in 0..crosswords.len() as i32 {
        for x in 0..crosswords[0].len() as i32 {
            'lp: for (dx, dy) in [
                (1, 0),
                (0, 1),
                (-1, 0),
                (0, -1),
                (1, 1),
                (-1, -1),
                (1, -1),
                (-1, 1),
            ] {
                for (c, i) in b"XMAS".iter().zip(0..) {
                    if let Some(s) = crosswords.get((x + dx * i) as usize)
                        && let Some(f) = s.as_bytes().get((y + dy * i) as usize)
                        && f == c
                    {
                        continue;
                    }

                    continue 'lp;
                }

                res += 1;
            }
        }
    }

    Some(res)
}

pub fn part_two(input: &str) -> Option<u64> {
    let crosswords = input.lines().collect::<Vec<_>>();
    let mut res: u64 = 0;

    for y in 0..crosswords.len() as i32 {
        for x in 0..crosswords[0].len() as i32 {
            if let Some(b'A') = crosswords
                .get(x as usize)
                .unwrap()
                .as_bytes()
                .get(y as usize)
            {
                let bytes = [(-1, -1), (1, -1), (-1, 1), (1, 1)]
                    .into_iter()
                    .filter_map(|(dx, dy)| {
                        crosswords
                            .get((x + dx) as usize)
                            .and_then(|s| s.as_bytes().get((y + dy) as usize))
                    })
                    .copied()
                    .collect::<Vec<_>>();

                if matches!(
                    str::from_utf8(bytes.as_slice()).unwrap(),
                    "MMSS" | "MSMS" | "SSMM" | "SMSM"
                ) {
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
        assert_eq!(result, Some(3));
    }
}
