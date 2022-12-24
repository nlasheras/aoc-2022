use aoc_runner_derive::aoc;
use aoc_runner_derive::aoc_generator;

pub enum Instruction {
    Noop,
    Add(i64),
}

#[aoc_generator(day10)]
pub fn parse_input(input: &str) -> Vec<Instruction> {
    input
        .lines()
        .map(|s| {
            let parts: Vec<&str> = s.split(' ').collect();
            match parts[0] {
                "noop" => Instruction::Noop,
                "addx" => Instruction::Add(parts[1].parse::<i64>().unwrap()),
                _ => panic!("Wrong input"),
            }
        })
        .collect()
}

fn cycles_for_instruction(instruction: &Instruction) -> usize {
    match *instruction {
        Instruction::Noop => 1,
        Instruction::Add(_) => 2,
    }
}

fn get_x_values(instructions: &Vec<Instruction>, interesting: &Vec<usize>) -> Vec<i64> {
    let mut ret = Vec::new();
    assert!(interesting.windows(2).all(|w| w[0] < w[1])); // if the cycles are sorted, we can reuse some calculations
    let mut start_instruction = 0;
    let mut cycle_counter = 1;
    let mut x = 1;
    for cycle in interesting {
        for i in start_instruction..instructions.len() {
            let instruction_cycles = cycles_for_instruction(&instructions[i]);
            if cycle_counter + instruction_cycles > *cycle {
                x += instructions[start_instruction..i]
                    .iter()
                    .fold(0, |x, instruction| match instruction {
                        Instruction::Add(n) => x + n,
                        Instruction::Noop => x,
                    });

                ret.push(x);
                start_instruction = i;

                break;
            }
            cycle_counter += instruction_cycles;
        }
    }
    ret
}

#[aoc(day10, part1)]
fn sum_six_signal_strengths(instructions: &Vec<Instruction>) -> i64 {
    let interesting = vec![20, 60, 100, 140, 180, 220];
    let xs = get_x_values(instructions, &interesting);
    interesting
        .into_iter()
        .zip(xs.into_iter())
        .map(|pair| pair.0 as i64 * pair.1)
        .sum::<i64>()
}

#[aoc(day10, part2)]
fn render_crt(instructions: &Vec<Instruction>) -> String {
    let xs = get_x_values(instructions, &(1..=240).collect::<Vec<usize>>());
    let mut buffer = "".to_string();
    for i in 1..=240 {
        let x = xs[i - 1];
        let col = ((i - 1) % 40) as i64;

        if (x - col).abs() <= 1 {
            buffer.push('#');
        } else {
            buffer.push('.');
        }

        if col == 39 && i != 240 {
            //add newlines after column 40, 80
            buffer.push('\n')
        };
    }
    buffer
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

    const DAY10_PART2_OUTPUT: &str = "##..##..##..##..##..##..##..##..##..##..
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
