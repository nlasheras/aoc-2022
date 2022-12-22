use crate::utils::Grid;
use aoc_runner_derive::aoc;
use aoc_runner_derive::aoc_generator;
use std::cmp;
use std::collections::HashMap;

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
    for movement in path.iter() {
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
    ((state.0 .1 + 1) * 1000 + (state.0 .0 + 1) * 4 + facing_value) as i64
}

struct CubeNet {
    pub size: (i32, i32),
    pub cells: Vec<i32>,
    connections: HashMap<(i32, (i32, i32)), (i32, (i32, i32))>,
}

impl CubeNet {
    pub fn example() -> CubeNet {
        let mut connections = HashMap::new();
        connections.insert((1, (0, -1)), (2, (0, 1)));
        connections.insert((1, (-1, 0)), (3, (0, 1)));
        connections.insert((1, (1, 0)), (6, (-1, 0)));

        connections.insert((2, (-1, 0)), (6, (0, -1)));
        connections.insert((2, (0, -1)), (1, (0, 1)));
        connections.insert((2, (0, 1)), (5, (-1, 0)));

        connections.insert((3, (0, -1)), (1, (1, 0)));
        connections.insert((3, (0, 1)), (5, (1, 0)));

        connections.insert((4, (1, 0)), (6, (0, 1)));

        connections.insert((5, (-1, 0)), (3, (0, -1)));
        connections.insert((5, (0, 1)), (2, (0, -1)));

        connections.insert((6, (1, 0)), (1, (-1, 0)));
        connections.insert((6, (0, -1)), (4, (-1, 0)));
        connections.insert((6, (0, 1)), (2, (1, 0)));

        CubeNet {
            size: (4, 4),
            cells: vec![0, 0, 1, 0, 2, 3, 4, 0, 0, 0, 5, 6, 0, 0, 0, 0],
            connections: connections,
        }
    }

    fn get_cube(&self, cube: i32) -> (i32, i32) {
        for y in 0..4 {
            for x in 0..4 {
                if self.cells[4 * y + x] == cube {
                    return (x as i32, y as i32);
                }
            }
        }
        panic!("Won't happen")
    }

    fn cube_at(&self, x: i32, y: i32) -> Option<i32> {
        let cube = self.cells.iter().nth((4 * y + x) as usize).unwrap();
        if *cube != 0 {
            return Some(*cube);
        }
        None
    }

    fn get_cube_with_facing(&self, x: i32, y: i32, facing: (i32, i32)) -> (i32, (i32, i32)) {
        let cube = self.cube_at(x, y).unwrap();
        *self.connections.get(&(cube, facing)).unwrap()
    }

    fn fold_pos(&self, pos: (i32, i32, i32), facing: (i32, i32)) -> (i32, i32, i32) {
        match facing {
            (1, 0) => (0, pos.0, pos.2),
            (-1, 0) => (self.size.0 - 1, self.size.1 - 1 - pos.0, pos.2),
            (0, 1) => (self.size.0 - 1 - pos.1, 0, pos.2),
            (0, -1) => (self.size.0 - 1 - pos.0, self.size.1 - 1, pos.2),
            _ => todo!(),
        }
    }

    fn move_to_other_face(
        &self,
        pos: (i32, i32, i32),
        facing: (i32, i32),
    ) -> ((i32, i32, i32), (i32, i32)) {
        let (cube_x, cube_y) = self.get_cube(pos.2);

        let mut new_pos = pos.clone();
        if new_pos.0 < 0 {
            new_pos.0 = self.size.0 - 1;
        } else if new_pos.0 >= self.size.0 {
            new_pos.0 = 0;
        }
        if new_pos.1 < 0 {
            new_pos.1 = self.size.1 - 1;
        }
        if new_pos.1 >= self.size.1 {
            new_pos.1 = 0
        }

        if let Some(cube) = self.cube_at(cube_x + facing.0, cube_y + facing.1) {
            new_pos.2 = cube;
            return (new_pos, facing);
        } else {
            let (cube, new_facing) = self.get_cube_with_facing(cube_x, cube_y, facing);
            new_pos.2 = cube;
            (self.fold_pos(new_pos, new_facing), new_facing)
        }
    }

