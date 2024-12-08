advent_of_code::solution!(2024, 07);

pub fn part_one(input: &str) -> Option<i64> {
    Some(
        input
            .lines()
            .map(|line| line.split_once(": ").unwrap())
            .map(|(left, right)| {
                (
                    left.parse::<i64>().unwrap(),
                    right
                        .split(' ')
                        .map(|s| s.parse::<i64>().unwrap())
                        .collect::<Vec<_>>(),
                )
            })
            .filter(|(l, r)| valid((*l, r.clone())))
            .map(|(left, _)| left)
            .sum::<i64>(),
    )
}

pub fn part_two(input: &str) -> Option<i64> {
    Some(
        input
            .lines()
            .map(|line| line.split_once(": ").unwrap())
            .map(|(left, right)| {
                (
                    left.parse::<i64>().unwrap(),
                    right
                        .split(' ')
                        .map(|s| s.parse::<i64>().unwrap())
                        .collect::<Vec<_>>(),
                )
            })
            .filter(|(l, r)| valid_p2((*l, r.clone())))
            .map(|(left, _)| left)
            .sum::<i64>(),
    )
}

fn valid((left, right): (i64, Vec<i64>)) -> bool {
    if right.len() == 1 {
        return left == right[0];
    }

    let new = *right.last().unwrap();
    let next = right[..right.len() - 1].to_vec();

    if left % new == 0 && valid((left / new, next.clone())) {
        return true;
    }

    valid((left - new, next))
}

fn valid_p2((left, right): (i64, Vec<i64>)) -> bool {
    if right.len() == 1 {
        return left == right[0];
    }

    let new = *right.last().unwrap();
    let next = right[..right.len() - 1].to_vec();

    if (left - new) % 10i64.pow(new.ilog10() + 1) == 0
        && valid_p2(((left - new) / 10i64.pow(new.ilog10() + 1), next.clone()))
    {
        return true;
    }

    if left % new == 0 && valid_p2((left / new, next.clone())) {
        return true;
    }

    valid_p2((left - new, next))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3749));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11387));
    }
}
