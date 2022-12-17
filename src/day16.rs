use aoc_runner_derive::aoc;
use aoc_runner_derive::aoc_generator;
use petgraph::graph::NodeIndex;
use petgraph::Graph;
use petgraph::algo::dijkstra;
use std::collections::HashMap;
use std::cmp;

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
            edges.push((src.clone(), dest.clone()));
        }
    }
    graph.extend_with_edges(&edges);
    graph
}


type Paths = HashMap<NodeIndex, HashMap<NodeIndex, i32>>;

fn find_max(_graph: &Graph<(String, i32), i32>, weights: &HashMap<NodeIndex, i32>, paths: &Paths, current: &NodeIndex, steps: i32,  valves_to_open:&Vec<NodeIndex>) -> u32 {
    if valves_to_open.is_empty() {
        return 0;
    }

    let mut max : u32 = 0;
    for n in valves_to_open.iter() {
        let w = weights.get(n).unwrap();
    
        let steps_to_go = paths.get(current).unwrap().get(n).unwrap();

        if steps > *steps_to_go {
            let remaining = steps - *steps_to_go - 1;
            let flow = remaining * w;
            max = cmp::max(max, flow as u32);
            let mut tmp = valves_to_open.clone();
            tmp.retain(|v| *v != *n);
            let sub = find_max(_graph, weights, paths, n, remaining, &tmp);
            max = cmp::max(max, flow as u32 + sub);
        }
    }
    max 
}

fn find_best_path(graph: &Graph<(String, i32), i32>) -> u64 {
    let mut valves_to_open = Vec::new();
    let mut all_paths = Paths::new();
    let mut all_weights = HashMap::<NodeIndex, i32>::new();
    graph.node_indices().for_each(|i| {
        let w = graph.node_weight(i).unwrap();
        let paths = dijkstra(graph, i, None, |_| 1);
        all_paths.insert(i, paths);
        if w.1 > 0 {
            valves_to_open.push(i);
            all_weights.insert(i, w.1);
        }
    });

    println!("Need to open {} valves", valves_to_open.len());
    let start = graph.node_indices().find(|i| graph.node_weight(*i).unwrap().0 == "AA").unwrap();
    find_max(graph, &all_weights, &all_paths, &start, 30, &valves_to_open) as u64
}

#[aoc(day16, part1)]
pub fn find_most_pressure(input: &Graph<(String, i32), i32>) -> u64 {
    find_best_path(input)
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
