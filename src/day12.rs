use aoc_runner_derive::aoc;
use aoc_runner_derive::aoc_generator;
use priority_queue::DoublePriorityQueue;
use std::collections::BTreeSet;
use std::collections::BTreeMap;

use crate::utils::Grid;

#[aoc_generator(day12)]
pub fn parse_input(input: &str) -> Grid<char> {
    let chars = input.lines().map(|s| s.chars().collect::<Vec<char>>()).collect::<Vec<Vec<char>>>();
    let width = chars.first().unwrap().len();
    let cells = chars.into_iter().flatten().collect::<Vec<char>>();
    Grid::new(&cells, width)
}

fn fix_start_end(a: char) -> char {
    match a {
        'S' => 'a',
        'E' => 'z',
        _ => a
    }
}
fn diff_height(current: char, other: char) -> i32 {
    (fix_start_end(other) as i32) - (fix_start_end(current) as i32)
}

fn find_path(grid: &Grid<char>, start: (i32, i32), end: (i32, i32)) -> Option<Vec<(i32, i32)>>  {
    let mut open_set = DoublePriorityQueue::new();
    open_set.push(start, 0);

    let mut closed_set = BTreeSet::new();
    let mut came_from = BTreeMap::new();

    let inf = u64::MAX;
    let mut dists = BTreeMap::new();
    dists.insert(start, 0u64);

    while !open_set.is_empty() {
        // get the element with smallest fScore
        let (current, _priority) = open_set.pop_min().unwrap();
        if closed_set.contains(&current) {
            continue;
        }

        if current == end {
            let mut path = vec![current];
            let mut path_node = current;
            while came_from.contains_key(&path_node) {
                path_node = *came_from.get(&path_node).unwrap();
                path.push(path_node);
            }
            path.reverse();
            return Some(path)
        } 

        closed_set.insert(current);

        let current_height = grid.cell_at(current.0, current.1).unwrap();
        for (candidate_height, cell_pos) in grid.neighbors_at(current.0, current.1) {
            let candidate = (cell_pos.0 as i32, cell_pos.1 as i32);
            if closed_set.contains(&candidate) {
                continue;
            }
            
            // apply constraint of either going up at most one (or downhill)
            if diff_height(current_height, candidate_height) > 1 {
                continue;
            }
            
            let dist_u = dists.entry(current).or_insert(inf).to_owned() + 1;

            let dist_v = dists.entry(candidate).or_insert(inf).to_owned();
            if dist_u < dist_v {
                *came_from.entry(candidate).or_insert(current) = current;

                dists.entry(candidate).and_modify(|e| { *e = dist_u }).or_insert(dist_u);
               
                open_set.push(candidate, _priority + 1 );
            }
        }
    }

    None
}

fn find_cell(value: char, input: &Grid<char>) -> (i32, i32) {
    let index = input.cells.iter().position(|c| *c == value).unwrap();
    let (width, _) = input.size();
    ((index % width) as i32, (index / width) as i32)
}

#[aoc(day12, part1)]
fn find_shortest_path_len(input: &Grid<char>) -> u64 {
    let start = find_cell('S', input);
    let end = find_cell('E', input);
    if let Some(path) = find_path(input, start, end) {
        return (path.len() - 1) as u64
    }
    0
}

#[aoc(day12, part2)]
fn find_shortest_path_any_a(input: &Grid<char>) -> u64 {
    let (width, height) = input.size();
    let end = find_cell('E', input);
    let mut min_dist = u64::MAX;
    for y in 0..height as i32 {
        for x in 0..width as i32 {
            if input.cell_at(x, y).unwrap() != 'a' && input.cell_at(x, y).unwrap() != 'S' {
                continue;
            }
        if let Some(path) = find_path(input, (x as i32, y as i32), end) {
            let steps = (path.len() - 1) as u64;
            if steps < min_dist {
                min_dist = steps
            }
        }
    }
    }
    min_dist as u64
}

#[cfg(test)]
mod tests {
    use super::*;

    const DAY12_EXAMPLE: &str = "Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi";

    #[test]
    fn test_day12_part1() {
        let input = parse_input(DAY12_EXAMPLE);
        assert_eq!(find_shortest_path_len(&input), 31);
    }

    #[test]
    fn test_day12_part2() {
        let input = parse_input(DAY12_EXAMPLE);
        assert_eq!(find_shortest_path_any_a(&input), 29);
    }
}
