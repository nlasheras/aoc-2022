use aoc_runner_derive::aoc;
use aoc_runner_derive::aoc_generator;

#[derive(Clone)]
pub struct Monkey {
    pub items: Vec<u64>,
    pub operation: Operation,
    pub test: Test,
}

impl Monkey {
    fn from(input: &str) -> Monkey {
        let lines = input.lines().skip(1).collect::<Vec<&str>>();
        let items = lines[0]
            .split("Starting items: ")
            .nth(1)
            .unwrap()
            .split(", ")
            .map(|s| s.parse::<u64>().unwrap())
            .collect::<Vec<u64>>();
        Monkey {
            items: items,
            operation: Operation::from(lines[1]),
            test: Test::from(&lines[2..=4]),
        }
    }

    pub fn inspect(&self, item: u64) -> u64 {
        self.operation.eval(item)
    }

    pub fn who_to_throw(&self, item: u64) -> usize {
        if item % self.test.divisible == 0 {
            return self.test.if_true;
        }
        self.test.if_false
    }
}

#[derive(Clone)]
pub enum Operation {
    Add(u64),
    Mul(u64),
    Pow,
}

impl Operation {
    fn from(input: &str) -> Operation {
        let expression = input
            .split("Operation: ")
            .nth(1)
            .unwrap()
            .split("new = ")
            .nth(1)
            .unwrap();
        if expression == "old * old" {
            return Operation::Pow;
        }
        assert!(expression.starts_with("old "));
        let parts = expression.split(" ").collect::<Vec<&str>>();
        let n = parts[2].parse::<u64>().unwrap();
        let operator = parts[1];
        match operator {
            "+" => Operation::Add(n),
            "*" => Operation::Mul(n),
            _ => panic!("Wrong operator!"),
        }
    }

    fn eval(&self, old: u64) -> u64 {
        match self {
            Operation::Pow => old * old,
            Operation::Add(n) => old + n,
            Operation::Mul(n) => old * n,
        }
    }
}

#[derive(Clone)]
pub struct Test {
    divisible: u64,
    if_true: usize,
    if_false: usize,
}

impl Test {
    fn from(input: &[&str]) -> Test {
        assert!(input.len() >= 3);
        Test {
            divisible: input[0]
                .split("divisible by ")
                .nth(1)
                .unwrap()
                .parse::<u64>()
                .unwrap(),
            if_true: input[1]
                .split("throw to monkey ")
                .nth(1)
                .unwrap()
                .parse::<usize>()
                .unwrap(),
            if_false: input[2]
                .split("throw to monkey ")
                .nth(1)
                .unwrap()
                .parse::<usize>()
                .unwrap(),
        }
    }
}

#[aoc_generator(day11)]
pub fn parse_input(input: &str) -> Vec<Monkey> {
    input.split("\n\n").map(|s| Monkey::from(s)).collect()
}

fn get_monkey_business(input: &Vec<Monkey>, rounds: i32, relief: u64) -> u64 {
    let mut monkeys = input.clone();
    let mut throws = vec![0u64; monkeys.len()];
    let mcd = monkeys
        .iter()
        .fold(1, |mcd, monkey| mcd * monkey.test.divisible);

    for _ in 0..rounds {
        for i in 0..monkeys.len() {
            let current = &mut monkeys[i];
            let items_to_throw = current.items.clone();
            current.items.clear();
            throws[i] += items_to_throw.len() as u64;
            for item in items_to_throw {
                let current = &monkeys[i]; // outer &mut cannot be used non-mutably
                let new = (current.inspect(item) / relief) % mcd;
                let throw_at = current.who_to_throw(new);

                let other = &mut monkeys[throw_at];
                other.items.push(new);
            }
        }
    }
    throws.sort();
    throws[throws.len() - 2] * throws[throws.len() - 1]
}

#[aoc(day11, part1)]
fn get_monkey_business_after_20rounds(input: &Vec<Monkey>) -> u64 {
    get_monkey_business(input, 20, 3)
}

#[aoc(day11, part2)]
fn get_monkey_business_after_10krounds(input: &Vec<Monkey>) -> u64 {
    get_monkey_business(input, 10_000, 1)
}

#[cfg(test)]
mod tests {
    use super::*;

    const DAY11_EXAMPLE: &str = "Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
  Starting items: 54, 65, 75, 74
  Operation: new = old + 6
  Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 2:
  Starting items: 79, 60, 97
  Operation: new = old * old
  Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

Monkey 3:
  Starting items: 74
  Operation: new = old + 3
  Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1";

    #[test]
    fn test_day11_part1() {
        let input = parse_input(DAY11_EXAMPLE);
        assert_eq!(get_monkey_business_after_20rounds(&input), 10605);
    }

    #[test]
    fn test_day11_part2() {
        let input = parse_input(DAY11_EXAMPLE);
        assert_eq!(get_monkey_business_after_10krounds(&input), 2713310158);
    }
}