    pub fn move_in_cube(
        &self,
        pos: (i32, i32, i32),
        facing: (i32, i32),
    ) -> ((i32, i32, i32), (i32, i32)) {
        let new_pos = (pos.0 + facing.0, pos.1 + facing.1, pos.2);
        if new_pos.0 >= 0 && new_pos.0 < self.size.0 && new_pos.1 >= 0 && new_pos.1 < self.size.1 {
            return (new_pos, facing); // moving inside the same face
        }

        self.move_to_other_face(new_pos, facing)
    }
}

fn move_with_cube(
    map: &Grid<char>,
    cube: &CubeNet,
    pos: (i32, i32, i32),
    n: u32,
    facing: (i32, i32),
) -> ((i32, i32, i32), (i32, i32)) {
    let mut new_pos = pos.clone();
    let mut new_facing = facing.clone();
    for _ in 0..n {
        let (candidate, candidate_facing) = cube.move_in_cube(new_pos, new_facing);
        let (cube_x, cube_y) = cube.get_cube(candidate.2);
        let c = map
            .cell_at(
                candidate.0 + cube_x * cube.size.0,
                candidate.1 + cube_y * cube.size.1,
            )
            .unwrap();
        if c == '#' {
            break;
        }
        new_pos = candidate;
        new_facing = candidate_facing;
    }
    (new_pos, new_facing)
}

#[aoc(day22, part2)]
pub fn get_password_with_cube(input: &Input) -> i64 {
    let (map, path) = input;
    let cube = CubeNet::example();
    let start_cube = 1; // todo!
    let mut state = ((0, 0, start_cube), (1, 0)); // facing right
    for movement in path.iter() {
        match movement {
            Move::Right | Move::Left => state.1 = rotate(state.1, movement),
            Move::Number(n) => state = move_with_cube(map, &cube, state.0, *n, state.1),
        }
    }
    let facing_value = match state.1 {
        (1, 0) => 0,
        (0, 1) => 1,
        (-1, 0) => 2,
        (0, -1) => 3,
        _ => panic!("Shouldn't happen"),
    };
    let (cube_x, cube_y) = cube.get_cube(state.0 .2);
    let col = state.0 .0 + 4 * cube_x;
    let row = state.0 .1 + 4 * cube_y;
    ((row + 1) * 1000 + (col + 1) * 4 + facing_value) as i64
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

    #[test]
    fn test_day22_cube_simple_move() {
        let cube = CubeNet::example();
        let (pos, facing) = cube.move_in_cube((3, 1, 3), (1, 0));
        assert_eq!(facing, (1, 0));
        assert_eq!(pos, (0, 1, 4));
    }

    #[test]
    fn test_day22_cube_example_ab() {
        let cube = CubeNet::example();
        let (pos, facing) = cube.move_in_cube((3, 1, 4), (1, 0));
        assert_eq!(facing, (0, 1));
        assert_eq!(pos, (2, 0, 6));
    }

    #[test]
    fn test_day22_cube_example_cd() {
        let cube = CubeNet::example();
        let (pos, facing) = cube.move_in_cube((2, 3, 5), (0, 1));
        assert_eq!(facing, (0, -1));
        assert_eq!(pos, (1, 3, 2));
    }

    #[test]
    fn test_day22_cube_example_end() {
        let cube = CubeNet::example();
        let (pos, facing) = cube.move_in_cube((2, 0, 3), (0, -1));
        assert_eq!(facing, (1, 0));
        assert_eq!(pos, (0, 2, 1));
    }

    #[test]
    fn test_day22_part2() {
        let input = parse_input(DAY22_EXAMPLE);
        assert_eq!(get_password_with_cube(&input), 5031);
    }
}
