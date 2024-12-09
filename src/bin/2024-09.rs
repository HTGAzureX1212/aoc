use itertools::Itertools;

advent_of_code::solution!(2024, 09);

pub fn part_one(input: &str) -> Option<i64> {
    let mut empties = 0;
    let mut data = input
        .trim_end()
        .char_indices()
        .map(|(i, c)| {
            let num = c.to_digit(10).unwrap();
            let repeater = if i % 2 == 0 {
                i as i64 / 2
            } else {
                empties += num;
                -1
            };
            std::iter::repeat(repeater).take(num as usize)
        })
        .flatten()
        .collect_vec();

    while empties > 0 {
        let l = data.iter().position(|x| *x == -1).unwrap();
        let r = data.iter().rposition(|x| *x != -1).unwrap();

        if l > r {
            break;
        }

        data.swap(l, r);
        empties -= 1;
    }

    Some(
        data.iter()
            .enumerate()
            .filter(|(_, k)| **k != -1)
            .map(|(i, k)| i as i64 * *k)
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<i64> {
    let mut data = input
        .trim_end()
        .char_indices()
        .map(|(i, c)| {
            let num = c.to_digit(10).unwrap();
            let k = if i % 2 == 0 { i as i64 / 2 } else { -1 };
            (num, k)
        })
        .collect_vec();

    let (_, mut max) = data
        .clone()
        .iter()
        .max_by(|(_, k), (_, l)| k.cmp(l))
        .unwrap();
    loop {
        if max <= 0 {
            break;
        }

        let r = data
            .iter()
            .rposition(|(_, k)| *k != -1 && *k <= max)
            .unwrap();
        max -= 1;
        let Some(l) = data
            .iter()
            .position(|(size, k)| *size >= data[r].0 && *k == -1)
        else {
            continue;
        };

        if l > r {
            continue;
        }

        data.swap(l, r);

        if data[r].0 > data[l].0 {
            let diff = data[l].0.abs_diff(data[r].0);
            data[r].0 -= diff;
            data.insert(l + 1, (diff, -1));
        }
    }

    Some(
        data.iter()
            .map(|(repeat, k)| std::iter::repeat(*k).take(*repeat as usize))
            .flatten()
            .enumerate()
            .filter(|(_, k)| *k != -1)
            .map(|(i, k)| i as i64 * k)
            .sum(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1928));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2858));
    }
}
