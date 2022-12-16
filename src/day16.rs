use aoc_runner_derive::aoc;
use aoc_runner_derive::aoc_generator;
use petgraph::graph::Node;
use petgraph::graph::NodeIndex;
use petgraph::Graph;
use std::collections::HashMap;
use priority_queue::DoublePriorityQueue;
use std::cmp;
use std::collections::BTreeSet;

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

fn print_path(graph: &Graph<(String, i32), i32>, path: &Vec<NodeIndex>) -> String {
    let mut buffer = "".to_string();
    for n in path {
        if buffer.len() == 0 {
            buffer.push('[')
        }
        if buffer.len() > 1 {
            buffer.push(',');
        }
        let (name, _) = graph.node_weight(*n).unwrap();
        buffer.push_str(name)
    }
    if buffer.len() > 0 { 
        buffer.push(']');
    }
    buffer
}

#[derive(Copy, Clone, PartialEq, Eq, Ord)]
struct Priority
{
    pub steps : usize,
    pub flow: i32
}

impl PartialOrd for Priority {
    fn partial_cmp(&self, other: &Priority) -> Option<cmp::Ordering> {
        if self.flow > other.flow {
            return Some(cmp::Ordering::Less);
        }
        else if self.flow < other.flow {
            return Some(cmp::Ordering::Greater);
        }
        self.steps.partial_cmp(&other.steps)
    }
}

fn path_complete(graph: &Graph<(String, i32), i32>, path: &Vec<NodeIndex>) -> bool {
    graph.node_indices().all(|i| path.contains(&i))
}

fn is_open(path: &Vec<NodeIndex>, n: &NodeIndex) -> bool {
    path.windows(2).any(|w| w[0] == *n && w[0] == w[1])
}

fn find_best_path(graph: &Graph<(String, i32), i32>) -> u64 {
    let mut candidates = DoublePriorityQueue::new();

    let mut _c = 0;
    let aa = graph.node_indices().nth(0).unwrap();
    candidates.push(vec![aa], Priority{steps: 0, flow: 0});

    println!("Start!");
    while !candidates.is_empty() {
        _c += 1;

        // get the path with the biggest
        let (path, _priority) = candidates.pop_min().unwrap();
        
        if _c % 100000 == 0 {
            println!("evaluating {} {} ({}/{}) {}", _c, print_path(graph, &path), _priority.steps, _priority.flow, candidates.len());
        }
        
        if path_complete(graph, &path) {
            println!("evaluating {} ({}/{})", print_path(graph, &path), _priority.steps, _priority.flow);
            println!(" this is best!");
            return 1;
        }
        else
        {
            let last = path[path.len() - 1];
            for n in graph.neighbors(last) {
                let w = graph.node_weight(n).unwrap();
                let mut new_path = Vec::from(path.clone());
                new_path.push(n);


                if !is_open(&path, &n) && _priority.steps < 28 && w.1 > 0 {
                    let flow = _priority.flow + w.1 * (28 - _priority.steps) as i32;
                    let mut new_path2 = Vec::from(new_path.clone());
                    new_path2.push(n);
                    candidates.push(new_path2, Priority{ steps: _priority.steps+2, flow: flow});
                }
                candidates.push(new_path, Priority{ steps: _priority.steps+1, flow: _priority.flow});

            }
        }
    }
    
    0
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
