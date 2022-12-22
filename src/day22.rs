use aoc_runner_derive::aoc;
use aoc_runner_derive::aoc_generator;
use crate::utils::Grid;
use std::cmp;

pub enum Move
{
    Number(u32),
    Right,
    Left
}

type Input = (Grid<char>, Vec<Move>);

fn parse_map(input: &str) -> Grid<char> {
    let lines = input.lines().map(|s| s.chars().collect::<Vec<char>>()).collect::<Vec<Vec<char>>>();
    let height = lines.len();
    let width = lines.iter().fold(0, |max, l| cmp::max(max, l.len()));
    let mut cells = vec![' ';width*height];
    let mut y = 0;
    for row in lines {
        let mut x = 0;
        for col in row {
            cells[y*width + x] = col;
            x += 1
        }
        y += 1;
    }
    Grid::new(&cells, width)
}

fn parse_path(input: &str) -> Vec<Move> {
    assert!(input.chars().nth(0).unwrap().is_numeric());
    let mut path = Vec::new();
    let mut number = 0;
    for c in input.chars() {
        if c.is_numeric() {
            number = number*10 + c.to_digit(10).unwrap();
        }
        else {
            path.push(Move::Number(number));
            number = 0;

            match c {
                'R' => path.push(Move::Right),
                'L' => path.push(Move::Left),
                _ => panic!("Shouldn't happen")
            }
        }
    }
    if number != 0 {
        path.push(Move::Number(number));
    }
    path
}

#[aoc_generator(day22)]
pub fn parse_input(input: &str) -> Input {
    let parts = input.split("\n\n").collect::<Vec<&str>>();
    (parse_map(parts[0]), parse_path(parts[1]))
}

#[aoc(day22, part1)]
pub fn get_password(input: &Input) -> i64 {
    input.1.len() as i64
}

#[cfg(test)]
mod tests {
    use super::*;

    const DAY22_EXAMPLE: &str = "        ...#
    .#..
    #...
    ....
...#.......#
........#...
..#....#....
..........#.
    ...#....
    .....#..
    .#......
    ......#.

10R5L5R10L4R5L5";

    #[test]
    fn test_day22_part1() {
        let input = parse_input(DAY22_EXAMPLE);
        assert_eq!(get_password(&input), 6032);
    }
}
