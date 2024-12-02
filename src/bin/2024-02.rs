advent_of_code::solution!(2024, 02);

use std::cmp::Reverse;

use itertools::Itertools;

pub fn part_one(input: &str) -> Option<usize> {
    Some(
        input
            .lines()
            .map(|s| s.split_whitespace().map(|s| s.parse::<i8>().unwrap()))
            .filter(safe_predicate)
            .count(),
    )
}

pub fn part_two(input: &str) -> Option<usize> {
    let already_safe = part_one(input).unwrap();

    let damped = input
        .lines()
        .map(|s| s.split_whitespace().map(|s| s.parse::<i8>().unwrap()))
        .filter(|iter| !safe_predicate(iter))
        .filter(damped_safe_predicate)
        .count();

    Some(already_safe + damped)
}

fn safe_predicate(iter: &(impl Iterator<Item = i8> + Clone)) -> bool {
    (iter.clone().is_sorted() || iter.clone().is_sorted_by_key(|a| Reverse(a)))
        && iter
            .clone()
            .tuple_windows()
            .map(|(a, b)| a.abs_diff(b))
            .all(|d| (1..=3).contains(&d))
}

fn damped_safe_predicate(iter: &(impl Iterator<Item = i8> + Clone)) -> bool {
    let vec = iter.clone().collect::<Vec<_>>();
    for i in 0..vec.len() {
        let mut new = vec.clone();
        new.remove(i);
        if safe_predicate(&new.into_iter()) {
            return true;
        }
    }

    false
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
