use aoc_runner_derive::aoc;
use aoc_runner_derive::aoc_generator;
use std::fmt;

#[derive(Debug)]
pub struct SnafuNumber {
    digits: Vec<char>,
}

impl SnafuNumber {
    fn from(input: &str) -> SnafuNumber {
        SnafuNumber {
            digits: input.chars().rev().collect(),
        }
    }

    fn sum_of_digits(n: i64) -> i64 {
        let mut pow = 1;
        let mut sum = 0;
        for _ in 0..n {
            sum += 2 * pow;
            pow *= 5;
        }
        sum
    }

    fn pos(n: i64) -> i64 {
        5_i64.pow(n as u32)
    }

    fn from_u64(input: u64) -> SnafuNumber {
        let mut digits = Vec::new();
        let mut num_digits = 1_i64;
        let mut remaining = input as i64;
        while Self::sum_of_digits(num_digits) < remaining {
            num_digits += 1
        }
        for index in (0..num_digits).rev() {
            let next_positions = Self::sum_of_digits(index);
            let position = Self::pos(index);
            let n = if remaining >= 0 {
                (remaining + next_positions) / position
            } else {
                (remaining - next_positions) / position
            };
            let d = match n {
                2 => '2',
                1 => '1',
                0 => '0',
                -1 => '-',
                -2 => '=',
                _ => panic!("Wrong digit! {n}"),
            };
            remaining -= position * n;
            digits.push(d);
        }
        digits.reverse();
        SnafuNumber { digits }
    }

    fn as_u64(&self) -> u64 {
        let mut value = 0i64;
        let mut pos = 1;
        for d in self.digits.iter() {
            value += match d {
                '2' => 2,
                '1' => 1,
                '0' => 0,
                '-' => -1,
                '=' => -2,
                _ => panic!("Wrong digit"),
            } * pos;
            pos *= 5;
        }
        value as u64
    }
}

impl fmt::Display for SnafuNumber {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.digits.iter().rev().collect::<String>())
    }
}

#[aoc_generator(day25)]
pub fn parse_input(input: &str) -> Vec<SnafuNumber> {
    input.lines().map(SnafuNumber::from).collect()
}

#[aoc(day25, part1)]
pub fn sum_all_numbers(input: &[SnafuNumber]) -> String {
    let result = input.iter().map(SnafuNumber::as_u64).sum();
    SnafuNumber::from_u64(result).to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    const DAY25_EXAMPLE: &str = "1=-0-2
12111
2=0=
21
2=01
111
20012
112
1=-1=
1-12
12
1=
122";

    #[test]
    fn test_day25_snafu_as_u64() {
        let n = SnafuNumber::from("1=-0-2");
        assert_eq!(n.as_u64(), 1747);
    }

    #[test]
    fn test_day25_snafu_to_string() {
        let n = SnafuNumber::from_u64(976);
        assert_eq!(n.to_string(), "2=-01");
    }

    #[test]
    fn test_day25_snafu_to_string_test() {
        let n = SnafuNumber::from_u64(20);
        assert_eq!(n.to_string(), "1-0");
    }

    #[test]
    fn test_day25_snafu_to_string_11() {
        let n = SnafuNumber::from_u64(11);
        assert_eq!(n.to_string(), "21");
    }

    #[test]
    fn test_day25_snafu_to_string_2022() {
        let n = SnafuNumber::from_u64(2022);
        assert_eq!(n.to_string(), "1=11-2");
    }

    #[test]
    fn test_day25_snafu_to_string_314159265() {
        let n = SnafuNumber::from_u64(314159265);
        assert_eq!(n.to_string(), "1121-1110-1=0");
    }

    #[test]
    fn test_day25_snafu_to_string_all() {
        let input = parse_input(DAY25_EXAMPLE);
        for n in input {
            let value = n.as_u64();
            assert_eq!(SnafuNumber::from_u64(value).to_string(), n.to_string());
        }
    }

    #[test]
    fn test_day25_part1() {
        let input = parse_input(DAY25_EXAMPLE);
        assert_eq!(sum_all_numbers(&input), "2=-1=0");
    }
}
