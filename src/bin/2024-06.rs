#![feature(let_chains)]

use std::{collections::HashSet, hash::Hash};

use itertools::Itertools;

advent_of_code::solution!(2024, 06);

#[derive(Clone, Copy, PartialEq, Eq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Hash for Direction {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        match self {
            Self::Up => state.write_u8(0),
            Self::Down => state.write_u8(1),
            Self::Left => state.write_u8(2),
            Self::Right => state.write_u8(3),
        }
    }
}

struct Cursor {
    dir: Direction,
    x: usize,
    y: usize,
}

impl Cursor {
    pub fn new(x: usize, y: usize) -> Self {
        Self {
            dir: Direction::Up,
            x,
            y,
        }
    }

    pub fn forward(&mut self) {
        match self.dir {
            Direction::Up => self.y -= 1,
            Direction::Down => self.y += 1,
            Direction::Left => self.x -= 1,
            Direction::Right => self.x += 1,
        }
    }

    pub fn backward(&mut self) {
        match self.dir {
            Direction::Up => self.y += 1,
            Direction::Down => self.y -= 1,
            Direction::Left => self.x += 1,
            Direction::Right => self.x -= 1,
        }
    }

    pub fn turn(&mut self) {
        self.dir = match self.dir {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        };
    }

    pub fn coordinates(&self) -> (usize, usize) {
        (self.x, self.y)
    }

    pub fn headed(&self) -> Direction {
        self.dir
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let matrix = input
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let (y, vec) = matrix
        .iter()
        .find_position(|row| row.iter().find_position(|k| **k == '^').is_some())
        .unwrap();
    let (x, _) = vec.iter().find_position(|k| **k == '^').unwrap();
    let mut cursor = Cursor::new(x, y);

    let mut visited = HashSet::new();

    let mut count = 0;
    while let Some(row) = matrix.get(cursor.coordinates().1)
        && let Some(c) = row.get(cursor.coordinates().0)
    {
        if visited.insert((cursor.coordinates().0, cursor.coordinates().1)) && *c != '#' {
            count += 1;
        }

        if *c == '#' {
            cursor.backward();
            cursor.turn();
            continue;
        }
        cursor.forward();
    }

    Some(count)
}

pub fn part_two(input: &str) -> Option<u64> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(41));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
