use crate::utils::Point;
use aoc_runner_derive::aoc;
use aoc_runner_derive::aoc_generator;
use std::cmp;

#[aoc_generator(day17)]
pub fn parse_input(input: &str) -> Vec<char> {
    input.chars().filter(|c| *c == '>' || *c == '<').collect()
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum Shape {
    Flat,
    Plus,
    L,
    Tall,
    Square,
}

impl Shape {
    pub fn next(&self) -> Shape {
        match self {
            Shape::Flat => Shape::Plus,
            Shape::Plus => Shape::L,
            Shape::L => Shape::Tall,
            Shape::Tall => Shape::Square,
            Shape::Square => Shape::Flat,
        }
    }

    pub fn points(&self) -> Vec<Point> {
        match self {
            Shape::Flat => vec![(0, 0), (1, 0), (2, 0), (3, 0)]
                .iter()
                .map(|p| Point::new(p.0, p.1))
                .collect(),
            Shape::Plus => vec![(0, 1), (1, 0), (1, 1), (1, 2), (2, 1)]
                .iter()
                .map(|p| Point::new(p.0, p.1))
                .collect(),
            Shape::L => vec![(0, 0), (1, 0), (2, 0), (2, 1), (2, 2)]
                .iter()
                .map(|p| Point::new(p.0, p.1))
                .collect(),
            Shape::Tall => vec![(0, 0), (0, 1), (0, 2), (0, 3)]
                .iter()
                .map(|p| Point::new(p.0, p.1))
                .collect(),
            Shape::Square => vec![(0, 0), (0, 1), (1, 0), (1, 1)]
                .iter()
                .map(|p| Point::new(p.0, p.1))
                .collect(),
        }
    }
}

#[derive(Clone, Copy, Debug)]
struct Rock {
    pub pos: Point,
    pub shape: Shape,
}

impl Rock {
    fn new(pos: Point, shape: Shape) -> Rock {
        Rock { pos, shape }
    }

    pub fn bb(&self) -> (Point, Point) {
        match self.shape {
            Shape::Flat => (self.pos - Point::new(0, 0), self.pos + Point::new(3, 0)),
            Shape::Plus => (self.pos - Point::new(0, -2), self.pos + Point::new(2, 0)),
            Shape::L => (self.pos - Point::new(0, -2), self.pos + Point::new(2, 0)),
            Shape::Tall => (self.pos - Point::new(0, -3), self.pos + Point::new(0, 0)),
            Shape::Square => (self.pos - Point::new(0, -1), self.pos + Point::new(1, 0)),
        }
    }

    pub fn points(&self) -> Vec<Point> {
        self.shape.points().iter().map(|p| *p + self.pos).collect()
    }

    pub fn collide(&self, other: &Rock) -> bool {
        if (self.pos.y - other.pos.y).abs() > 4 {
            return false;
        }
        // todo: use the bb to speed up collide
        let p1 = self.points();
        let p2 = other.points();
        for p in p1 {
            if p2.contains(&p) {
                return true;
            }
        }

        false
    }
}

struct World {
    pub rocks: Vec<Rock>,
    pub next_shape: Shape,
    pub time: u32,
    falling: Option<Rock>,
    heights: Vec<i32>,
    pub pattern: Option<usize>,
}

impl World {
    fn new() -> World {
        World {
            rocks: Vec::new(),
            next_shape: Shape::Flat,
            time: 0,
            falling: None,
            heights: Vec::new(),
            pattern: None,
        }
    }

    fn collide(&self, rock: &Rock) -> bool {
        self.rocks.iter().rev().any(|r| r.collide(&rock))
    }

    fn check_pattern(&mut self) -> () {
        let max_width = self.heights.len() / 2;

        for rw in 0..=max_width - 10 {
            let pw = max_width - rw;
            let mut correct = true;
            let offset = self.heights.len() - pw * 2;
            for i in 0..pw {
                if self.heights[offset + i] != self.heights[offset + pw + i] {
                    correct = false;
                    break;
                }
            }
            if correct {
                println!("Found pattern with width = {}", pw);
                self.pattern = Some(pw);
            }
        }
    }

    pub fn tick(&mut self, streams: &Vec<char>) -> bool {
        if self.falling.is_none() {
            let pos = Point::new(2, self.highest_height() + 1 + 3);
            self.falling = Some(Rock::new(pos, self.next_shape.clone()));
            self.next_shape = self.next_shape.next();
        }

        let gas_idx = self.time % streams.len() as u32;
        self.time += 1;
        let gas_dir = match streams[gas_idx as usize] {
            '>' => Point::new(1, 0),
            '<' => Point::new(-1, 0),
            _ => panic!("Wrong index!"),
        };

        let mut rock = self.falling.unwrap().clone();
        if rock.bb().0.x + gas_dir.x >= 0 && rock.bb().1.x + gas_dir.x < 7 {
            rock.pos = rock.pos + gas_dir;

            if self.collide(&rock) {
                rock.pos = rock.pos - gas_dir;
            }
        }

        let downward = Rock::new(rock.pos + Point::new(0, -1), rock.shape);
        if self.collide(&downward) || downward.bb().1.y == -1 {
            // resting
            let old = self.highest_height();
            self.rocks.push(rock);
            self.heights.push(self.highest_height() - old);
            self.falling = None;
            if self.time > streams.len() as u32 * 4 {
                self.check_pattern();
            }
            return true;
        } else {
            self.falling = Some(downward);
        }
        false
    }

    pub fn sum_height_using_pattern(&self, count: i64) -> u64 {
        let mut sum = 0;
        if count <= self.heights.len() as i64 {
            for i in 0..count {
                sum += self.heights[i as usize];
            }

            return sum as u64;
        }

        let width = self.pattern.unwrap() as i64;
        let offset = self.heights.len() as i64 - width;
        let count_div = (count - offset) / width;
        let count_mod = (count - offset) % width;

        let offset_sum: i32 = self.heights[0..offset as usize].iter().sum();
        let pattern_sum: i32 = self.heights[offset as usize..(offset + width) as usize]
            .iter()
            .sum();

        let sum_mod: i32 = self.heights[offset as usize..(offset + count_mod) as usize]
            .iter()
            .sum();
        offset_sum as u64 + (pattern_sum as u64 * count_div as u64) + sum_mod as u64
    }

    pub fn highest_height(&self) -> i32 {
        self.rocks
            .iter()
            .fold(-1, |accum, r| cmp::max(accum, r.bb().0.y))
    }

    #[allow(dead_code)]
    fn render(&self, height: i32) -> String {
        let mut buf = "".to_string();
        for ry in 0..height {
            let y = height - ry - 1;
            for x in 0..7 {
                let p = Point::new(x, y);
                if self.rocks.iter().any(|r| r.points().contains(&p)) {
                    buf.push('#');
                } else if self.falling.is_some() && self.falling.unwrap().points().contains(&p) {
                    buf.push('@');
                } else {
                    buf.push('.');
                }
            }
            buf.push('\n');
        }
        return buf;
    }
}

#[aoc(day17, part1)]
pub fn find_tower_height(input: &Vec<char>) -> u64 {
    let mut world = World::new();

    while world.pattern.is_none() && world.rocks.len() < 2022 {
        world.tick(&input);
    }

    world.sum_height_using_pattern(2022) as u64
}

#[aoc(day17, part2)]
pub fn find_tower_height_2(input: &Vec<char>) -> u64 {
    let mut world = World::new();

    while world.pattern.is_none() {
        world.tick(&input);
    }

    world.sum_height_using_pattern(1_000_000_000_000) as u64
}

#[cfg(test)]
mod tests {
    use super::*;

    const DAY17_EXAMPLE: &str = ">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>";

    //#[ignore]
    #[test]
    fn test_day17_part1() {
        let input = parse_input(DAY17_EXAMPLE);
        assert_eq!(find_tower_height(&input), 3068);
    }

    #[test]
    fn test_day17_collide() {
        let flat = Rock::new(Point::new(3, 0), Shape::Flat);
        let plus = Rock::new(Point::new(3, 0), Shape::Plus);
        assert_eq!(plus.collide(&flat), true);
        assert_eq!(flat.collide(&plus), true);
    }

    #[test]
    fn test_day17_collide2() {
        let flat = Rock::new(Point::new(3, 0), Shape::Flat);
        let lshape = Rock::new(Point::new(1, 0), Shape::L);
        assert_eq!(flat.collide(&lshape), true);
        assert_eq!(lshape.collide(&flat), true);
    }

    #[test]
    fn test_day17_collide3() {
        let plus = Rock::new(Point::new(1, 2), Shape::Plus);
        let lshape = Rock::new(Point::new(1, 0), Shape::L);
        assert_eq!(plus.collide(&lshape), false);
        assert_eq!(lshape.collide(&plus), false);
    }

    #[test]
    fn test_day17_collide4() {
        let plus = Rock::new(Point::new(1, 1), Shape::Plus);
        let lshape = Rock::new(Point::new(1, 0), Shape::L);
        assert_eq!(plus.collide(&lshape), true);
        assert_eq!(lshape.collide(&plus), true);
    }

    #[test]
    fn test_day17_collide5() {
        let tall = Rock::new(Point::new(3, 0), Shape::Tall);
        let lshape = Rock::new(Point::new(0, 0), Shape::L);
        assert_eq!(tall.collide(&lshape), false);
        assert_eq!(lshape.collide(&tall), false);
    }

    #[test]
    fn test_day17_collide6() {
        let plus = Rock::new(Point::new(2, 1), Shape::Plus);
        let tall = Rock::new(Point::new(2, 0), Shape::Tall);
        assert_eq!(plus.collide(&tall), true);
        assert_eq!(tall.collide(&plus), true);
    }

    #[test]
    fn test_day17_collide7() {
        let square = Rock::new(Point::new(0, 1), Shape::Square);
        let lshape = Rock::new(Point::new(1, 0), Shape::L);
        assert_eq!(square.collide(&lshape), false);
        assert_eq!(lshape.collide(&square), false);
    }

    #[test]
    fn test_day17_collide8() {
        let plus = Rock::new(Point::new(1, 2), Shape::Plus);
        let square = Rock::new(Point::new(0, 1), Shape::Square);
        assert_eq!(plus.collide(&square), false);
        assert_eq!(square.collide(&plus), false);
    }

    #[test]
    fn test_day17_collide9() {
        let plus = Rock::new(Point::new(2, 10), Shape::Plus);
        let lshape = Rock::new(Point::new(4, 12), Shape::L);
        assert_eq!(plus.collide(&lshape), false);
        assert_eq!(lshape.collide(&plus), false);
    }

    #[test]
    fn test_day17_collide10() {
        let plus = Rock::new(Point::new(2, 10), Shape::Plus);
        let lshape = Rock::new(Point::new(4, 11), Shape::L);
        assert_eq!(plus.collide(&lshape), true);
        assert_eq!(lshape.collide(&plus), true);
    }
}
