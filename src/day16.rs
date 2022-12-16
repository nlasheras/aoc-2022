use aoc_runner_derive::aoc;
use aoc_runner_derive::aoc_generator;
use petgraph::graph::NodeIndex;
use petgraph::Graph;
use std::collections::HashMap;

#[aoc_generator(day16)]
pub fn parse_input(input: &str) -> Graph<(String, i32), i32> {
    let lines = input
        .lines()
        .map(|s| {
            // Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
            let parts = s.split("; ").collect::<Vec<&str>>();
            let valve = parts[0]
                .split(" has ")
                .collect::<Vec<&str>>()
                .first()
                .unwrap()
                .replace("Valve ", "");
            let flow = parts[0]
                .split("=")
                .collect::<Vec<&str>>()
                .last()
                .unwrap()
                .parse::<i32>()
                .unwrap();
            let tunnels = parts[1]
                .replace("tunnels lead to valves ", "")
                .replace("tunnel leads to valve ", "")
                .split(", ")
                .collect::<Vec<&str>>()
                .iter()
                .map(|&s| s.into())
                .collect();
            (valve, flow, tunnels)
        })
        .collect::<Vec<(String, i32, Vec<String>)>>();

    let mut tmp = HashMap::<String, NodeIndex>::new();
    let mut graph = Graph::<(String, i32), i32>::new();
    for l in lines.iter() {
        let id = graph.add_node((String::from(&l.0), l.1));
        tmp.insert(String::from(&l.0), id);
    }
    let mut edges = Vec::new();
    for l in lines.iter() {
        let src = tmp.get(&l.0).unwrap();
        for tunnel in &l.2 {
            let dest = tmp.get(tunnel).unwrap();
            edges.push((src, dest));
        }
    }
    graph
}

#[aoc(day16, part1)]
pub fn find_most_pressure(input: &Graph<(String, i32), i32>) -> u64 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    const DAY16_EXAMPLE: &str = "Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
Valve BB has flow rate=13; tunnels lead to valves CC, AA
Valve CC has flow rate=2; tunnels lead to valves DD, BB
Valve DD has flow rate=20; tunnels lead to valves CC, AA, EE
Valve EE has flow rate=3; tunnels lead to valves FF, DD
Valve FF has flow rate=0; tunnels lead to valves EE, GG
Valve GG has flow rate=0; tunnels lead to valves FF, HH
Valve HH has flow rate=22; tunnel leads to valve GG
Valve II has flow rate=0; tunnels lead to valves AA, JJ
Valve JJ has flow rate=21; tunnel leads to valve II";

    #[test]
    fn test_day16_part1() {
        let input = parse_input(DAY16_EXAMPLE);
        assert_eq!(find_most_pressure(&input), 1651);
    }
}
