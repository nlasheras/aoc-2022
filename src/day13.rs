use aoc_runner_derive::aoc;
use aoc_runner_derive::aoc_generator;
use std::fmt;
use std::cmp;

#[derive(Clone, PartialEq)]
pub enum PacketData {
    Integer(i64),
    List(Vec<PacketData>),
}

impl std::fmt::Display for PacketData {
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
                    buffer.push_str(&format!("{}", packet));
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
            // List
            let mut values = Vec::new();
            let input = input.strip_prefix('[').unwrap().strip_suffix(']').unwrap();
            let mut start = 0;
            let mut level = 0;
            for (index, c) in input.char_indices() {
                match c {
                    '[' => level += 1,
                    ']' => level -= 1,
                    ',' => {
                        if level == 0 {
                            values.push(Self::from(&input[start..index]));
                            start = index+1;
                        }
                    }
                    _ => ()
                }
            }
            if start < input.len() {
                values.push(Self::from(&input[start..input.len()]));
            }
            PacketData::List(values)
        } else {
            // Integer
            PacketData::Integer(input.parse().unwrap())
        }
    }
    
    fn as_list(&self) -> PacketData {
        if let PacketData::Integer(_) = self {
            return PacketData::List(vec![self.clone()]);
        }
        self.clone()
    }

}

impl PartialOrd for PacketData {
    fn partial_cmp(&self, other: &PacketData) -> Option<cmp::Ordering> {
        match (self, other) {
            (PacketData::Integer(_), PacketData::List(_)) => return Self::partial_cmp(&self.as_list(), other),
            (PacketData::List(_), PacketData::Integer(_)) => return Self::partial_cmp(self, &other.as_list()),
            _=> ()
        }

        match (self, other) {
            (PacketData::Integer(n), PacketData::Integer(m)) => {
                n.partial_cmp(m)
            },
            (PacketData::List(left_list), PacketData::List(right_list)) => {
                for (left, right) in left_list.iter().zip(right_list.iter()) {
                    if let Some(comparison) = left.partial_cmp(right) {
                        if comparison == cmp::Ordering::Equal {
                            continue;
                        }
                        return Some(comparison);
                    }
                }
                if right_list.len() > left_list.len() {
                    // left list ran out of items
                    return Some(cmp::Ordering::Less);
                }
                if left_list.len() > right_list.len() {
                    // right list ran out of items
                    return Some(cmp::Ordering::Greater);
                }
                Some(cmp::Ordering::Equal)
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
      if pair.0.partial_cmp(&pair.1).unwrap() == cmp::Ordering::Less {
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

    input_packets.sort_by(|a, b| a.partial_cmp(b).unwrap());

    let idx1 = input_packets.iter().position(|p| p.partial_cmp(&divider1).unwrap() == cmp::Ordering::Equal).unwrap() + 1;
    let idx2 = input_packets.iter().position(|p| p.partial_cmp(&divider2).unwrap() == cmp::Ordering::Equal).unwrap() + 1;

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
        println!("a = {}", a);
        println!("b = {}", b);
        assert_eq!(a.partial_cmp(&b), Some(cmp::Ordering::Less));
    }

    #[test]
    fn test_day13_pair3() {
        let a = PacketData::from("[9]");
        let b = PacketData::from("[[8,7,6]]");
        assert_eq!(a.partial_cmp(&b), Some(cmp::Ordering::Greater));
    }

    #[test]
    fn test_day13_pair4() {
        let a = PacketData::from("[[4,4],4,4]");
        let b = PacketData::from("[[4,4],4,4,4]");
        assert_eq!(a.partial_cmp(&b), Some(cmp::Ordering::Less));
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
