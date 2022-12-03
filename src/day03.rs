use aoc_runner_derive::aoc;
use aoc_runner_derive::aoc_generator;

fn string_to_rucksack(s: &str) -> Vec<char> {
    s.chars().collect()
}

#[aoc_generator(day3)]
pub fn parse_input(input: &str) -> Vec<Vec<char>> {
    input.lines().map(string_to_rucksack).collect()
}

fn intersection(set: &[char], other: &[char]) -> Vec<char> {
    set.iter().cloned().filter(|item| other.contains(item)).collect()
}

fn get_failing_item(sack: &Vec<char>) -> char {
    let len = sack.len();
    let same = intersection(&sack[0..len/2], &sack[len/2..len]);
    same[0]
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
    input.into_iter().fold(0, |sum, sack| {
       let item = get_failing_item(sack);
       sum + get_item_priority(item) as u64  
    })
}

fn get_badge(input: &[Vec<char>]) -> char {
    // the shared element between all the sets
    input.iter().cloned().reduce(|set, other| {
        intersection(&set, &other)
    }).unwrap()[0]
}

#[aoc(day3, part2)]
pub fn sum_badge_priorities(input: &Vec<Vec<char>>) -> u64 {
    input.chunks(3).fold(0, |sum, sacks| {
        let item = get_badge(sacks);
        sum + get_item_priority(item) as u64
    })
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