use aoc_runner_derive::aoc;
use aoc_runner_derive::aoc_generator;

#[aoc_generator(day2)]
pub fn parse_input(input: &str) -> Vec<(char, char)> {
    input
        .lines()
        .map(|s| {
            let mut chars = s.chars();
            (chars.nth(0).unwrap(), chars.nth(1).unwrap()) // NOTE: nth consumes
        })
        .collect()
}

#[derive(PartialEq, Clone, Copy)]
pub enum Shape {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

impl Shape {
    fn from_i64(n: i64) -> Shape {
        return match n {
            1 => Shape::Rock,
            2 => Shape::Paper,
            3 => Shape::Scissors,
            _ => panic!("Unknown shape!"),
        };
    }

    pub fn beats_shape(&self) -> Shape {
        let as_num = *self as i64;
        let previous_shape = (as_num - 2).rem_euclid(3) + 1;
        Shape::from_i64(previous_shape)
    }

    pub fn is_beaten_by_shape(&self) -> Shape {
        let as_num = *self as i64;
        let next_shape = as_num.rem_euclid(3) + 1;
        Shape::from_i64(next_shape)
    }

    pub fn value(&self) -> u64 {
        *self as u64
    }
}

pub fn get_score(player: Shape, opponent: Shape) -> u64 {
    if player.beats_shape() == opponent {
        return 6;
    }
    if opponent.beats_shape() == player {
        return 0;
    }
    return 3;
}

pub fn decode_shape(c: char) -> Shape {
    return match c {
        'A' | 'X' => Shape::Rock,
        'B' | 'Y' => Shape::Paper,
        'C' | 'Z' => Shape::Scissors,
        _ => panic!("Unsupported char in strategy"),
    };
}

#[aoc(day2, part1)]
pub fn solve_part1(strategy: &Vec<(char, char)>) -> u64 {
    let mut score = 0;
    for (opponent_play, suggested_play) in strategy {
        let player_shape = decode_shape(*suggested_play);
        let opponent_shape = decode_shape(*opponent_play);
        score += get_score(player_shape, opponent_shape) + player_shape.value();
    }
    score
}

pub fn get_right_shape(score: u64, opponent: Shape) -> Shape {
    return match score {
        3 => opponent,                      // draw
        6 => opponent.is_beaten_by_shape(), // win
        0 => opponent.beats_shape(),        // lose
        _ => panic!("Unsupported outcome!"),
    };
}

pub fn decode_outcome(c: char) -> u64 {
    return match c {
        'X' => 0,
        'Y' => 3,
        'Z' => 6,
        _ => panic!("Unsupported outcome in strategy!"),
    };
}

#[aoc(day2, part2)]
pub fn solve_part2(strategy: &Vec<(char, char)>) -> u64 {
    let mut score = 0;
    for (opponent_play, outcome) in strategy {
        let oponent_shape = decode_shape(*opponent_play);
        let outcome_score = decode_outcome(*outcome);
        score += outcome_score + get_right_shape(outcome_score, oponent_shape).value();
    }
    score
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day2_part1() {
        let input: Vec<(char, char)> = vec![('A', 'Y'), ('B', 'X'), ('C', 'Z')];
        assert_eq!(solve_part1(&input), 15);
    }

    #[test]
    fn test_day2_part2() {
        let input: Vec<(char, char)> = vec![('A', 'Y'), ('B', 'X'), ('C', 'Z')];
        assert_eq!(solve_part2(&input), 12);
    }
}
