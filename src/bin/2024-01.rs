advent_of_code::solution!(2024, 01);

pub fn part_one(input: &str) -> Option<u32> {
    let (mut l, mut r): (Vec<u32>, Vec<u32>) = input
        .lines()
        .map(|line| line.split("   ").collect::<Vec<_>>())
        .map(|vec| {
            (
                vec[0].parse::<u32>().unwrap(),
                vec[1].parse::<u32>().unwrap(),
            )
        })
        .unzip();

    l.sort_unstable();
    r.sort_unstable();

    Some(
        l.iter()
            .zip(r.iter())
            .map(|(li, ri)| li.abs_diff(*ri))
            .sum::<u32>(),
    )
}

pub fn part_two(input: &str) -> Option<u64> {
    let (l, r): (Vec<u32>, Vec<u32>) = input
        .lines()
        .map(|line| line.split("   ").collect::<Vec<_>>())
        .map(|vec| {
            (
                vec[0].parse::<u32>().unwrap(),
                vec[1].parse::<u32>().unwrap(),
            )
        })
        .unzip();

    Some(
        l.iter()
            .map(|i| r.iter().filter(|j| i == *j).count() as u64 * *i as u64)
            .sum::<u64>(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(31));
    }
}
