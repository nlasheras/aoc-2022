use aoc_runner_derive::aoc;
use aoc_runner_derive::aoc_generator;
use std::collections::VecDeque;

// ore, clay, obsidian, geode
type Minerals = (i32, i32, i32, i32);

#[derive(Debug)]
pub struct Robot {
    pub cost: Minerals,
    pub out: Minerals,
}

impl Robot {
    fn cost_from(input: &str) -> Minerals {
        let costs = input.split(" and ").collect::<Vec<&str>>();
        let mut cost = (0, 0, 0, 0);
        for c in costs {
            let mineral = Self::out_from(c);
            let amount = c.split(" ").nth(0).unwrap().parse::<i32>().unwrap();
            cost = (
                cost.0 + mineral.0 * amount,
                cost.1 + mineral.1 * amount,
                cost.2 + mineral.2 * amount,
                cost.3 + mineral.3 * amount,
            )
        }
        cost
    }

    fn out_from(input: &str) -> Minerals {
        let mineral = input.split(" ").nth(1).unwrap();
        match mineral {
            "ore" => (1, 0, 0, 0),
            "clay" => (0, 1, 0, 0),
            "obsidian" => (0, 0, 1, 0),
            "geode" => (0, 0, 0, 1),
            _ => panic!("Wrong mineral {}", mineral),
        }
    }

    fn from(input: &str) -> Robot {
        let parts = input.split(" costs ").collect::<Vec<&str>>();
        let tmp = parts[0].replace(" Each", "1").replace("Each", "1");
        Robot {
            cost: Self::cost_from(parts[1]),
            out: Self::out_from(&tmp),
        }
    }
}

pub struct Blueprint {
    pub id: i32,
    pub robots: Vec<Robot>,
}

#[derive(Clone, Copy, Debug, PartialEq)]
struct State {
    pub inventory: Minerals,
    pub robots: Minerals,
    pub time: i32,
}

impl State {
    pub fn tick(&mut self) -> () {
        self.inventory = (
            self.inventory.0 + self.robots.0,
            self.inventory.1 + self.robots.1,
            self.inventory.2 + self.robots.2,
            self.inventory.3 + self.robots.3,
        );
        self.time += 1;
    }

    #[allow(dead_code)]
    pub fn value(&self, time: i32) -> u64 {
        let remaining = time - self.time;
        ((self.inventory.0 + self.robots.0 * remaining) as u64
            + (self.inventory.1 + self.robots.1 * remaining) as u64 * 10u64
            + (self.inventory.2 + self.robots.2 * remaining) as u64 * 100u64
            + (self.inventory.3 + self.robots.3 * remaining) as u64 * 10000u64) as u64
    }
}

impl Blueprint {
    fn from(input: &str) -> Blueprint {
        let name = input.split(": ").nth(0).unwrap();
        let id = name.replace("Blueprint ", "").parse::<i32>().unwrap();
        let recipes = input.split(": ").nth(1).unwrap();
        let robots = recipes
            .split(".")
            .filter(|s| !s.is_empty())
            .map(Robot::from)
            .collect::<Vec<Robot>>();
        Blueprint {
            id: id,
            robots: robots,
        }
    }

    fn can_reach(robots: &Minerals, cost: &Minerals) -> bool {
        (cost.0 == 0 || robots.0 > 0)
            && (cost.1 == 0 || robots.1 > 0)
            && (cost.2 == 0 || robots.2 > 0)
            && (cost.3 == 0 || robots.3 > 0)
    }

    fn can_pay(inventory: &Minerals, cost: &Minerals) -> bool {
        inventory.0 >= cost.0
            && inventory.1 >= cost.1
            && inventory.2 >= cost.2
            && inventory.3 >= cost.3
    }

    fn sub(inventory: &Minerals, cost: &Minerals) -> Minerals {
        (
            inventory.0 - cost.0,
            inventory.1 - cost.1,
            inventory.2 - cost.2,
            inventory.3 - cost.3,
        )
    }

