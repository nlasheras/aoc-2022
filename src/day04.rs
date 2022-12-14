use aoc_runner_derive::aoc;
use aoc_runner_derive::aoc_generator;
use regex::Regex;
use std::cmp;
use std::ops::Range;

type Assignment = Range<u8>;

#[aoc_generator(day4)]
pub fn parse_input(input: &str) -> Vec<(Assignment, Assignment)> {
    let re = Regex::new(r"(\d+)-(\d+),(\d+)-(\d+)").unwrap();
    input
        .lines()
        .map(|s| {
            let error_msg = "Invalid input!";
            let numbers: Vec<u8> = re
                .captures(s)
                .expect(error_msg)
                .iter()
                .skip(1) // capture 0 is the whole matching pattern
                .map(|s| s.unwrap().as_str().parse::<u8>().expect(error_msg))
                .collect(); // get the 4 numbers as a Vec<u8>
            ((numbers[0]..numbers[1] + 1), (numbers[2]..numbers[3] + 1))
        })
        .collect()
}

fn overlap_range(pair: &(Assignment, Assignment)) -> Option<Assignment> {
    let start = cmp::max(pair.0.start, pair.1.start);
    let end = cmp::min(pair.0.end, pair.1.end);
    if start < end {
        return Some(start..end);
    }
    None
}

#[aoc(day4, part1)]
pub fn count_fully_contained(input: &[(Assignment, Assignment)]) -> usize {
    input
        .iter()
        .filter(|pair| {
            if let Some(overlap) = overlap_range(pair) {
                return overlap.len() == cmp::min(pair.0.len(), pair.1.len());
            }
            false
        })
        .count()
}

#[aoc(day4, part2)]
pub fn count_overlaps(input: &[(Assignment, Assignment)]) -> usize {
    input
        .iter()
        .map(overlap_range)
        .filter(|o| o.is_some())
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    const DAY04_EXAMPLE: &str = "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8";

    #[test]
    fn test_day4_part1() {
        let input = parse_input(DAY04_EXAMPLE);
        assert_eq!(count_fully_contained(&input), 2);
    }

    #[test]
    fn test_day4_part2() {
        let input = parse_input(DAY04_EXAMPLE);
        assert_eq!(count_overlaps(&input), 4);
    }
}
