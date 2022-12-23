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

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
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

trait WrapLogic {
    fn cell_at(&self, pos: &Point) -> Option<char>;
    fn move_point(&self, pos: &Point, facing: &Facing) -> (Point, Facing);
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

    fn move_point(&self, pos: &Point, facing: &Facing) -> (Point, Facing) {
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
        (candidate, *facing)
    }
}

fn move_in_map(map: &dyn WrapLogic, pos: &Point, facing: &Facing) -> (Point, Facing) {
    let (new_pos, new_facing) = map.move_point(pos, facing);
    let c = map.cell_at(&new_pos).unwrap();
    if c == '#' {
        return (*pos, *facing);
    }
    (new_pos, new_facing)
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
                    state = move_in_map(map, &state.0, &state.1)
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

struct CubeNet<'a> {
    pub size: (i32, i32),
    pub cells: Vec<i32>,
    connections: HashMap<(i32, Facing), (i32, Facing)>,
    pub map: &'a Grid<char>,
}

impl CubeNet<'_> {
    pub fn example(map: &Grid<char>) -> CubeNet {
        let mut connections = HashMap::new();
        connections.insert((1, Facing::Up), (2, Facing::Down));
        connections.insert((1, Facing::Left), (3, Facing::Down));
        connections.insert((1, Facing::Right), (6, Facing::Left));

        connections.insert((2, Facing::Left), (6, Facing::Up));
        connections.insert((2, Facing::Up), (1, Facing::Down));
        connections.insert((2, Facing::Down), (5, Facing::Left));

        connections.insert((3, Facing::Up), (1, Facing::Right));
        connections.insert((3, Facing::Down), (5, Facing::Right));

        connections.insert((4, Facing::Right), (6, Facing::Down));

        connections.insert((5, Facing::Left), (3, Facing::Up));
        connections.insert((5, Facing::Down), (2, Facing::Up));

        connections.insert((6, Facing::Right), (1, Facing::Left));
        connections.insert((6, Facing::Up), (4, Facing::Left));
        connections.insert((6, Facing::Down), (2, Facing::Right));

        CubeNet {
            size: (4, 4),
            cells: vec![0, 0, 1, 0, 2, 3, 4, 0, 0, 0, 5, 6, 0, 0, 0, 0],
            connections: connections,
            map: map,
        }
    }

    pub fn input(map: &Grid<char>) -> CubeNet {
        let mut connections = HashMap::new();
        connections.insert((1, Facing::Up), (4, Facing::Right));
        connections.insert((1, Facing::Left), (6, Facing::Right));

        connections.insert((2, Facing::Up), (6, Facing::Up));
        connections.insert((2, Facing::Right), (5, Facing::Left));
        connections.insert((2, Facing::Down), (3, Facing::Left));

        connections.insert((3, Facing::Right), (2, Facing::Up));
        connections.insert((3, Facing::Left), (4, Facing::Down));

        connections.insert((4, Facing::Up), (3, Facing::Right));
        connections.insert((4, Facing::Left), (1, Facing::Right));

        connections.insert((5, Facing::Right), (2, Facing::Left));
        connections.insert((5, Facing::Down), (6, Facing::Left));

        connections.insert((6, Facing::Right), (5, Facing::Up));
        connections.insert((6, Facing::Left), (1, Facing::Down));
        connections.insert((6, Facing::Down), (2, Facing::Down));

        CubeNet {
            size: (50, 50),
            cells: vec![0, 1, 2, 0, 0, 3, 0, 0, 4, 5, 0, 0, 6, 0, 0, 0],
            connections: connections,
            map: map,
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

    fn get_cube_with_facing(&self, x: i32, y: i32, facing: &Facing) -> (i32, Facing) {
        let cube = self.cube_at(x, y).unwrap();
        println!("get_cube_with_facing({}, {:?})", cube, facing);
        let (a, b) = *self.connections.get(&(cube, *facing)).unwrap();
        println!("    {:?} {:?}", a, b);
        (a, b)
    }

    fn fold_pos(&self, pos: &Point, facing: &Facing) -> Point {
        let p = match facing {
            Facing::Right => Point::new_3d(0, pos.x, pos.z),
            Facing::Left => Point::new_3d(self.size.0 - 1, self.size.1 - 1 - pos.x, pos.z),
            Facing::Down => Point::new_3d(self.size.0 - 1 - pos.y, 0, pos.z),
            Facing::Up => Point::new_3d(self.size.0 - 1 - pos.x, self.size.1 - 1, pos.z),
        };
        println!("  fold pos({:?})= {:?}", pos, p);
        p
    }

    fn move_to_other_face(&self, pos: &Point, facing: &Facing) -> (Point, Facing) {
        let (cube_x, cube_y) = self.get_cube(pos.z);

        let mut new_pos = pos.clone();
        if new_pos.x < 0 {
            new_pos.x = self.size.0 - 1;
        } else if new_pos.x >= self.size.0 {
            new_pos.x = 0;
        }
        if new_pos.y < 0 {
            new_pos.y = self.size.1 - 1;
        }
        if new_pos.y >= self.size.1 {
            new_pos.y = 0
        }

        let dir = facing.as_vector();
        if let Some(cube) = self.cube_at(cube_x + dir.x, cube_y + dir.y) {
            new_pos.z = cube;
            return (new_pos, *facing);
        } else {
            let (cube, new_facing) = self.get_cube_with_facing(cube_x, cube_y, facing);
            new_pos.z = cube;
            (self.fold_pos(&new_pos, &new_facing), new_facing)
        }
    }
}

impl WrapLogic for CubeNet<'_> {
    fn cell_at(&self, pos: &Point) -> Option<char> {
        let col = pos.x + 4 * self.size.0;
        let row = pos.y + 4 * self.size.1;
        self.map.cell_at(col, row)
    }

    fn move_point(&self, pos: &Point, facing: &Facing) -> (Point, Facing) {
        let new_pos = *pos + facing.as_vector();
        if new_pos.x >= 0 && new_pos.x < self.size.0 && new_pos.y >= 0 && new_pos.y < self.size.1 {
            return (new_pos, *facing); // moving inside the same face
        }

        self.move_to_other_face(&new_pos, facing)
    }
}

