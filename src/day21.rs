use aoc_runner_derive::aoc;
use aoc_runner_derive::aoc_generator;
use std::collections::HashMap;

#[derive(Clone, Debug, PartialEq)]
pub enum Monkey {
    Number(i64),
    Add(String, String),
    Sub(String, String),
    Mul(String, String),
    Div(String, String),
}

impl Monkey {
    fn from(input: &str) -> Monkey {
        if let Ok(number) = input.parse::<i64>() {
            return Monkey::Number(number);
        }
        let parts = input.split(" ").collect::<Vec<&str>>();
        match parts[1] {
            "+" => Monkey::Add(parts[0].to_string(), parts[2].to_string()),
            "-" => Monkey::Sub(parts[0].to_string(), parts[2].to_string()),
            "*" => Monkey::Mul(parts[0].to_string(), parts[2].to_string()),
            "/" => Monkey::Div(parts[0].to_string(), parts[2].to_string()),
            _ => panic!("Wrong op {}", parts[1]),
        }
    }

    pub fn branches(&self) -> (&String, &String) {
        match self {
            Monkey::Number(_) => panic!("Not a operation"),
            Monkey::Add(k1, k2) => (k1, k2),
            Monkey::Sub(k1, k2) => (k1, k2),
            Monkey::Mul(k1, k2) => (k1, k2),
            Monkey::Div(k1, k2) => (k1, k2),
        }
    }
}

type Monkeys = HashMap<String, Monkey>;

#[aoc_generator(day21)]
pub fn parse_input(input: &str) -> Monkeys {
    let mut map = HashMap::new();
    input.lines().for_each(|s| {
        let parts = s.split(": ").collect::<Vec<&str>>();
        map.insert(parts[0].to_string(), Monkey::from(parts[1]));
    });
    map
}

fn get_monkey(map: &Monkeys, key: &str) -> i64 {
    let monkey = &map[key];
    match monkey {
        Monkey::Number(n) => *n,
        Monkey::Add(k1, k2) => get_monkey(map, &k1) + get_monkey(map, &k2),
        Monkey::Sub(k1, k2) => get_monkey(map, &k1) - get_monkey(map, &k2),
        Monkey::Mul(k1, k2) => get_monkey(map, &k1) * get_monkey(map, &k2),
        Monkey::Div(k1, k2) => get_monkey(map, &k1) / get_monkey(map, &k2),
    }
}

#[aoc(day21, part1)]
pub fn get_root(input: &Monkeys) -> i64 {
    get_monkey(input, "root")
}

fn has_monkey(input: &Monkeys, root: &str, key: &str) -> bool {
    if key == root {
        return true;
    }
    let monkey = &input[root];
    match monkey {
        Monkey::Number(_) => false,
        Monkey::Add(k1, k2) | Monkey::Div(k1, k2) | Monkey::Sub(k1, k2) | Monkey::Mul(k1, k2) => {
            has_monkey(input, k1, key) || has_monkey(input, k2, key)
        }
    }
}

#[aoc(day21, part2)]
pub fn get_human(input: &Monkeys) -> i64 {
    let root = &input["root"];
    let (a, b) = root.branches();
    let mut value;
    let mut branch;
    if has_monkey(input, a, "humn") {
        value = get_monkey(input, b);
        branch = a;
    } else {
        value = get_monkey(input, a);
        branch = b;
    }

    while branch != "humn" {
        let monkey = &input[branch];
        match monkey {
            Monkey::Number(_) => panic!("Shouldn't happen"),
            Monkey::Add(k1, k2)
            | Monkey::Sub(k1, k2)
            | Monkey::Mul(k1, k2)
            | Monkey::Div(k1, k2) => {
                let branch_value;
                if has_monkey(input, k1, "humn") {
                    branch = k1;
                    branch_value = get_monkey(input, k2);
                } else {
                    branch = k2;
                    branch_value = get_monkey(input, k1);
                };
                match monkey {
                    Monkey::Add(_, _) => value -= branch_value,
                    Monkey::Sub(left, _) => {
                        if has_monkey(input, left, "humn") {
                            value += branch_value
                        } else {
                            value = branch_value - value;
                        }
                    }
                    Monkey::Mul(_, _) => value /= branch_value,
                    Monkey::Div(left, _) => {
                        if has_monkey(input, left, "humn") {
                            value *= branch_value
                        } else {
                            value = branch_value / value;
                        }
                    }
                    _ => panic!("Shouldn't happen"),
                }
            }
        }
    }
    value
}

#[cfg(test)]
mod tests {
    use super::*;

    const DAY21_EXAMPLE: &str = "root: pppw + sjmn
dbpl: 5
cczh: sllz + lgvd
zczc: 2
ptdq: humn - dvpt
dvpt: 3
lfqf: 4
humn: 5
ljgn: 2
sjmn: drzm * dbpl
sllz: 4
pppw: cczh / lfqf
lgvd: ljgn * ptdq
drzm: hmdt - zczc
hmdt: 32";

    #[test]
    fn test_day21_part1() {
        let input = parse_input(DAY21_EXAMPLE);
        assert_eq!(get_root(&input), 152);
    }

    #[test]
    fn test_day21_part2() {
        let input = parse_input(DAY21_EXAMPLE);
        assert_eq!(get_human(&input), 301);
    }
}
