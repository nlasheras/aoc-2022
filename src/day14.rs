use aoc_runner_derive::aoc;
use aoc_runner_derive::aoc_generator;
use std::cmp;
use std::collections::BTreeSet;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Ord, PartialOrd)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl Point {
    fn new(x: i32, y: i32) -> Point {
        Point { x: x, y: y }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct Line {
    pub start: Point,
    pub end: Point,
}

impl Line {
    fn new(start: (i32, i32), end: (i32, i32)) -> Line {
        Line {
            start: Point::new(start.0, start.1),
            end: Point::new(end.0, end.1),
        }
    }

    pub fn contains(&self, point: (i32, i32)) -> bool {
        (self.start.x <= point.0
            && point.0 <= self.end.x
            && self.start.y <= point.1
            && point.1 <= self.end.y)
            || (self.end.x <= point.0
                && point.0 <= self.start.x
                && self.end.y <= point.1
                && point.1 <= self.start.y)
    }
}

#[aoc_generator(day14)]
pub fn parse_input(input: &str) -> Vec<Line> {
    let points = input
        .split("\n")
        .map(|s| {
            let parts = s
                .split(" -> ")
                .map(|s| {
                    let nums = s
                        .split(",")
                        .map(|s| s.parse::<i32>().unwrap())
                        .collect::<Vec<i32>>();
                    (nums[0], nums[1])
                })
                .collect::<Vec<(i32, i32)>>();
            parts
        })
        .collect::<Vec<Vec<(i32, i32)>>>();

    let mut ret = Vec::<Line>::new();
    for lines in points.into_iter() {
        for i in 1..lines.len() {
            ret.push(Line::new(
                (lines[i - 1].0, lines[i - 1].1),
                (lines[i].0, lines[i].1),
            ));
        }
    }
    ret
}

fn is_blocked_by_rocks(rocks: &Vec<Line>, point: (i32, i32)) -> bool {
    for line in rocks {
        if line.contains(point) {
            return true;
        }
    }
    return false;
}

type SandSet = BTreeSet<Point>;

fn is_blocked_by_sand(sand: &SandSet, point: (i32, i32)) -> bool {
    sand.contains(&Point::new(point.0, point.1))
}

fn test_block(rocks: &Vec<Line>, sand: &SandSet, point: (i32, i32)) -> bool {
    is_blocked_by_rocks(rocks, point) || is_blocked_by_sand(sand, point)
}

fn is_falling_to_void(rocks: &Vec<Line>, point: &Point) -> bool {
    for l in rocks {
        if point.y <= l.start.y && point.y <= l.end.y {
            return false;
        }
    }
    true
}

#[aoc(day14, part1)]
fn count_sand_in_rest(input: &Vec<Line>) -> u64 {
    let mut _time = 0;
    let mut failing_to_void = false;
    let mut resting_sand = SandSet::new();
    while !failing_to_void {
        let mut sand = Point::new(500, 0);
        let mut resting = false;

        while !resting {
            let dirs = [(0, 1), (-1, 1), (1, 1)];
            resting = true;
            for d in dirs {
                if !test_block(&input, &resting_sand, (sand.x + d.0, sand.y + d.1)) {
                    sand.x = sand.x + d.0;
                    sand.y = sand.y + d.1;
                    resting = false;
                    break;
                }
            }

            _time += 1;

            if resting {
                resting_sand.insert(sand);
            } else {
                failing_to_void = is_falling_to_void(&input, &sand);
                if failing_to_void {
                    break;
                }
            }
        }
    }
    resting_sand.len() as u64
}

#[aoc(day14, part2)]
fn count_sand_until_block(input: &Vec<Line>) -> u64 {
    let floor = input
        .iter()
        .map(|l| cmp::max(l.start.y, l.end.y))
        .max()
        .unwrap()
        + 2;

    let mut _time = 0;
    let mut resting_sand = SandSet::new();
    let mut blocking_source = false;
    let source = Point::new(500, 0);
    while !blocking_source {
        let mut sand = source;
        let mut resting = false;

        while !resting {
            let dirs = [(0, 1), (-1, 1), (1, 1)];
            resting = true;
            for d in dirs {
                if !test_block(&input, &resting_sand, (sand.x + d.0, sand.y + d.1)) {
                    sand.x = sand.x + d.0;
                    sand.y = sand.y + d.1;
                    resting = false;
                    break;
                }
            }

            if sand.y == floor - 1 {
                resting = true;
            }

            _time += 1;

            if resting {
                resting_sand.insert(sand);
            }
            if sand == source {
                blocking_source = true;
                break;
            }
        }
    }
    resting_sand.len() as u64
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
        assert!(line.contains((point.x, point.y)));
    }

    #[test]
    fn test_day14_contains2() {
        let line = Line::new((502, 4), (502, 9));
        let point = Point::new(502, 8);
        assert!(line.contains((point.x, point.y)));
    }

    #[test]
    fn test_day14_blocks() {
        let input = parse_input(DAY14_EXAMPLE);
        let point = Point::new(502, 8);
        assert!(is_blocked_by_rocks(&input, (point.x, point.y)));
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
