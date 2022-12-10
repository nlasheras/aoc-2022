use aoc_runner_derive::aoc;
use aoc_runner_derive::aoc_generator;

#[aoc_generator(day10)]
pub fn parse_input(input: &str) -> Vec<(String, i64, i64)> {
    input
        .lines()
        .map(|s| {
            let parts : Vec<&str> = s.split(" ").collect();
            if parts[0] == "noop" {
                return ("noop".to_string(), 0, 1)
            }
            (String::from(parts[0]), parts[1].parse::<i64>().unwrap(), 2)
        })
        .collect()
}

fn get_x_value(instructions: &Vec<(String, i64, i64)>, interesting: &Vec<i64>) -> Vec<i64> {
    let mut ret = Vec::new();
    for cycle in interesting {
        let mut sum = 1;
        for i in 0..instructions.len() {
            let instruction_cycles = instructions[i].2;
            if sum + instruction_cycles > *cycle {

                let mut x = 1;
                for j in 0..i {
                    x += instructions[j].1;
                }

                ret.push(x);
                break;
            }
            sum += instruction_cycles;
        }
    }
    ret
}

#[aoc(day10, part1)]
fn sum_six_signal_strengths(instructions: &Vec<(String, i64, i64)>) -> i64 {
    let interesting = vec![20, 60, 100, 140, 180, 220];
    let xs = get_x_value(instructions, &interesting);
    let mut sum = 0;
    for i in 0..interesting.len() {
        sum += xs[i] * interesting[i];
    }
    sum
}

#[aoc(day10, part2)]
fn render_crt(instructions: &Vec<(String, i64, i64)>) -> String {
    let xs = get_x_value(instructions, &(0..=240).collect::<Vec<i64>>());
    let mut ret = String::from("");
    for i in 1..=240 {
        let x = xs[i];
        let col = ((i - 1) % 40) as i64;
        if (x - col).abs() <= 1 {
            ret.push('#');
        }
        else {
            ret.push('.');
        }
        
        if i % 40 == 0 && i != 240 { ret.push('\n') };
    }
    println!("{}", ret);
    ret
}


#[cfg(test)]
mod tests {
    use super::*;

    const DAY10_EXAMPLE: &str = "addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop";

    #[test]
    fn test_day10_part1() {
        let input = parse_input(DAY10_EXAMPLE);
        assert_eq!(sum_six_signal_strengths(&input), 13140);
    }

    const DAY10_PART2_OUTPUT : &str = "##..##..##..##..##..##..##..##..##..##..
###...###...###...###...###...###...###.
####....####....####....####....####....
#####.....#####.....#####.....#####.....
######......######......######......####
#######.......#######.......#######.....";

    #[test]
    fn test_day10_part2() {
        let input = parse_input(DAY10_EXAMPLE);
        let output = render_crt(&input);
        assert_eq!(output.as_str(), DAY10_PART2_OUTPUT); 
    }



}