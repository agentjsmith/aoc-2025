use std::iter::zip;

advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
    let lines = input.lines();
    let mut total = 0u32;

    let mut left = Vec::new();
    let mut right = Vec::new();

    for line in lines {
        if line.is_empty() {
            continue;
        }

        let mut parts = line.split_whitespace();
        let a = parts.next()?;
        let b = parts.next()?;

        let a = a.parse::<u32>().unwrap();
        let b = b.parse::<u32>().unwrap();

        left.push(a);
        right.push(b);
    }

    left.sort();
    right.sort();

    for (a, b) in zip(left, right) {
        let diff = a.abs_diff(b);
        println!("{} {} = {}", a, b, diff);

        total += diff;
    }

    Some(total)
}

pub fn part_two(input: &str) -> Option<u32> {
    let lines = input.lines();
    let mut total = 0u32;

    let mut left = Vec::new();
    let mut right = Vec::new();

    for line in lines {
        if line.is_empty() {
            continue;
        }

        let mut parts = line.split_whitespace();
        let a = parts.next()?;
        let b = parts.next()?;

        let a = a.parse::<u32>().unwrap();
        let b = b.parse::<u32>().unwrap();

        left.push(a);
        right.push(b);
    }

    left.sort();
    right.sort();

    for a in left {
        let b: u32 = right
            .iter()
            .filter(|x| **x == a)
            .count()
            .try_into()
            .expect("oopr");
        total += a * b;
    }

    Some(total)
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
