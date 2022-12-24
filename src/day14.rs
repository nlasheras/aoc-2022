use crate::utils::Point;
use aoc_runner_derive::aoc;
use aoc_runner_derive::aoc_generator;
use itertools::Itertools;
use std::cmp;
use std::collections::BTreeSet;

#[derive(Copy, Clone, Debug)]
pub struct Line {
    pub start: Point,
    pub end: Point,
}

impl Line {
    fn new(start: (i32, i32), end: (i32, i32)) -> Line {
        Line {
            start: Point::new(cmp::min(start.0, end.0), cmp::min(start.1, end.1)),
            end: Point::new(cmp::max(start.0, end.0), cmp::max(start.1, end.1)),
        }
    }

    pub fn contains(&self, point: &Point) -> bool {
        self.start.x <= point.x
            && point.x <= self.end.x
            && self.start.y <= point.y
            && point.y <= self.end.y
    }
}

#[aoc_generator(day14)]
pub fn parse_input(input: &str) -> Vec<Line> {
    input
        .lines()
        .flat_map(|s| {
            s.split(" -> ")
                .map(|s| {
                    s.split(',')
                        .map(|s| s.parse::<i32>().unwrap())
                        .collect_tuple::<(i32, i32)>()
                        .unwrap()
                })
                .collect::<Vec<(i32, i32)>>()
                .windows(2)
                .map(|w| Line::new(w[0], w[1]))
                .collect::<Vec<Line>>()
        })
        .collect()
}

type SandSet = BTreeSet<Point>;

struct World<'a> {
    pub rocks: &'a Vec<Line>,
    pub resting_sand: SandSet,
    pub source: Point,
    pub floor: i32,
    pub simulating: Option<Point>,
}

impl World<'_> {
    fn new(rocks: &Vec<Line>) -> World {
        let floor = rocks
            .iter()
            .map(|l| cmp::max(l.start.y, l.end.y))
            .max()
            .unwrap()
            + 2;
        World {
            rocks,
            resting_sand: SandSet::new(),
            source: Point::new(500, 0),
            floor,
            simulating: None,
        }
    }

    pub fn falling_under_walls(&self) -> bool {
        if let Some(point) = self.simulating {
            if point.y == self.floor - 1 {
                return true;
            }
        }
        self.resting_sand.iter().any(|p| p.y == self.floor - 1)
    }

    fn update_moving_sand(&mut self) -> bool {
        let sand = self.simulating.unwrap();

        // add the floor resting condition for part 2
        if sand.y == self.floor - 1 {
            // hit floor
            return false;
        }

        let movement_directions = [Point::new(0, 1), Point::new(-1, 1), Point::new(1, 1)];
        for vector in movement_directions {
            let new_position = sand + vector;
            if !self.is_blocked(&new_position) {
                self.simulating = Some(new_position);
                return true;
            }
        }
        false
    }

    pub fn tick(&mut self) -> bool {
        // if there is no sand, spawn one at source
        if self.simulating.is_none() {
            self.simulating = Some(Point::new(self.source.x, self.source.y));
        }

        if !self.update_moving_sand() {
            let sand = self.simulating.unwrap();
            self.resting_sand.insert(sand);
            if sand != self.source {
                // so we can check that a sand is blocking the source
                self.simulating = None;
            }
            return true;
        }
        false
    }

    pub fn sand_blocking_source(&self) -> bool {
        if let Some(sand) = self.simulating {
            return sand == self.source;
        }
        false
    }

    pub fn is_blocked_by_rocks(&self, point: &Point) -> bool {
        self.rocks.iter().any(|line| line.contains(point))
    }

    fn is_blocked_by_sand(&self, point: &Point) -> bool {
        self.resting_sand.contains(point)
    }

    fn is_blocked(&self, point: &Point) -> bool {
        self.is_blocked_by_rocks(point) || self.is_blocked_by_sand(point)
    }
}

#[aoc(day14, part1)]
fn count_sand_in_rest(input: &Vec<Line>) -> u64 {
    let mut world = World::new(input);

    while !world.falling_under_walls() {
        world.tick();
    }
    world.resting_sand.len() as u64
}

#[aoc(day14, part2)]
fn count_sand_until_block(input: &Vec<Line>) -> u64 {
    let mut world = World::new(input);

    while !world.sand_blocking_source() {
        world.tick();
    }

    world.resting_sand.len() as u64
}

#[cfg(test)]
mod tests {
    use super::*;

    const DAY14_EXAMPLE: &str = "498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9";

    #[test]
    fn test_day14_contains() {
        let line = Line::new((502, 9), (494, 9));
        let point = Point::new(500, 9);
        assert!(line.contains(&point));
    }

    #[test]
    fn test_day14_contains2() {
        let line = Line::new((502, 4), (502, 9));
        let point = Point::new(502, 8);
        assert!(line.contains(&point));
    }

    #[test]
    fn test_day14_blocks() {
        let input = parse_input(DAY14_EXAMPLE);
        let world = World::new(&input);
        let point = Point::new(502, 8);
        assert!(world.is_blocked(&point));
    }

    #[test]
    fn test_day14_part1() {
        let input = parse_input(DAY14_EXAMPLE);
        assert_eq!(count_sand_in_rest(&input), 24);
    }

    #[test]
    fn test_day14_part2() {
        let input = parse_input(DAY14_EXAMPLE);
        assert_eq!(count_sand_until_block(&input), 93);
    }
}
