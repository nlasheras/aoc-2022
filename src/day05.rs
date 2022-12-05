use aoc_runner_derive::aoc;
use aoc_runner_derive::aoc_generator;
use regex::Regex;

type Stack = Vec<char>;
type Input = (Vec<Stack>, Vec<(u8, u8, u8)>);

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

    let moves: Vec<(u8, u8, u8)> = parts[1]
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
            (numbers[0], numbers[1] - 1, numbers[2] - 1)
        })
        .collect();

    (stacks, moves)
}

#[aoc(day5, part1)]
pub fn get_top_of_stacks(input: &Input) -> String {
    let mut stacks = input.0.clone();

    for (count, from, to) in input.1.iter() {
        for _ in 0..*count {
            let c = stacks
                .iter_mut()
                .nth(*from as usize)
                .unwrap()
                .pop()
                .unwrap();
            stacks.iter_mut().nth(*to as usize).unwrap().push(c)
        }
    }

    let mut ret = String::new();
    for stack in stacks.iter() {
        ret.push_str(&stack.last().unwrap().to_string());
    }
    ret.to_string()
}

#[aoc(day5, part2)]
pub fn get_top_of_stacks_move_multiple(input: &Input) -> String {
    let mut stacks = input.0.clone();

    for (count, from, to) in input.1.iter() {
        let mut tmp = Vec::new();
        for _ in 0..*count {
            let c = stacks
                .iter_mut()
                .nth(*from as usize)
                .unwrap()
                .pop()
                .unwrap();
            tmp.push(c)
        }
        stacks
            .iter_mut()
            .nth(*to as usize)
            .unwrap()
            .extend(tmp.into_iter().rev())
    }

    let mut ret = String::new();
    for stack in stacks.iter() {
        ret.push_str(&stack.last().unwrap().to_string());
    }
    ret.to_string()
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
        assert_eq!(get_top_of_stacks(&input), "CMZ");
    }

    #[test]
    fn test_day5_part2() {
        let input = parse_input(DAY05_EXAMPLE);
        assert_eq!(get_top_of_stacks_move_multiple(&input), "MCD");
    }
}
