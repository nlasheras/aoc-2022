use aoc_runner_derive::aoc;
use aoc_runner_derive::aoc_generator;

use std::fmt;
use std::cmp;

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

    fn to_list(integer: &PacketData) -> PacketData {
        PacketData::List(vec![integer.clone()])
    }

    fn compare(&self, other: &PacketData) -> Option<bool> {
        let (left, right) = match (self, other) {
            (PacketData::Integer(_), PacketData::List(_)) => (Self::to_list(self), other.clone()),
            (PacketData::List(_), PacketData::Integer(_)) => (self.clone(), Self::to_list(other)),
            _=> (self.clone(), other.clone()),
            };

        match (left, right) {
            (PacketData::Integer(n), PacketData::Integer(m)) => {
                if n < m { 
                    return Some(true);
                }
                else if n > m {
                    return Some(false);
                }
                None
            },
            (PacketData::List(left_list), PacketData::List(right_list)) => {
                for i in 0..cmp::max(left_list.len(), right_list.len()) {
                    let elem_l = left_list.iter().nth(i);
                    let elem_r = right_list.iter().nth(i);
                    if let Some(left) = elem_l {
                        if let Some(right) = elem_r {
                            if let Some(comparison) = left.compare(right) {
                                if comparison == true {
                                    return Some(true);
                                }
                                else if comparison == false {
                                    return Some(false);
                                }
                            }
                        } else { 
                            // right list ran out of items
                            return Some(false);
                        }
                    } else if let Some(_) = elem_r {
                        // left list ran out of items
                        return Some(true);
                    }
                }
                None
            },
            _ => panic!("Shouldn't happen")
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
      if pair.0.compare(&pair.1).unwrap() {
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

#[aoc(day13, part2)]
fn locate_decoder_key(input: &Vec<(PacketData, PacketData)>) -> u64 {
    let mut input_packets = input.iter().map(|p| vec![p.0.clone(), p.1.clone()]).into_iter().flatten().collect::<Vec<PacketData>>();
    let divider1=  PacketData::from("[[2]]");
    let divider2 = PacketData::from("[[6]]");
    input_packets.push(divider1.clone());
    input_packets.push(divider2.clone());

    input_packets.sort_by(|a, b| 
        match a.compare(b) { 
            Some(true) => cmp::Ordering::Less,
            Some(false) => cmp::Ordering::Greater,
            None => cmp::Ordering::Equal
        });

    let idx1 = input_packets.iter().position(|p| p.compare(&divider1) == None).unwrap() + 1;
    let idx2 = input_packets.iter().position(|p| p.compare(&divider2) == None).unwrap() + 1;

    (idx1 * idx2) as u64
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
    fn test_day13_pair2() {
        let a = PacketData::from("[[1],[2,3,4]]");
        let b = PacketData::from("[[1],4]");
        assert!(a.compare(&b).unwrap());
    }

    #[test]
    fn test_day13_pair3() {
        let a = PacketData::from("[9]");
        let b = PacketData::from("[[8,7,6]]");
        assert!(!a.compare(&b).unwrap());
    }

    #[test]
    fn test_day13_pair4() {
        let a = PacketData::from("[[4,4],4,4]");
        let b = PacketData::from("[[4,4],4,4,4]");
        assert!(a.compare(&b).unwrap());
    }

    #[test]
    fn test_day13_part1() {
        let input = parse_input(DAY13_EXAMPLE);
        assert_eq!(sum_packets_in_order(&input), 13);
    }

    #[test]
    fn test_day13_part2() {
        let input = parse_input(DAY13_EXAMPLE);
        assert_eq!(locate_decoder_key(&input), 140);
    }
}
