use aoc_runner_derive::aoc;
use aoc_runner_derive::aoc_generator;
use itertools::Itertools;
use std::collections::BTreeSet;

type Cube = (i32, i32, i32);

#[aoc_generator(day18)]
pub fn parse_input(input: &str) -> BTreeSet<Cube> {
    input
        .lines()
        .map(|x| {
            x.split(",")
                .map(|s| s.parse::<i32>().unwrap())
                .collect_tuple()
                .unwrap()
        })
        .collect::<BTreeSet<Cube>>()
}

#[aoc(day18, part1)]
pub fn find_surface_area(cubes: &BTreeSet<Cube>) -> u64 {
    let mut sum = 0;
    for c in cubes.iter() {
        sum += [
            (1, 0, 0),
            (-1, 0, 0),
            (0, 1, 0),
            (0, -1, 0),
            (0, 0, 1),
            (0, 0, -1),
        ]
        .into_iter()
        .map(|d| (c.0 + d.0, c.1 + d.1, c.2 + d.2))
        .filter(|c| !cubes.contains(&c))
        .count();
    }
    sum as u64
}

fn is_open_water(cube: &Cube, min: &Cube, max: &Cube) -> bool {
    cube.0 == min.0
        || cube.0 == max.0
        || cube.1 == min.1
        || cube.1 == max.1
        || cube.2 == min.2
        || cube.2 == max.2
}

fn in_pocket(set: &BTreeSet<Cube>, cube: &Cube, min: &Cube, max: &Cube) -> bool {
    let mut closed_set = BTreeSet::new();
    let mut open_set = Vec::new();
    open_set.push(cube.clone());
    while !open_set.is_empty() {
        let c = open_set.pop().unwrap();
        closed_set.insert(c);

        if is_open_water(&c, min, max) {
            return false;
        }

        [
            (1, 0, 0),
            (-1, 0, 0),
            (0, 1, 0),
            (0, -1, 0),
            (0, 0, 1),
            (0, 0, -1),
        ]
        .iter()
        .map(|d| (c.0 + d.0, c.1 + d.1, c.2 + d.2))
        .for_each(|c| {
            if !closed_set.contains(&c) && !set.contains(&c) {
                open_set.push(c);
            }
        });
    }

    true
}

#[aoc(day18, part2)]
pub fn find_surface_area_without_pockets(cubes: &BTreeSet<Cube>) -> u64 {
    let mut sum = 0;
    let mut min = (i32::MAX, i32::MAX, i32::MAX);
    let mut max = (i32::MIN, i32::MIN, i32::MIN);
    cubes.iter().for_each(|c| {
        min = (
            std::cmp::min(min.0, c.0 - 1),
            std::cmp::min(min.1, c.1 - 1),
            std::cmp::min(min.2, c.2 - 1),
        );
        max = (
            std::cmp::max(max.0, c.0 + 1),
            std::cmp::max(max.1, c.1 + 1),
            std::cmp::max(max.2, c.2 + 1),
        );
    });
    for c in cubes.iter() {
        sum += [
            (1, 0, 0),
            (-1, 0, 0),
            (0, 1, 0),
            (0, -1, 0),
            (0, 0, 1),
            (0, 0, -1),
        ]
        .into_iter()
        .map(|d| (c.0 + d.0, c.1 + d.1, c.2 + d.2))
        .filter(|c| !cubes.contains(&c) && !in_pocket(&cubes, &c, &min, &max))
        .count();
    }
    sum as u64
}

#[cfg(test)]
mod tests {
    use super::*;

    const DAY18_EXAMPLE: &str = "2,2,2
1,2,2
3,2,2
2,1,2
2,3,2
2,2,1
2,2,3
2,2,4
2,2,6
1,2,5
3,2,5
2,1,5
2,3,5";

    #[test]
    fn test_day18_part1() {
        let input = parse_input(DAY18_EXAMPLE);
        assert_eq!(find_surface_area(&input), 64);
    }

    #[test]
    fn test_day18_part2() {
        let input = parse_input(DAY18_EXAMPLE);
        assert_eq!(find_surface_area_without_pockets(&input), 58);
    }
}
