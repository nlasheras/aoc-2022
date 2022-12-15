use aoc_runner_derive::aoc;
use aoc_runner_derive::aoc_generator;
use regex::Regex;
use std::cmp;

use crate::utils::Point;

pub struct Sensor {
    pub pos: Point,
    pub closest_beacon: Point,
}

impl Sensor {
    pub fn new(pos: (i32, i32), beacon: (i32, i32)) -> Sensor {
        Sensor {
            pos: Point::new(pos.0, pos.1),
            closest_beacon: Point::new(beacon.0, beacon.1),
        }
    }
}

#[aoc_generator(day15)]
pub fn parse_input(input: &str) -> Vec<Sensor> {
    let re =
        Regex::new(r"Sensor at x=(-?\d+), y=(-?\d+): closest beacon is at x=(-?\d+), y=(-?\d+)")
            .unwrap();
    input
        .lines()
        .map(|s| {
            let error_msg = "Invalid input!";
            let numbers: Vec<i32> = re
                .captures(s)
                .expect(error_msg)
                .iter()
                .skip(1)
                .map(|s| s.unwrap().as_str().parse::<i32>().expect(error_msg))
                .collect(); // get the 4 numbers as a Vec<u8>
            Sensor::new((numbers[0], numbers[1]), (numbers[2], numbers[3]))
        })
        .collect()
}

fn count_positions_in_row(sensors: &Vec<Sensor>, row: i32) -> u64 {
    let mut min_x = i32::MAX;
    let mut max_x = i32::MIN;
    sensors.iter().for_each(|s| {
        min_x = cmp::min(s.pos.x - s.pos.manhattan_dist(&s.closest_beacon), min_x);
        max_x = cmp::max(s.pos.x + s.pos.manhattan_dist(&s.closest_beacon), max_x);
    });

    let mut sum = 0;
    for x in min_x..=max_x {
        let candidate = Point::new(x, row);
        if sensors.iter().any(|s| s.closest_beacon == candidate) {
            continue; // there is already a beacon
        }
        if sensors.iter().any(|s| {
            let closest_dist = s.pos.manhattan_dist(&s.closest_beacon);
            let dist = s.pos.manhattan_dist(&candidate);
            dist <= closest_dist
        }) {
            sum += 1;
        }
    }
    sum
}

#[aoc(day15, part1)]
pub fn count_positions_in_row_10(input: &Vec<Sensor>) -> u64 {
    count_positions_in_row(input, 2000000)
}

#[cfg(test)]
mod tests {
    use super::*;

    const DAY15_EXAMPLE: &str = "Sensor at x=2, y=18: closest beacon is at x=-2, y=15
Sensor at x=9, y=16: closest beacon is at x=10, y=16
Sensor at x=13, y=2: closest beacon is at x=15, y=3
Sensor at x=12, y=14: closest beacon is at x=10, y=16
Sensor at x=10, y=20: closest beacon is at x=10, y=16
Sensor at x=14, y=17: closest beacon is at x=10, y=16
Sensor at x=8, y=7: closest beacon is at x=2, y=10
Sensor at x=2, y=0: closest beacon is at x=2, y=10
Sensor at x=0, y=11: closest beacon is at x=2, y=10
Sensor at x=20, y=14: closest beacon is at x=25, y=17
Sensor at x=17, y=20: closest beacon is at x=21, y=22
Sensor at x=16, y=7: closest beacon is at x=15, y=3
Sensor at x=14, y=3: closest beacon is at x=15, y=3
Sensor at x=20, y=1: closest beacon is at x=15, y=3";

    #[test]
    fn test_day15_part1() {
        let input = parse_input(DAY15_EXAMPLE);
        assert_eq!(count_positions_in_row(&input, 10), 26);
    }
}
