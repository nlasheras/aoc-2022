use aoc_runner_derive::aoc;
use aoc_runner_derive::aoc_generator;

use eval::eval;

#[derive(Clone)]
pub struct Test
{
    divisible: u64,
    if_true: usize,
    if_false: usize
}

#[derive(Clone)]
pub struct Monkey
{
    pub items: Vec<u64>,
    pub operation: String, 
    pub test: Test,
}

impl Monkey {
    fn from(input: &str) -> Monkey {
        let lines = input.lines().skip(1).collect::<Vec<&str>>();
        let items = lines[0].split("Starting items: ").nth(1).unwrap().split(", ").map(|s| s.parse::<u64>().unwrap()).collect::<Vec<u64>>();
        let operation = lines[1].split("Operation: ").nth(1).unwrap().split("new = ").nth(1).unwrap();
        let divisible_by = lines[2].split("divisible by ").nth(1).unwrap().parse::<u64>().unwrap();
        let if_true = lines[3].split("throw to monkey ").nth(1).unwrap().parse::<usize>().unwrap();
        let if_false = lines[4].split("throw to monkey ").nth(1).unwrap().parse::<usize>().unwrap();
        Monkey {
            items: items,
            operation: operation.to_string(),
            test: Test{divisible: divisible_by, if_true: if_true, if_false: if_false}
        }
    }

    fn operation(&self, old: u64) -> u64 {
        let tmp = self.operation.replace("old", &old.to_string());
        eval(&tmp).unwrap().as_u64().unwrap()
    }

    pub fn inspect(&self, item: u64) -> u64 {
        self.operation(item)
    }

    pub fn who_to_throw(&self, item: u64) -> usize {
        if item % self.test.divisible == 0 {
            return self.test.if_true;
        }
        self.test.if_false
    }

}

#[aoc_generator(day11)]
pub fn parse_input(input: &str) -> Vec<Monkey> {
    input.split("\n\n").map(|s| Monkey::from(s)).collect()
}

#[aoc(day11, part1)]
fn get_monkey_business_after_20rounds(input: &Vec<Monkey>) -> u64 {
    let rounds = 20;
    let mut monkeys = input.clone();
    let mut throws = vec![0;monkeys.len()];
    for _ in 0..rounds {
        for i in 0..monkeys.len() {
            let mut items = Vec::new();
            {
                let mut current = &mut monkeys[i];
                items = current.items.clone();
                current.items = Vec::new();
                throws[i] += items.len();
            }
            for item in items {
                let new = monkeys[i].inspect(item) / 3;
                let throw_at = monkeys[i].who_to_throw(new);
                
                let other = &mut monkeys[throw_at];
                other.items.push(new);
            }
        }
    }
    throws.sort();
    (throws[throws.len()-2] * throws[throws.len()-1]) as u64
}

#[aoc(day11, part2)]
fn get_monkey_business_after_10krounds(input: &Vec<Monkey>) -> u64 {
    let rounds = 10_000;
    let mut monkeys = input.clone();
    let mut throws = vec![0u64;monkeys.len()];
    let mut mcd = 1;
    for m in input.iter() {
        mcd *= m.test.divisible;
    }

    for r in 0..rounds {
        for i in 0..monkeys.len() {
            let mut items = Vec::new();
            {
                let mut current = &mut monkeys[i];
                items = current.items.clone();
                current.items = Vec::new();
                throws[i] += items.len() as u64;
            }
            for item in items {
                let new = monkeys[i].inspect(item) % mcd;
                let throw_at = monkeys[i].who_to_throw(new);
                
                let other = &mut monkeys[throw_at];
                other.items.push(new);
            }
        }
    }
    throws.sort();
    throws[throws.len()-2] * throws[throws.len()-1]
}

#[cfg(test)]
mod tests {
    use super::*;

    const DAY11_EXAMPLE : &str = "Monkey 0:
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