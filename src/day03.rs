use aoc_runner_derive::aoc;
use aoc_runner_derive::aoc_generator;
use std::collections::BTreeSet;

fn string_to_rucksack(s: &str) -> Vec<char> {
    s.chars().collect()
}

#[aoc_generator(day3)]
pub fn parse_input(input: &str) -> Vec<Vec<char>> {
    input.lines().map(string_to_rucksack).collect()
}

fn get_failing_item(sack: &Vec<char>) -> char {
    let len = sack.len();
    let left = BTreeSet::from_iter(sack[0..len/2].iter());
    let right = BTreeSet::from_iter(sack[len/2..len].iter());    
    let mut same = left.intersection(&right);
    **same.next().expect("There should be 1 wrong item")
}

fn get_item_priority(c: char) -> u8 {
    return match c {
        'a'..='z' => 1 + (c as u8) - ('a' as u8),
        'A'..='Z' => 27 + (c as u8) - ('A' as u8),
        _ => panic!("Unsupported item!")
    }    
}

#[aoc(day3, part1)]
pub fn sum_mismatched_priorities(input: &Vec<Vec<char>>) -> u64 {
    let mut sum = 0;
    for sack in input {
        let item = get_failing_item(&sack);
        sum += get_item_priority(item) as u64;
    }
    sum
}

fn get_badge(input: &[Vec<char>]) -> char {
    let mut sacks = input.iter();
    let mut set = BTreeSet::from_iter(sacks.next().unwrap().iter());
    for sack in sacks {
        let current = BTreeSet::from_iter(sack.iter());
        let same = set.intersection(&current);
        set = BTreeSet::from_iter(same.map(|r| *r).collect::<Vec<&char>>());
    }
    **set.iter().next().expect("There should be 1 shared item")
}

#[aoc(day3, part2)]
pub fn sum_badge_priorities(input: &Vec<Vec<char>>) -> u64 {
    let mut sum = 0;
    for sacks in input.chunks(3) {
        let item = get_badge(sacks);
        sum += get_item_priority(item) as u64;
    }
    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day3_get_failing_item() {
        let input = string_to_rucksack("vJrwpWtwJgWrhcsFMMfFFhFp");
        assert_eq!(get_failing_item(&input), 'p');
    }
        
    const DAY03_EXAMPLE : &str = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";
    
    #[test]
    fn test_day3_part1() {
        let input = parse_input(DAY03_EXAMPLE);
        assert_eq!(sum_mismatched_priorities(&input), 157);
    }

    #[test]
    fn test_day3_part2() {
        let input = parse_input(DAY03_EXAMPLE);
        assert_eq!(sum_badge_priorities(&input), 70);
    }
}