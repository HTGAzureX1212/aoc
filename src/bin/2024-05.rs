#![feature(int_roundings)]

use std::cmp;

advent_of_code::solution!(2024, 05);

#[derive(Clone, Copy)]
struct Rule {
    left: u8,
    right: u8,
}

pub fn part_one(input: &str) -> Option<u64> {
    let parts = input.split("\n\n").collect::<Vec<_>>();
    let rules = parts[0]
        .lines()
        .map(|line| {
            let scanned: (u8, u8) = serde_scan::scan!("{}|{}" <- line).unwrap();
            Rule {
                left: scanned.0,
                right: scanned.1,
            }
        })
        .collect::<Vec<_>>();

    Some(
        parts[1]
            .lines()
            .into_iter()
            .filter_map(|line| {
                let vec = line
                    .split(',')
                    .map(|part| part.parse::<u8>().unwrap())
                    .collect::<Vec<_>>();

                if rules.iter().all(|rule| {
                    vec.iter()
                        .position(|x| *x == rule.left)
                        .and_then(|idx1| {
                            vec.iter()
                                .position(|x| *x == rule.right)
                                .and_then(|idx2| Some(idx1 < idx2))
                        })
                        .unwrap_or(true)
                }) {
                    Some(vec[(vec.len() - 1).div_floor(2)] as u64)
                } else {
                    None
                }
            })
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u64> {
    let parts = input.split("\n\n").collect::<Vec<_>>();
    let rules = parts[0]
        .lines()
        .map(|line| {
            let scanned: (u8, u8) = serde_scan::scan!("{}|{}" <- line).unwrap();
            Rule {
                left: scanned.0,
                right: scanned.1,
            }
        })
        .collect::<Vec<_>>();

    Some(
        parts[1]
            .lines()
            .into_iter()
            .filter_map(|line| {
                let vec = line
                    .split(',')
                    .map(|part| part.parse::<u8>().unwrap())
                    .collect::<Vec<_>>();

                if rules.iter().all(|rule| {
                    vec.iter()
                        .position(|x| *x == rule.left)
                        .and_then(|idx1| {
                            vec.iter()
                                .position(|x| *x == rule.right)
                                .and_then(|idx2| Some(idx1 < idx2))
                        })
                        .unwrap_or(true)
                }) {
                    None
                } else {
                    Some(vec)
                }
            })
            .map(|mut vec| {
                vec.sort_by(|a, b| {
                    if rules
                        .iter()
                        .filter(|rule| *a == rule.left)
                        .any(|rule| *b == rule.right)
                    {
                        cmp::Ordering::Less
                    } else if rules
                        .iter()
                        .filter(|rule| *b == rule.right)
                        .any(|rule| *a == rule.left)
                    {
                        cmp::Ordering::Greater
                    } else {
                        cmp::Ordering::Equal
                    }
                });
                vec
            })
            .map(|vec| vec[(vec.len() - 1).div_floor(2)] as u64)
            .sum(),
    )
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
        assert_eq!(result, None);
    }
}
