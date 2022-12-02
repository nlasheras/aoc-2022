use aoc_runner_derive::aoc;
use aoc_runner_derive::aoc_generator;

#[aoc_generator(day2)]
pub fn parse_input(input: &str) -> Vec<(char, char)> {
    input.lines().map(|s| {
        let mut chars = s.chars();
        (chars.nth(0).unwrap(), chars.nth(1).unwrap()) // NOTE: nth consumes
    }).collect()
}

#[derive(PartialEq, Clone, Copy)]
pub enum Shape {
    Rock = 1,
    Paper = 2,
    Scissors = 3
}

pub fn beats(this: Shape, other: Shape) -> bool {
    (this == Shape::Rock && other == Shape::Scissors) ||
    (this == Shape::Paper && other == Shape::Rock) ||
    (this == Shape::Scissors && other == Shape::Paper)
}

pub fn get_score(you: Shape, oponent: Shape) -> u64
{
    if beats(you, oponent) { 
        return 6
    }
    if beats(oponent, you) {
        return 0
    }
    return 3
}

pub fn get_shape(c: char) -> Shape
{
    if c == 'A' || c == 'X' { return Shape::Rock } 
    if c == 'B' || c == 'Y' { return Shape::Paper } 
    return Shape::Scissors
}

#[aoc(day2, part1)]
pub fn solve_part1(strategy: &Vec<(char, char)>) -> u64 {
    let mut score = 0;
    for (coponent, cyou) in strategy {
        let you = get_shape(*cyou);
        let oponent = get_shape(*coponent);
        score += get_score(you, oponent) + you as u64;
    }
    score
}

pub fn get_right_shape(score: u64, oponent: Shape) -> Shape {
    if score == 3 { return oponent }
    if score == 6 {  // win
        return match oponent {
            Shape::Rock => Shape::Paper,
            Shape::Scissors => Shape::Rock,
            Shape::Paper => Shape::Scissors
        }
    }
    else {  // lose
        return match oponent {
            Shape::Paper => Shape::Rock,
            Shape::Rock => Shape::Scissors,
            Shape::Scissors => Shape::Paper
        }
    }
}

pub fn letter_into_outcome(c: char) -> u64 {
    if c == 'Y' { return 3 }
    if c == 'Z' { return 6 }
    return 0
}

#[aoc(day2, part2)]
pub fn solve_part2(strategy: &Vec<(char, char)>) -> u64 {
    let mut score = 0;
    for (coponent, coutcome) in strategy {
        let oponent = get_shape(*coponent);
        let outcome = letter_into_outcome(*coutcome);
        score += outcome + get_right_shape(outcome, oponent) as u64;
    }
    score
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day1_part1() {
        let input: Vec<(char, char)> = vec![
            ('A', 'Y'), ('B', 'X'), ('C', 'Z')
        ];
        assert_eq!(solve_part1(&input), 15);
    }

    #[test]
    fn test_day1_part2() {
        let input: Vec<(char, char)> = vec![
            ('A', 'Y'), ('B', 'X'), ('C', 'Z')
        ];
        assert_eq!(solve_part2(&input), 12);
    }
    
}