#[aoc(day22, part2)]
pub fn get_password_with_cube(input: &Input) -> i64 {
    let (map, path) = input;
    let cube = if map.size().0 == 150 {
        CubeNet::input(map)
    } else {
        CubeNet::example(map)
    };
    let start_cube = 1; // todo!
    let start = (Point::new_3d(0, 0, start_cube), Facing::Right);
    let end = follow_path(&cube, path, &start.0, &start.1);

    let facing_value = end.1 as i32;
    let (cube_x, cube_y) = cube.get_cube(end.0.z);
    let col = end.0.x + 4 * cube_x;
    let row = end.0.y + 4 * cube_y;
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
        let input = parse_input(DAY22_EXAMPLE);
        let cube = CubeNet::example(&input.0);
        let (pos, facing) = cube.move_point(&Point::new_3d(3, 1, 3), &Facing::Right);
        assert_eq!(facing, Facing::Right);
        assert_eq!(pos, Point::new_3d(0, 1, 4));
    }

    #[test]
    fn test_day22_cube_example_ab() {
        let input = parse_input(DAY22_EXAMPLE);
        let cube = CubeNet::example(&input.0);
        let (pos, facing) = cube.move_point(&Point::new_3d(3, 1, 4), &Facing::Right);
        assert_eq!(facing, Facing::Down);
        assert_eq!(pos, Point::new_3d(2, 0, 6));
    }

    #[test]
    fn test_day22_cube_example_cd() {
        let input = parse_input(DAY22_EXAMPLE);
        let cube = CubeNet::example(&input.0);
        let (pos, facing) = cube.move_point(&Point::new_3d(2, 3, 5), &Facing::Down);
        assert_eq!(facing, Facing::Up);
        assert_eq!(pos, Point::new_3d(1, 3, 2));
    }

    #[test]
    fn test_day22_cube_example_end() {
        let input = parse_input(DAY22_EXAMPLE);
        let cube = CubeNet::example(&input.0);
        let (pos, facing) = cube.move_point(&Point::new_3d(2, 0, 3), &Facing::Up);
        assert_eq!(facing, Facing::Right);
        assert_eq!(pos, Point::new_3d(0, 2, 1));
    }

    #[ignore]
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
