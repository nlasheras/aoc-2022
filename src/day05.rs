use aoc_runner_derive::aoc;
use aoc_runner_derive::aoc_generator;
use regex::Regex;

type Stack = String;
type Input = (Vec<Stack>, Vec<(usize, usize, usize)>);

#[aoc_generator(day5)]
pub fn parse_input(input: &str) -> Input {
    let parts: Vec<&str> = input.split("\n\n").collect();

    let stack_input = parts[0];
    let count = (stack_input.lines().nth(0).unwrap().len() + 2) / 4;
    let mut stacks = vec![Stack::new(); count];
    for line in stack_input.lines().rev().skip(1) {
        for i in 0..count {
            let c = line.chars().nth(i * 4 + 1).unwrap();
            if c != ' ' {
                stacks[i].push(c);
            }
        }
    }

    let re = Regex::new(r"move (\d+) from (\d+) to (\d+)").unwrap();

    let moves: Vec<(usize, usize, usize)> = parts[1]
        .lines()
        .map(|s| {
            let error_msg = "Invalid input!";
            let numbers: Vec<usize> = re
                .captures(s)
                .expect(error_msg)
                .iter()
                .skip(1) // capture 0 is the whole matching pattern
                .map(|s| s.unwrap().as_str().parse::<usize>().expect(error_msg))
                .collect(); // get the 4 numbers as a Vec<usize>
            (numbers[0], numbers[1] - 1, numbers[2] - 1)
        })
        .collect();

    (stacks, moves)
}

fn move_crates_from_stack(stacks: &mut Vec<Stack>, from: usize, to: usize, count: usize) {
    let source = stacks.iter_mut().nth(from).unwrap();
    let moved_crates = source.split_off(source.len() - count);
    stacks
        .iter_mut()
        .nth(to)
        .unwrap()
        .extend(moved_crates.chars().into_iter());
}

fn get_top_of_stacks(stacks: &Vec<Stack>) -> String {
    let mut ret = String::new();
    for stack in stacks.iter() {
        ret.push_str(&stack.chars().into_iter().last().unwrap().to_string());
    }
    ret.to_string()
}

#[aoc(day5, part1)]
pub fn get_top_after_move(input: &Input) -> String {
    let mut stacks = input.0.clone();

    for (count, from, to) in input.1.iter() {
        for _ in 0..*count {
            move_crates_from_stack(&mut stacks, *from, *to, 1);
        }
    }

    get_top_of_stacks(&stacks)
}

#[aoc(day5, part2)]
pub fn get_top_after_move_with_9001(input: &Input) -> String {
    let mut stacks = input.0.clone();

    for (count, from, to) in input.1.iter() {
        move_crates_from_stack(&mut stacks, *from, *to, *count)
    }

    get_top_of_stacks(&stacks)
}

#[cfg(test)]
mod tests {
    use super::*;

    const DAY05_EXAMPLE: &str = "    [D]     
[N] [C]     
[Z] [M] [P] 
 1   2   3  

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2";

    #[test]
    fn test_day5_part1() {
        let input = parse_input(DAY05_EXAMPLE);
        assert_eq!(get_top_after_move(&input), "CMZ");
    }

    #[test]
    fn test_day5_part2() {
        let input = parse_input(DAY05_EXAMPLE);
        assert_eq!(get_top_after_move_with_9001(&input), "MCD");
    }
}
