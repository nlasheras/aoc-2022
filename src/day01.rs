use aoc_runner_derive::aoc;
use aoc_runner_derive::aoc_generator;

#[aoc_generator(day1)]
pub fn parse_input(input: &str) -> Vec<u64> {
    let mut ret: Vec<u64> = input
        .lines()
        .map(|x| if x.is_empty() { 0 } else { x.parse().unwrap() })
        .collect();
    ret.push(0); // make sure there is a elf "termination" to simplify the implementation
    ret
}

pub fn count_calories(entries: &[u64]) -> Vec<u64> {
    let mut sums = Vec::new();
    let mut elf_start = 0;
    for i in 1..entries.len() {
        if entries[i] == 0 {
            sums.push(entries[elf_start..i].iter().sum());
            elf_start = i + 1;
        }
    }
    sums
}

#[aoc(day1, part1)]
pub fn solve_part1(entries: &[u64]) -> u64 {
    let mut calories = count_calories(entries);
    calories.sort();
    *calories.last().expect("There is at least one elf")
}

#[aoc(day1, part2)]
pub fn solve_part2(entries: &[u64]) -> u64 {
    let mut calories = count_calories(entries);
    calories.sort_by_key(|&n| std::cmp::Reverse(n)); // sort descent
    calories[0..3].iter().sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day1_part1() {
        let input: Vec<u64> = vec![
            1000, 2000, 3000, 0, 4000, 0, 5000, 6000, 0, 7000, 8000, 9000, 0, 10000, 0,
        ];
        assert_eq!(solve_part1(&input), 24000);
    }

    #[test]
    fn test_day1_part2() {
        let input: Vec<u64> = vec![
            1000, 2000, 3000, 0, 4000, 0, 5000, 6000, 0, 7000, 8000, 9000, 0, 10000, 0,
        ];
        assert_eq!(solve_part2(&input), 45000);
    }
}
