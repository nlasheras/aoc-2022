use aoc_runner_derive::aoc;
use aoc_runner_derive::aoc_generator;
use itertools::Itertools;
use std::collections::BTreeSet;

#[aoc_generator(day18)]
pub fn parse_input(input: &str) -> Vec<(i32, i32, i32)> {
    input
        .lines()
        .map(|x| {
            x.split(",")
                .map(|s| s.parse::<i32>().unwrap())
                .collect_tuple()
                .unwrap()
        })
        .collect()
}

#[aoc(day18, part1)]
pub fn find_surface_area(entries: &Vec<(i32, i32, i32)>) -> u64 {
    let mut cubes = BTreeSet::new();
    for c in entries {
        cubes.insert(c);
    }
    let mut sum = 0;
    for c in cubes.iter() {
        sum += [
            (1, 0, 0),
            (-1, 0, 0),
            (0, 1, 0),
            (0, -1, 0),
            (0, 0, 1),
            (0, 0, -1),
        ]
        .into_iter()
        .filter(|n| !cubes.contains(&(c.0 + n.0, c.1 + n.1, c.2 + n.2)))
        .count();
    }
    sum as u64
}

#[cfg(test)]
mod tests {
    use super::*;

    const DAY18_EXAMPLE: &str = "2,2,2
1,2,2
3,2,2
2,1,2
2,3,2
2,2,1
2,2,3
2,2,4
2,2,6
1,2,5
3,2,5
2,1,5
2,3,5";

    #[test]
    fn test_day18_part1() {
        let input = parse_input(DAY18_EXAMPLE);
        assert_eq!(find_surface_area(&input), 64);
    }
}
