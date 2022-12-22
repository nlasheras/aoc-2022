use crate::utils::Grid;
use aoc_runner_derive::aoc;
use aoc_runner_derive::aoc_generator;
use std::cmp;

#[derive(Debug)]
pub enum Move {
    Number(u32),
    Right,
    Left,
}

type Input = (Grid<char>, Vec<Move>);

fn parse_map(input: &str) -> Grid<char> {
    let lines = input
        .lines()
        .map(|s| s.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();
    let height = lines.len();
    let width = lines.iter().fold(0, |max, l| cmp::max(max, l.len()));
    let mut cells = vec![' '; width * height];
    let mut y = 0;
    for row in lines {
        let mut x = 0;
        for col in row {
            cells[y * width + x] = col;
            x += 1
        }
        y += 1;
    }
    println!("grid is {}x{}", width, height);
    Grid::new(&cells, width)
}

fn parse_path(input: &str) -> Vec<Move> {
    assert!(input.chars().nth(0).unwrap().is_numeric());
    let mut path = Vec::new();
    let mut number = 0;
    for c in input.chars() {
        if c.is_numeric() {
            number = number * 10 + c.to_digit(10).unwrap();
        } else {
            path.push(Move::Number(number));
            number = 0;

            match c {
                'R' => path.push(Move::Right),
                'L' => path.push(Move::Left),
                _ => panic!("Shouldn't happen"),
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

fn rotate(facing: (i32, i32), direction: &Move) -> (i32, i32) {
    match direction {
        Move::Right => (-facing.1, facing.0),
        Move::Left => (facing.1, -facing.0),
        _ => panic!("Wrong facing"),
    }
}

fn wraparound(map: &Grid<char>, pos: (i32, i32), facing: (i32, i32)) -> (i32, i32) {
    let back = (-facing.0, -facing.1);
    let mut wrap = (pos.0 + back.0, pos.1 + back.1);
    while let Some(c) = map.cell_at(wrap.0, wrap.1) {
        if c == ' ' {
            break;
        }
        wrap = (wrap.0 + back.0, wrap.1 + back.1)
    }
    (wrap.0 + facing.0, wrap.1 + facing.1)
}

fn move_in_map(map: &Grid<char>, pos: (i32, i32), n: u32, facing: (i32, i32)) -> (i32, i32) {
    let mut new_pos = pos.clone();
    let (width, height) = map.size();
    for _ in 0..n {
        let mut candidate = (new_pos.0 + facing.0, new_pos.1 + facing.1);
        if candidate.0 == -1
            || candidate.0 == width as i32
            || candidate.1 == -1
            || candidate.1 == height as i32
        {
            candidate = wraparound(map, candidate, facing);
        }
        let mut c = map.cell_at(candidate.0, candidate.1).unwrap();
        if c == ' ' {
            candidate = wraparound(map, candidate, facing);
            c = map.cell_at(candidate.0, candidate.1).unwrap();
        }
        if c == '#' {
            return new_pos;
        }
        new_pos = candidate
    }
    new_pos
}

#[aoc(day22, part1)]
pub fn get_password(input: &Input) -> i64 {
    let (map, path) = input;
    let start_x = map.cells.iter().position(|c| *c == '.').unwrap() as i32;
    let mut state = ((start_x, 0), (1, 0)); // facing right
    println!("path len {} start: {:?}", path.len(), state.0);
    for movement in path.iter() {
        println!("{:?}", movement);
        match movement {
            Move::Right | Move::Left => state.1 = rotate(state.1, movement),
            Move::Number(n) => state.0 = move_in_map(map, state.0, *n, state.1),
        }
    }
    let facing_value = match state.1 {
        (1, 0) => 0,
        (0, 1) => 1,
        (-1, 0) => 2,
        (0, -1) => 3,
        _ => panic!("Shouldn't happen"),
    };
    println!("pos:{:?} facing:{:?}", state.0, state.1);
    ((state.0 .1 + 1) * 1000 + (state.0 .0 + 1) * 4 + facing_value) as i64
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
