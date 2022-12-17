use crate::utils::Point;
use aoc_runner_derive::aoc;
use aoc_runner_derive::aoc_generator;
use std::cmp;

#[aoc_generator(day17)]
pub fn parse_input(input: &str) -> Vec<char> {
    input.chars().filter(|c| *c == '>' || *c == '<').collect()
}

#[derive(Clone, Copy, Debug)]
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
            Shape::Flat => vec![(-1, 0), (0, 0), (1, 0), (2, 0)]
                .iter()
                .map(|p| Point::new(p.0, p.1))
                .collect(),
            Shape::Plus => vec![(-1, 1), (0, 0), (0, 1), (0, 2), (1, 1)]
                .iter()
                .map(|p| Point::new(p.0, p.1))
                .collect(),
            Shape::L => vec![(-1, 0), (0, 0), (1, 0), (1, 1), (1, 2)]
                .iter()
                .map(|p| Point::new(p.0, p.1))
                .collect(),
            Shape::Tall => vec![(0, 0), (0, 1), (0, 2), (0, 3)]
                .iter()
                .map(|p| Point::new(p.0, p.1))
                .collect(),
            Shape::Square => vec![(0, 0), (1, 0), (0, 1), (1, 1)]
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
            Shape::Flat => (self.pos - Point::new(1, 0), self.pos + Point::new(2, 0)),
            Shape::Plus => (self.pos - Point::new(1, -2), self.pos + Point::new(1, 0)),
            Shape::L => (self.pos - Point::new(1, -2), self.pos + Point::new(1, 0)),
            Shape::Tall => (self.pos - Point::new(0, -3), self.pos + Point::new(0, 0)),
            Shape::Square => (self.pos - Point::new(0, -1), self.pos + Point::new(1, 0)),
        }
    }

    fn between(point: &Point, min: &Point, max: &Point) -> bool {
        // y is inverted since min = top-left
        point.x >= min.x && point.x <= max.x && point.y <= min.y && point.y >= max.y
    }
    pub fn collide(&self, other: &Rock) -> bool {
        let bb = self.bb();
        let bb2 = other.bb();
        if Self::between(&bb2.0, &bb.0, &bb.1) || Self::between(&bb2.1, &bb.0, &bb.1) {
            let p1 = self
                .shape
                .points()
                .iter()
                .map(|p| *p + self.pos)
                .collect::<Vec<Point>>();
            let p2 = other
                .shape
                .points()
                .iter()
                .map(|p| *p + other.pos)
                .collect::<Vec<Point>>();
            for p in p1 {
                if p2.contains(&p) {
                    return true;
                }
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
}

impl World {
    fn new() -> World {
        World {
            rocks: Vec::new(),
            next_shape: Shape::Flat,
            time: 0,
            falling: None,
        }
    }

    fn collide(&self, rock: &Rock) -> bool {
        self.rocks.iter().any(|r| r.collide(&rock))
    }

    pub fn tick(&mut self, streams: &Vec<char>) -> bool {
        if self.falling.is_none() {
            let pos = Point::new(3, self.highest_height() + 3);
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
            self.rocks.push(rock);
            self.falling = None;
            return true;
        } else {
            self.falling = Some(downward);
        }
        false
    }

    pub fn highest_height(&self) -> i32 {
        self.rocks
            .iter()
            .fold(0, |accum, r| cmp::max(accum, r.bb().0.y))
    }
}
#[aoc(day17, part1)]
pub fn find_tower_height(input: &Vec<char>) -> u64 {
    let mut count = 2022;
    let mut world = World::new();

    while count >= 0 {
        if world.tick(&input) {
            count -= 1;
        }
    }

    world.highest_height() as u64
}

#[cfg(test)]
mod tests {
    use super::*;

    const DAY17_EXAMPLE: &str = ">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>";

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
    }
}
