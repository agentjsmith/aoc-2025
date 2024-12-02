use itertools::Itertools;

advent_of_code::solution!(2);

pub fn levels_are_safe<I>(levels: I) -> bool
where
    I: IntoIterator<Item = u32>,
{
    let diffs: Vec<i32> = levels
        .into_iter()
        .inspect(|x| print!("{:?} ", x))
        .tuple_windows()
        .map(|(a, b)| a as i32 - b as i32)
        .collect_vec();

    let direction = diffs[0].signum();
    if diffs.iter().any(|d| d.signum() != direction) {
        println!("UNSAFE: monotonicity");
        return false; // levels are not safe; not monotonic!
    }

    if diffs.iter().any(|d| d.abs() < 1 || d.abs() > 3) {
        println!("UNSAFE: levels vary too much");
        return false; // levels ase not safe; difference not 1-3
    }

    println!("SAFE");
    true
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut num_safe = 0u32;
    for line in input.lines() {
        let levels = line.split_whitespace().map(|x| x.parse::<u32>().unwrap());
        if levels_are_safe(levels) {
            num_safe += 1;
        }
    }
    Some(num_safe)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut num_safe = 0u32;
    for line in input.lines() {
        let levels = line
            .split_whitespace()
            .map(|x| x.parse::<u32>().unwrap())
            .collect_vec();
        if levels_are_safe(levels.clone()) {
            num_safe += 1;
        } else {
            let len = levels.len() - 1;
            let mut tries = levels.into_iter().combinations(len);
            if tries.any(|x| levels_are_safe(x)) {
                num_safe += 1;
            }
        }
    }
    Some(num_safe)
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