    fn add(inventory: &Minerals, cost: &Minerals) -> Minerals {
        (
            inventory.0 + cost.0,
            inventory.1 + cost.1,
            inventory.2 + cost.2,
            inventory.3 + cost.3,
        )
    }

    fn too_much(inventory: &Minerals, robot: &Minerals, _time: i32, max: &Minerals) -> bool {
        (robot.0 > 0 && inventory.0 > max.0 * 2)
            || (robot.1 > 0 && inventory.1 > max.1 * 2)
            || (robot.2 > 0 && inventory.2 > max.2 * 2)
    }

    pub fn largest_geode(&self, time: i32) -> u64 {
        let start = State {
            inventory: (0, 0, 0, 0),
            robots: (1, 0, 0, 0),
            time: 0,
        };
        let mut closed_states = Vec::new();
        let mut states = VecDeque::new();
        let mut max = (0, 0, 0, 0);
        self.robots.iter().for_each(|r| {
            max = (
                std::cmp::max(max.0, r.cost.0),
                std::cmp::max(max.1, r.cost.1),
                std::cmp::max(max.2, r.cost.2),
                std::cmp::max(max.3, r.cost.3),
            )
        });
        states.push_back(start);
        while !states.is_empty() {
            let s = states.pop_front().unwrap();

            let mut new_states = Vec::new();
            for r in self.robots.iter().rev() {
                if Self::can_reach(&s.robots, &r.cost) {
                    if Self::too_much(&s.inventory, &r.out, time, &max) {
                        continue;
                    }
                    let mut new = s.clone();
                    while !Self::can_pay(&new.inventory, &r.cost) {
                        new.tick();
                    }
                    new.tick();
                    new.inventory = Self::sub(&new.inventory, &r.cost);
                    new.robots = Self::add(&new.robots, &r.out);
                    if new.time < time {
                        new_states.push(new);
                    }
                }
            }

            closed_states.push(s);
            for s in new_states.iter() {
                states.push_back(*s);
            }
        }

        closed_states.retain(|s| s.robots.3 > 0);
        if closed_states.is_empty() {
            return 0;
        }

        for s in closed_states.iter_mut() {
            while s.time < time {
                s.tick();
            }
        }

        closed_states.sort_by(|s1, s2| s2.inventory.3.cmp(&s1.inventory.3));
        closed_states.iter().nth(0).unwrap().inventory.3 as u64
    }
}

#[aoc_generator(day19)]
pub fn parse_input(input: &str) -> Vec<Blueprint> {
    input.lines().map(Blueprint::from).collect()
}

#[aoc(day19, part1)]
fn sum_quality_levels(input: &Vec<Blueprint>) -> u64 {
    input
        .iter()
        .fold(0, |sum, bp| sum + bp.largest_geode(24) * bp.id as u64)
}

#[aoc(day19, part2)]
fn mul_largest_geodes(input: &Vec<Blueprint>) -> u64 {
    let nums = input[0..3]
        .iter()
        .map(|bp| bp.largest_geode(32))
        .collect::<Vec<u64>>();
    nums.iter().product()
}

#[cfg(test)]
mod tests {
    use super::*;

    const DAY19_EXAMPLE: &str = "Blueprint 1: Each ore robot costs 4 ore. Each clay robot costs 2 ore. Each obsidian robot costs 3 ore and 14 clay. Each geode robot costs 2 ore and 7 obsidian.
Blueprint 2: Each ore robot costs 2 ore. Each clay robot costs 3 ore. Each obsidian robot costs 3 ore and 8 clay. Each geode robot costs 3 ore and 12 obsidian.";

    #[test]
    fn test_day19_simulate1() {
        let input = parse_input(DAY19_EXAMPLE);
        let bp1 = &input[0];
        assert_eq!(bp1.largest_geode(24), 9);
    }

    #[test]
    fn test_day19_simulate2() {
        let input = parse_input(DAY19_EXAMPLE);
        let bp2 = &input[1];
        assert_eq!(bp2.largest_geode(24), 12);
    }

    #[test]
    fn test_day19_part1() {
        let input = parse_input(DAY19_EXAMPLE);
        assert_eq!(sum_quality_levels(&input), 33);
    }
}
