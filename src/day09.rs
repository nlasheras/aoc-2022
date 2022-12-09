use std::collections::BTreeSet;

use aoc_runner_derive::aoc;
use aoc_runner_derive::aoc_generator;

#[aoc_generator(day9)]
pub fn parse_input(input: &str) -> Vec<(String, u64)> {
    input
        .lines()
        .map(|s| {
            let parts : Vec<&str> = s.split(" ").collect();
            (String::from(parts[0]), parts[1].parse::<u64>().unwrap())
        })
        .collect()
}

fn offset(pos: (i32, i32), other : (i32, i32)) -> (i32, i32) {
    (pos.0 - other.0, pos.1 - other.1)
}

#[aoc(day9, part1)]
fn count_tail_positions(motions: &Vec<(String, u64)>) -> u64 {
    let mut tail = (0, 0);
    let mut head = (0, 0);
    let mut set = BTreeSet::new();
    set.insert(tail);
    for motion in motions {
        let dir = match motion.0.as_str() {
            "R" => (1, 0),
            "L" => (-1, 0),
            "U" => (0, -1),
            "D" => (0, 1),
            _ => panic!("Unknown command!")
        };

        for _ in 0..motion.1 {
            head = (head.0 + dir.0, head.1 + dir.1);
            let mut dist = offset(head, tail);
            while dist.0.abs() > 1 || dist.1.abs() > 1 {
                tail = (tail.0 + dist.0.signum(), tail.1 + dist.1.signum());
                set.insert(tail);
                dist = offset(head, tail); // update offset
            }
        }
    }
    
    set.into_iter().count() as u64
}

fn count_tail_positions_arbitrary(motions: &Vec<(String, u64)>, rope_len: usize) -> u64 {
    let mut rope = vec![(0,0); rope_len];
    let mut set = BTreeSet::new();
    set.insert(rope[rope_len-1]);
    for motion in motions {
        let dir = match motion.0.as_str() {
            "R" => (1, 0),
            "L" => (-1, 0),
            "U" => (0, -1),
            "D" => (0, 1),
            _ => panic!("Unknown command!")
        };

        for _ in 0..motion.1 {
            rope[0] = (rope[0].0 + dir.0, rope[0].1 + dir.1);
            for i in 1..rope_len {
                let mut dist = offset(rope[i-1], rope[i]);
                while dist.0.abs() > 1 || dist.1.abs() > 1 {
                    rope[i] = (rope[i].0 + dist.0.signum(), rope[i].1 + dist.1.signum());
                    if i == rope_len-1 {
                        set.insert(rope[i]);
                    }
                    dist = offset(rope[i-1], rope[i]); // update offset
                }
            }
        }
    }
    
    set.into_iter().count() as u64
}

#[aoc(day9, part2)]
fn count_tail_positions_bigger_rope(motions: &Vec<(String, u64)>) -> u64 {
    count_tail_positions_arbitrary(motions, 10)
}

#[cfg(test)]
mod tests {
    use super::*;

    const DAY09_EXAMPLE: &str = "R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2";

    #[test]
    fn test_day9_part1() {
        let input = parse_input(DAY09_EXAMPLE);
        assert_eq!(count_tail_positions(&input), 13);
    }

    #[test]
    fn test_day9_part1_refactor() {
        let input = parse_input(DAY09_EXAMPLE);
        assert_eq!(count_tail_positions_arbitrary(&input, 2), 13);
    }

    const DAY09_EXAMPLE2: &str = "R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20";

    
    #[test]
    fn test_day9_part2() {
        let input = parse_input(DAY09_EXAMPLE2);
        assert_eq!(count_tail_positions_arbitrary(&input, 10), 36);
    }
}