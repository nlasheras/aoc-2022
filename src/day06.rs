use aoc_runner_derive::aoc;
use aoc_runner_derive::aoc_generator;

#[aoc_generator(day6)]
pub fn parse_input(input: &str) -> String {
    input.to_string()
}

fn repeated_char(window: &[char]) -> Option<char> {
    for i in 0..window.len() - 1 {
        for j in i + 1..window.len() {
            if window[i] == window[j] {
                return Some(window[i]);
            }
        }
    }
    return None;
}

fn find_start_marker(input: &str, window_size: usize) -> u64 {
    input
        .chars()
        .collect::<Vec<char>>()
        .windows(window_size)
        .position(|w| repeated_char(w).is_none())
        .expect("Marker not found") as u64
        + window_size as u64
}

#[aoc(day6, part1)]
pub fn find_first_start_package(input: &str) -> u64 {
    find_start_marker(input, 4)
}

#[aoc(day6, part2)]
pub fn find_first_start_message(input: &str) -> u64 {
    find_start_marker(input, 14)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day6_example() {
        assert_eq!( find_first_start_package("mjqjpqmgbljsphdztnvjfqwrcgsmlb"), 7);
    }

    #[test]
    fn test_day6_extra_examples() {
        assert_eq!(find_first_start_package("bvwbjplbgvbhsrlpgdmjqwftvncz"), 5);
        assert_eq!(find_first_start_package("nppdvjthqldpwncqszvftbrmjlhg"), 6);
        assert_eq!(find_first_start_package("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"), 10);
        assert_eq!(find_first_start_package("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"), 11);
    }

    #[test]
    fn test_day6_part2() {
        assert_eq!(find_first_start_message("mjqjpqmgbljsphdztnvjfqwrcgsmlb"), 19);
    }

    #[test]
    fn test_day6_part2_extra_examples() {
        assert_eq!(find_first_start_message("bvwbjplbgvbhsrlpgdmjqwftvncz"), 23);
        assert_eq!(find_first_start_message("nppdvjthqldpwncqszvftbrmjlhg"), 23);
        assert_eq!(find_first_start_message("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"), 29);
        assert_eq!(find_first_start_message("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"), 26);
    }
}
