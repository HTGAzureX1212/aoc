use pathfinding::directed::bfs::bfs_reach;
use pathfinding::directed::count_paths::count_paths;

advent_of_code::solution!(2024, 10);

pub fn part_one(input: &str) -> Option<u64> {
    let input = input.as_bytes();
    let len = memchr::memchr(b'\n', input).unwrap() + 1;
    let dirs = [-(len as isize), 1, len as isize, -1];

    Some(
        memchr::memchr_iter(b'0', input)
            .map(|pos| {
                bfs_reach(pos, |&old| successors(input, old, &dirs))
                    .filter(|&k| input[k] == b'9')
                    .count() as u64
            })
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u64> {
    let input = input.as_bytes();
    let len = memchr::memchr(b'\n', input).unwrap() + 1;
    let dirs = [-(len as isize), 1, len as isize, -1];

    Some(
        memchr::memchr_iter(b'0', input)
            .map(|pos| {
                count_paths(
                    pos,
                    |&old| successors(input, old, &dirs),
                    |k| input[*k] == b'9',
                ) as u64
            })
            .sum(),
    )
}

fn successors<'a>(
    input: &'a [u8],
    from: usize,
    dirs: &'a [isize; 4],
) -> impl Iterator<Item = usize> + use<'a> {
    let from_val = input[from];
    dirs.into_iter().filter_map(move |&offset| {
        let pos = from.checked_add_signed(offset)?;
        let val = *input.get(pos)?;
        if val == from_val + 1 {
            Some(pos)
        } else {
            None
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(36));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
