use crate::utils::Point;
use aoc_runner_derive::aoc;
use aoc_runner_derive::aoc_generator;
use std::cmp;
use std::collections::BTreeSet;

#[derive(Clone, Debug, PartialEq, PartialOrd, Eq, Ord)]
pub struct Elf {
    pub position: Point,
}

impl Elf {
    fn new(x: i32, y: i32) -> Elf {
        Elf {
            position: Point::new(x, y),
        }
    }
}

#[aoc_generator(day23)]
pub fn parse_input(input: &str) -> BTreeSet<Elf> {
    let mut map = BTreeSet::new();
    let rows = input.lines().map(|s| {
        s.char_indices()
            .map(|(i, c)| if c == '#' { i as i32 } else { -1 })
            .filter(|i| *i != -1)
            .collect::<Vec<i32>>()
    });
    let mut y = 0;
    for row in rows {
        for x in row {
            map.insert(Elf::new(x, y));
        }
        y += 1;
    }
    map
}

fn is_neighbor(elf: &Elf, other: &Elf) -> bool {
    (elf.position.x - other.position.x).abs() <= 1 && (elf.position.y - other.position.y).abs() <= 1
}

#[allow(dead_code)]
fn print(set: &BTreeSet<Elf>) {
    let mut min = (i32::MAX, i32::MAX);
    let mut max = (i32::MIN, i32::MIN);
    set.iter().for_each(|e| {
        min = (cmp::min(min.0, e.position.x), cmp::min(min.1, e.position.y));
        max = (cmp::max(max.0, e.position.x), cmp::max(max.1, e.position.y));
    });
    let mut buf = "".to_string();
    for y in min.1..=max.1 {
        for x in min.0..=max.0 {
            if set.contains(&Elf::new(x, y)) {
                buf.push('#');
            } else {
                buf.push('.');
            }
        }
        buf.push('\n');
    }
    println!("{}", buf);
}

fn simulate(set: &BTreeSet<Elf>, round: usize) -> BTreeSet<Elf> {
    let mut positions = Vec::new();
    for elf in set {
        if !set.iter().any(|e| e != elf && is_neighbor(e, elf)) {
            positions.push((elf, elf.position));
            continue;
        }

        let mut moved = false;
        let order = ['N', 'S', 'W', 'E'];
        for i in 0..4 {
            let c = order[(i + round) % 4];
            let dir = match c {
                'N' => Point::new(0, -1),
                'S' => Point::new(0, 1),
                'W' => Point::new(-1, 0),
                'E' => Point::new(1, 0),
                _ => todo!(),
            };

            let new_pos = elf.position + dir;
            if set.contains(&Elf::new(new_pos.x, new_pos.y)) {
                continue;
            }
            if dir.x == 0
                && (set.contains(&Elf::new(new_pos.x - 1, new_pos.y))
                    || set.contains(&Elf::new(new_pos.x + 1, new_pos.y)))
            {
                continue;
            }
            if dir.y == 0
                && (set.contains(&Elf::new(new_pos.x, new_pos.y - 1))
                    || set.contains(&Elf::new(new_pos.x, new_pos.y + 1)))
            {
                continue;
            }

            positions.push((elf, new_pos));
            moved = true;
            break;
        }

        if !moved {
            positions.push((elf, elf.position));
        }
    }

    let mut out = BTreeSet::new();
    for pair in positions.iter() {
        if positions.iter().any(|(e, p)| *p == pair.1 && *e != pair.0) {
            out.insert(pair.0.clone());
        } else {
            out.insert(Elf::new(pair.1.x, pair.1.y));
        }
    }
    out
}

#[aoc(day23, part1)]
pub fn count_empty_ground(input: &BTreeSet<Elf>) -> u64 {
    let mut set = input.clone();
    for i in 0..10 {
        set = simulate(&set, i);
    }
    let mut min = (i32::MAX, i32::MAX);
    let mut max = (i32::MIN, i32::MIN);
    set.iter().for_each(|e| {
        min = (cmp::min(min.0, e.position.x), cmp::min(min.1, e.position.y));
        max = (cmp::max(max.0, e.position.x), cmp::max(max.1, e.position.y));
    });
    let area = ((max.0 - min.0 + 1) * (max.1 - min.1 + 1)) as u64;
    area - input.len() as u64
}

#[aoc(day23, part2)]
fn simulate_until_stop(input: &BTreeSet<Elf>) -> u64 {
    let mut set = input.clone();
    let mut round = 0;
    loop {
        let new_set = simulate(&set, round);
        round += 1;

        if new_set == set {
            break;
        }
        set = new_set;
    }
    round as u64
}

#[cfg(test)]
mod tests {
    use super::*;

    const DAY23_EXAMPLE_SMALL: &str = ".....
..##.
..#..
.....
..##.
.....";

    const DAY23_EXAMPLE: &str = "..............
..............
.......#......
.....###.#....
...#...#.#....
....#...##....
...#.###......
...##.#.##....
....#..#......
..............
..............
..............";

    #[test]
    fn test_day23_parse() {
        let input = parse_input(DAY23_EXAMPLE_SMALL);
        assert_eq!(input.len(), 5);
    }

    #[test]
    fn test_day23_part1() {
        let input = parse_input(DAY23_EXAMPLE);
        assert_eq!(count_empty_ground(&input), 110);
    }

    #[test]
    fn test_day23_part2() {
        let input = parse_input(DAY23_EXAMPLE);
        assert_eq!(simulate_until_stop(&input), 20);
    }
}
