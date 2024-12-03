use regex::Regex;

advent_of_code::solution!(2024, 03);

pub fn part_one(input: &str) -> Option<u64> {
    let regex = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();

    Some(
        regex
            .captures_iter(input)
            .map(|capture| capture.extract())
            .map(|(_, [l, r])| l.parse::<u64>().unwrap() * r.parse::<u64>().unwrap())
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u64> {
    let regex = Regex::new(r"don\'t\(\)|do\(\)|mul\((\d+),(\d+)\)").unwrap();

    let mut on = true;

    Some(
        regex
            .captures_iter(input)
            .map(|capture| match &capture[0] {
                "don't()" => {
                    on = false;
                    0
                }
                "do()" => {
                    on = true;
                    0
                }
                _ if on => capture[1].parse::<u64>().unwrap() * capture[2].parse::<u64>().unwrap(),
                _ => 0,
            })
            .sum(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(322));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(209));
    }
}
