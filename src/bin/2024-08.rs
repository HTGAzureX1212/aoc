use std::collections::{HashMap, HashSet};

use itertools::Itertools;

advent_of_code::solution!(2024, 08);

pub fn part_one(input: &str) -> Option<usize> {
    let grid = input
        .lines()
        .map(|line| line.chars().collect_vec())
        .collect_vec();

    let mut map = HashMap::<char, Vec<_>>::new();
    grid.iter().enumerate().for_each(|(row, line)| {
        line.iter().enumerate().for_each(|(col, c)| {
            if *c != '.' {
                map.entry(*c).or_insert(Vec::new()).push((col, row));
            }
        })
    });

    let mut result = HashSet::new();
    map.values().for_each(|pos| {
        pos.iter().for_each(|pair| {
            pos.iter().filter(|pair2| pair != *pair2).for_each(|pair3| {
                let new = (
                    (pair3.0 as isize - (pair.0 as isize - pair3.0 as isize)),
                    (pair3.1 as isize - (pair.1 as isize - pair3.1 as isize)),
                );
                let new2 = (
                    (pair.0 as isize + (pair.0 as isize - pair3.0 as isize)),
                    (pair.1 as isize + (pair.1 as isize - pair3.1 as isize)),
                );

                if new.0 >= 0
                    && new.0 < grid.len() as isize
                    && new.1 >= 0
                    && new.1 < grid[0].len() as isize
                {
                    result.insert(new);
                }
                if new2.0 >= 0
                    && new2.0 < grid.len() as isize
                    && new2.1 >= 0
                    && new2.1 < grid[0].len() as isize
                {
                    result.insert(new2);
                }
            });
        });
    });

    Some(result.len())
}

pub fn part_two(input: &str) -> Option<usize> {
    let grid = input
        .lines()
        .map(|line| line.chars().collect_vec())
        .collect_vec();

    let mut map = HashMap::<char, Vec<_>>::new();
    grid.iter().enumerate().for_each(|(row, line)| {
        line.iter().enumerate().for_each(|(col, c)| {
            if *c != '.' {
                map.entry(*c).or_insert(Vec::new()).push((col, row));
            }
        })
    });

    let mut result = HashSet::new();
    map.values().for_each(|pos| {
        pos.iter().for_each(|pair| {
            pos.iter().filter(|pair2| pair != *pair2).for_each(|pair3| {
                let (dx, dy) = (
                    pair3.0 as isize - pair.0 as isize,
                    pair3.1 as isize - pair.1 as isize,
                );

                let mut new = (pair.0 as isize, pair.1 as isize);
                loop {
                    new = (new.0 - dx, new.1 - dy);
                    if new.0 >= 0
                        && new.0 < grid.len() as isize
                        && new.1 >= 0
                        && new.1 < grid.len() as isize
                    {
                        result.insert(new);
                    } else {
                        break;
                    };
                }

                new = (pair.0 as isize, pair.1 as isize);
                loop {
                    new = (new.0 + dx, new.1 + dy);
                    if new.0 >= 0
                        && new.0 < grid.len() as isize
                        && new.1 >= 0
                        && new.1 < grid.len() as isize
                    {
                        result.insert(new);
                    } else {
                        break;
                    };
                }
            });
        });
    });

    Some(result.len())
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
