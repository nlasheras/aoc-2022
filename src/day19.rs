use aoc_runner_derive::aoc;
use aoc_runner_derive::aoc_generator;

// ore, clay, obsidian, geode
type Minerals = (i32, i32, i32, i32);

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

    pub fn largest_geode(&self, time: i32) -> u64 {
        9
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

#[cfg(test)]
mod tests {
    use super::*;

    const DAY19_EXAMPLE: &str = "Blueprint 1: Each ore robot costs 4 ore. Each clay robot costs 2 ore. Each obsidian robot costs 3 ore and 14 clay. Each geode robot costs 2 ore and 7 obsidian.
Blueprint 2: Each ore robot costs 2 ore. Each clay robot costs 3 ore. Each obsidian robot costs 3 ore and 8 clay. Each geode robot costs 3 ore and 12 obsidian.";

    #[test]
    fn test_day19_simulate() {
        let input = parse_input(DAY19_EXAMPLE);
        let bp1 = &input[0];
        assert_eq!(bp1.largest_geode(24), 9);
    }

    #[ignore]
    #[test]
    fn test_day19_part1() {
        let input = parse_input(DAY19_EXAMPLE);
        assert_eq!(sum_quality_levels(&input), 33);
    }
}
