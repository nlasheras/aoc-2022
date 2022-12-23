use crate::utils::Grid;
use crate::utils::Point;
use aoc_runner_derive::aoc;
use aoc_runner_derive::aoc_generator;
use std::cmp;
use std::collections::HashMap;
use std::ops::Rem;

#[derive(Debug)]
pub enum Move {
    Number(u32),
    Right,
    Left,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Facing {
    Right = 0,
    Down = 1,
    Left = 2,
    Up = 3,
}

impl Facing {
    fn from_i8(n: i8) -> Facing {
        match n {
            0 => Facing::Right,
            1 => Facing::Down,
            2 => Facing::Left,
            3 => Facing::Up,
            _ => panic!("Wrong facing!"),
        }
    }

    fn rotate_cw(&self) -> Facing {
        let value = *self as i8;
        return Facing::from_i8((value + 1).rem(4));
    }

    fn rotate_ccw(&self) -> Facing {
        let value = *self as i8;
        return Facing::from_i8((value - 1).rem_euclid(4));
    }

    fn as_vector(&self) -> Point {
        match self {
            Facing::Right => Point::new(1, 0),
            Facing::Down => Point::new(0, 1),
            Facing::Left => Point::new(-1, 0),
            Facing::Up => Point::new(0, -1),
        }
    }
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

trait WrapLogic {
    fn cell_at(&self, pos: &Point) -> Option<char>;
    fn move_point(&self, pos: &Point, facing: &Facing) -> Point;
}

pub struct SimpleWraparound<'a> {
    map: &'a Grid<char>,
}

impl SimpleWraparound<'_> {
    fn new(map: &Grid<char>) -> SimpleWraparound {
        SimpleWraparound { map: map }
    }

    fn wraparound(&self, pos: &Point, facing: &Facing) -> Point {
        let dir = facing.as_vector();
        let mut wrap = *pos - dir;
        while let Some(c) = self.map.cell_at(wrap.x, wrap.y) {
            if c == ' ' {
                break;
            }
            wrap = wrap - dir;
        }
        wrap + dir
    }
}

impl WrapLogic for SimpleWraparound<'_> {
    fn cell_at(&self, pos: &Point) -> Option<char> {
        self.map.cell_at(pos.x, pos.y)
    }

    fn move_point(&self, pos: &Point, facing: &Facing) -> Point {
        let mut candidate = *pos + facing.as_vector();
        let (width, height) = self.map.size();

        if candidate.x < 0
            || candidate.x >= width as i32
            || candidate.y < 0
            || candidate.y >= height as i32
        {
            candidate = self.wraparound(&candidate, facing);
        }

        let c = self.map.cell_at(candidate.x, candidate.y).unwrap();
        if c == ' ' {
            candidate = self.wraparound(&candidate, facing);
        }
        candidate
    }
}

fn move_in_map(map: &dyn WrapLogic, pos: &Point, facing: &Facing) -> Point {
    let candidate = map.move_point(pos, facing);
    let c = map.cell_at(&candidate).unwrap();
    if c == '#' {
        return *pos;
    }
    candidate
}

fn follow_path(
    map: &dyn WrapLogic,
    path: &Vec<Move>,
    start: &Point,
    facing: &Facing,
) -> (Point, Facing) {
    let mut state = (start.clone(), facing.clone());
    for movement in path.iter() {
        match movement {
            Move::Right => state.1 = state.1.rotate_cw(),
            Move::Left => state.1 = state.1.rotate_ccw(),
            Move::Number(n) => {
                for _ in 0..*n {
                    state.0 = move_in_map(map, &state.0, &state.1)
                }
            }
        }
    }
    state
}

#[aoc(day22, part1)]
pub fn get_password(input: &Input) -> i64 {
    let (map, path) = input;
    let wrapping = SimpleWraparound::new(map);
    let start_x = map.cells.iter().position(|c| *c == '.').unwrap() as i32;
    let end = follow_path(&wrapping, path, &Point::new(start_x, 0), &Facing::Right);
    let facing_value = end.1 as i32;
    ((end.0.y + 1) * 1000 + (end.0.x + 1) * 4 + facing_value) as i64
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

    pub fn input() -> CubeNet {
        let mut connections = HashMap::new();
        connections.insert((1, (0, -1)), (4, (1, 0)));
        connections.insert((1, (-1, 0)), (6, (1, 0)));

        connections.insert((2, (0, -1)), (6, (0, -1)));
        connections.insert((2, (1, 0)), (5, (-1, 0)));
        connections.insert((2, (0, 1)), (3, (-1, 0)));

        connections.insert((3, (1, 0)), (2, (0, -1)));
        connections.insert((3, (-1, 0)), (4, (0, 1)));

        connections.insert((4, (0, -1)), (3, (1, 0)));
        connections.insert((4, (-1, 0)), (1, (1, 0)));

        connections.insert((5, (1, 0)), (2, (-1, 0)));
        connections.insert((5, (0, 1)), (6, (-1, 0)));

        connections.insert((6, (1, 0)), (5, (0, -1)));
        connections.insert((6, (-1, 0)), (1, (0, 1)));
        connections.insert((6, (0, 1)), (2, (0, 1)));

        CubeNet {
            size: (50, 50),
            cells: vec![0, 1, 2, 0, 0, 3, 0, 0, 4, 5, 0, 0, 6, 0, 0, 0],
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
        println!("get_cube_with_facing({}, {:?})", cube, facing);
        let (a, b) = *self.connections.get(&(cube, facing)).unwrap();
        println!("    {:?} {:?}", a, b);
        (a, b)
    }

    fn fold_pos(&self, pos: (i32, i32, i32), facing: (i32, i32)) -> (i32, i32, i32) {
        let p = match facing {
            (1, 0) => (0, pos.0, pos.2),
            (-1, 0) => (self.size.0 - 1, self.size.1 - 1 - pos.0, pos.2),
            (0, 1) => (self.size.0 - 1 - pos.1, 0, pos.2),
            (0, -1) => (self.size.0 - 1 - pos.0, self.size.1 - 1, pos.2),
            _ => todo!(),
        };
        println!("  fold pos({:?})= {:?}", pos, p);
        p
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
    let cube = if map.size().0 == 150 {
        CubeNet::input()
    } else {
        CubeNet::example()
    };
    let start_cube = 1; // todo!
    let mut state = ((0, 0, start_cube), (1, 0)); // facing right
    for movement in path.iter() {
        println!("Move {:?} at {:?}", movement, state);
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

    #[test]
    fn test_day22_facing_cw() {
        let start = Facing::Right;
        let cw1 = start.rotate_cw();
        assert_eq!(cw1, Facing::Down);
        let cw2 = cw1.rotate_cw();
        assert_eq!(cw2, Facing::Left);
        let cw3 = cw2.rotate_cw();
        assert_eq!(cw3, Facing::Up);
        let cw4 = cw3.rotate_cw();
        assert_eq!(cw4, Facing::Right);
    }

    #[test]
    fn test_day22_facing_ccw() {
        let start = Facing::Right;
        let ccw1 = start.rotate_ccw();
        assert_eq!(ccw1, Facing::Up);
        let ccw2 = ccw1.rotate_ccw();
        assert_eq!(ccw2, Facing::Left);
        let ccw3 = ccw2.rotate_ccw();
        assert_eq!(ccw3, Facing::Down);
        let ccw4 = ccw3.rotate_ccw();
        assert_eq!(ccw4, Facing::Right);
    }
}
