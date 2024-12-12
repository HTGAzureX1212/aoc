#![allow(static_mut_refs)]

use std::{collections::HashMap, sync::OnceLock};

use itertools::Itertools;

advent_of_code::solution!(2024, 11);

static mut CACHE: OnceLock<HashMap<(String, u32), u64>> = OnceLock::new();

pub fn part_one(input: &str) -> Option<u64> {
    unsafe {
        CACHE.get_or_init(HashMap::new);
    }

    let stones = input
        .trim()
        .split(' ')
        .map(|com| com.to_string())
        .collect_vec();

    Some(stones.iter().map(|num| process(num.clone(), 25)).sum())
}

pub fn part_two(input: &str) -> Option<u64> {
    unsafe {
        CACHE.get_or_init(HashMap::new);
    }

    let stones = input
        .trim()
        .split(' ')
        .map(|com| com.to_string())
        .collect_vec();

    Some(stones.iter().map(|num| process(num.clone(), 75)).sum())
}

fn process(num: String, iteration: u32) -> u64 {
    if iteration == 0 {
        return 1;
    }

    if let Some(result) = unsafe { CACHE.get().unwrap().get(&(num.clone(), iteration)) } {
        return *result;
    }

    let as_int = num.parse::<u64>().unwrap();
    let result = if as_int == 0 {
        process("1".to_string(), iteration - 1)
    } else if num.len() % 2 == 0 {
        process(
            num[0..num.len() / 2]
                .to_string()
                .parse::<u64>()
                .unwrap()
                .to_string(),
            iteration - 1,
        ) + process(
            num[num.len() / 2..num.len()]
                .to_string()
                .parse::<u64>()
                .unwrap()
                .to_string(),
            iteration - 1,
        )
    } else {
        process((as_int * 2024).to_string(), iteration - 1)
    };

    *(unsafe {
        CACHE
            .get_mut()
            .unwrap()
            .entry((num, iteration))
            .or_insert(result)
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(55312));
    }
}
