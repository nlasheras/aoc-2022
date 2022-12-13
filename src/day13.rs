use aoc_runner_derive::aoc;
use aoc_runner_derive::aoc_generator;

use std::fmt;

#[derive(Clone)]
pub enum PacketData {
    Integer(i64),
    List(Vec<PacketData>),
}


impl std::fmt::Debug for PacketData {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut buffer = "".to_string();
        match self {
            Self::Integer(n) => buffer.push_str(&format!("{}", n)),
            Self::List(contents) => {
                buffer.push_str("[");
                for packet in contents {
                    if buffer.len() > 1 {
                        buffer.push_str(",");
                    }
                    buffer.push_str(&format!("{:?}", packet));
                }
                buffer.push_str("]");
            }
        }
        write!(f, "{}", buffer)
    }
}

impl PacketData {
    fn from(input: &str) -> PacketData {
        if input.starts_with('[') {
            // list
            let mut values = Vec::new();
            let input = input.strip_prefix('[').unwrap().strip_suffix(']').unwrap();
            let mut buf = String::new();
            let mut level = 0;
            for c in input.chars() {
                match c {
                    '[' => {
                        level += 1;
                        buf.push('[')
                    }
                    ']' => {
                        level -= 1;
                        buf.push(']')
                    }
                    ',' => {
                        if level == 0 {
                            values.push(PacketData::from(&buf));
                            buf.clear();
                        } else {
                            buf.push(',');
                        }
                    }
                    c => buf.push(c),
                }
            }
            if !buf.is_empty() {
                values.push(PacketData::from(&buf));
            }
            PacketData::List(values)
        } else {
            PacketData::Integer(input.parse().unwrap())
        }
    }

    fn smaller(&self, other: &PacketData) -> bool {
        //println!("Compare {:?} to {:?}", self, other);
        match self {
            PacketData::Integer(n) => {
                match *other {
                    PacketData::Integer(m) => {
                        *n < m
                    },
                    PacketData::List(_) => PacketData::List(vec![self.clone()]).smaller(other)
                }
            },
            PacketData::List(left) => {
                match other {
                    PacketData::Integer(_) => self.smaller(&PacketData::List(vec![other.clone()])),
                    PacketData::List(right) => {
                        for i in 0..left.len()  {
                            let elem_l = left.iter().nth(i);
                            let elem_r = right.iter().nth(i);
                            if let Some(left) = elem_l {
                                if let Some(right) = elem_r {
                                    if left.smaller(right) {
                                        return true;
                                    }
                                    else if right.smaller(left) {
                                        return false;
                                    }
                                }
                                else {
                                    return false;
                                }
                            }
                        }
                        true
                    }
                }
            }
        }
    }
}

#[aoc_generator(day13)]
pub fn parse_input(input: &str) -> Vec<(PacketData, PacketData)> {
    input
        .split("\n\n")
        .map(|s| {
            let mut parts = s.split("\n");
            (
                PacketData::from(parts.next().unwrap()),
                PacketData::from(parts.next().unwrap()),
            )
        })
        .collect::<Vec<(PacketData, PacketData)>>()
}

#[aoc(day13, part1)]
fn sum_packets_in_order(input: &Vec<(PacketData, PacketData)>) -> u64 {
    let are_right_order = input.iter().map(|pair| {
      if pair.0.smaller(&pair.1) {
        return 1;
      }
      0
    }).collect::<Vec<i32>>();
        

    let mut sum = 0;
    for i in 0..are_right_order.len() {
        sum += (i+1) as i32*are_right_order.iter().nth(i).unwrap();
    }
    sum as u64
}

#[cfg(test)]
mod tests {
    use super::*;

    const DAY13_EXAMPLE: &str = "[1,1,3,1,1]
[1,1,5,1,1]

[[1],[2,3,4]]
[[1],4]

[9]
[[8,7,6]]

[[4,4],4,4]
[[4,4],4,4,4]

[7,7,7,7]
[7,7,7]

[]
[3]

[[[]]]
[[]]

[1,[2,[3,[4,[5,6,7]]]],8,9]
[1,[2,[3,[4,[5,6,0]]]],8,9]";

    #[test]
    fn test_day12_part1() {
        let input = parse_input(DAY13_EXAMPLE);
        assert_eq!(sum_packets_in_order(&input), 13);
    }
}
