use aoc_runner_derive::aoc;
use aoc_runner_derive::aoc_generator;

#[aoc_generator(day1)]
pub fn parse_input(input: &str) -> Vec<u64> {
    let mut ret : Vec<u64> = input.lines().map(|x| if x.is_empty() { 0 } else { x.parse().unwrap() }).collect();
    ret.push(0);
    ret
}

pub fn count_calories(entries: &[u64]) -> Vec<u64> {
    let mut sums = Vec::new();
    let mut sum = 0;
    for entry in entries {
        if *entry == 0 {
            sums.push(sum);
            sum = 0;
        }
        else {
            sum += entry;
        }
    }
    sums
}

#[aoc(day1, part1)]
pub fn solve_part1(entries: &[u64]) -> u64 {
    let mut calories = count_calories(entries);
    calories.sort();
    *calories.last().expect("Entries are empty!")
}

#[aoc(day1, part2)]
pub fn solve_part2(entries: &[u64]) -> u64 {
    let mut calories = count_calories(entries);
    calories.sort();
    calories[calories.len()-3..calories.len()].iter().sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day1_part1() {
        let input: Vec<u64> = vec![1000, 2000, 3000, 0, 4000, 0, 5000, 6000, 0, 7000, 8000, 9000, 0, 10000, 0];
        assert_eq!(solve_part1(&input), 24000);
    }

    #[test]
    fn test_day1_part2() {
        let input: Vec<u64> = vec![1000, 2000, 3000, 0, 4000, 0, 5000, 6000, 0, 7000, 8000, 9000, 0, 10000, 0];
        assert_eq!(solve_part2(&input), 45000);
    }

}