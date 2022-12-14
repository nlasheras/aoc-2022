use crate::utils::Grid;
use aoc_runner_derive::aoc;
use aoc_runner_derive::aoc_generator;
use priority_queue::DoublePriorityQueue;
use std::collections::BTreeMap;
use std::collections::BTreeSet;

#[derive(Clone)]
pub struct Blizzard {
    pos: (i32, i32),
    dir: char,
}

impl Blizzard {
    fn new(pos: (i32, i32), dir: char) -> Blizzard {
        assert!(dir == '>' || dir == '<' || dir == 'v' || dir == '^');
        Blizzard { pos, dir }
    }

    fn as_vec(&self) -> (i32, i32) {
        match self.dir {
            '>' => (1, 0),
            '<' => (-1, 0),
            'v' => (0, 1),
            '^' => (0, -1),
            _ => panic!("Wrong direction {}", self.dir),
        }
    }
}

#[derive(Clone)]
pub struct Valley {
    map: Grid<char>,
    start: (i32, i32),
    end: (i32, i32),
    blizzards: Vec<Blizzard>,
}

impl Valley {
    #[allow(dead_code)]
    fn print(&self, minutes: usize, pos: Option<(i32, i32)>) -> String {
        let mut ret = "".to_string();
        let (width, height) = self.map.size();
        let state = self.tick(minutes);
        for y in 0..height as i32 {
            for x in 0..width as i32 {
                if let Some(current) = pos {
                    if current == (x, y) {
                        ret.push('E');
                        continue;
                    }
                }
                let blizzards = state
                    .iter()
                    .filter(|b| b.pos == (x, y))
                    .collect::<Vec<&Blizzard>>();
                if !blizzards.is_empty() {
                    if blizzards.len() > 1 {
                        ret.push(blizzards.len().to_string().chars().next().unwrap());
                        continue;
                    }
                    ret.push(blizzards.get(0).unwrap().dir);
                    continue;
                }
                let c = self.map.cell_at(x, y).unwrap();
                ret.push(c);
            }
            ret.push('\n');
        }
        ret
    }

    fn simulate(&self, blizzards: &mut [Blizzard], minutes: usize) {
        let (width, height) = self.map.size();
        for b in blizzards.iter_mut() {
            for _ in 0..minutes {
                let dir = b.as_vec();
                let mut new_pos = b.pos;
                loop {
                    new_pos = (
                        (new_pos.0 + dir.0).rem_euclid(width as i32),
                        (new_pos.1 + dir.1).rem_euclid(height as i32),
                    );
                    let c = self.map.cell_at(new_pos.0, new_pos.1);
                    if let Some(c) = c {
                        if c == '.' {
                            break;
                        }
                    }
                }
                b.pos = new_pos;
            }
        }
    }

    fn tick(&self, minutes: usize) -> Vec<Blizzard> {
        let mut ret = self.blizzards.clone();
        self.simulate(&mut ret, minutes);
        ret
    }
}

#[aoc_generator(day24)]
pub fn parse_input(input: &str) -> Valley {
    let rows = input
        .lines()
        .map(|s| s.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();
    let height = rows.len() as i32;
    let width = rows.get(0).unwrap().len() as i32;
    let mut cells = rows.into_iter().flatten().collect::<Vec<char>>();
    let mut blizzards = Vec::new();
    for y in 0..height {
        for x in 0..width {
            let idx = (y * width + x) as usize;
            let c = cells[idx];
            match c {
                '<' | '>' | 'v' | '^' => {
                    cells[idx] = '.';
                    blizzards.push(Blizzard::new((x, y), c));
                }
                _ => (),
            }
        }
    }

    let grid = Grid::new(&cells, width as usize);
    Valley {
        map: grid,
        start: (1, 0),
        end: (width - 2, height - 1),
        blizzards,
    }
}

fn dist(pos: (i32, i32), goal: (i32, i32)) -> u32 {
    (goal.0 - pos.0).unsigned_abs() + (goal.1 - pos.1).unsigned_abs()
}

pub fn find_path(input: &Valley) -> Option<i32> {
    let mut open_set = DoublePriorityQueue::new();
    open_set.push((input.start, 0), dist(input.start, input.end) as usize);

    let mut state_cache: BTreeMap<usize, Vec<Blizzard>> = BTreeMap::new();
    let mut closed_set = BTreeSet::new();

    while !open_set.is_empty() {
        let (elem, priority) = open_set.pop_min().unwrap();
        let (current, time) = elem;
        if closed_set.contains(&elem) {
            continue;
        }
        closed_set.insert(elem);

        if current == input.end {
            return Some(time as i32);
        }

        let state = match state_cache.get(&(time + 1)) {
            Some(l) => l,
            None => {
                let prev = state_cache.get(&time);
                let blizzards = if let Some(state) = prev {
                    let mut tmp = state.clone();
                    input.simulate(&mut tmp, 1);
                    tmp
                } else {
                    input.tick(time + 1)
                };

                state_cache.insert(time + 1, blizzards);
                state_cache.get(&(time + 1)).unwrap()
            }
        };
        if !state.iter().any(|b| b.pos == current) {
            // wait state
            open_set.push((current, time + 1), priority + 1);
        }
        for (c, cell_pos) in input.map.neighbors_at(current.0, current.1) {
            let candidate = (cell_pos.0 as i32, cell_pos.1 as i32);
            if closed_set.contains(&(candidate, time + 1)) {
                continue;
            }
            if c != '.' {
                continue;
            }

            if state.iter().any(|b| b.pos == candidate) {
                continue;
            }

            let dist = dist(candidate, input.end) as usize;
            let score = dist + time;
            open_set.push((candidate, time + 1), score);
        }
    }
    None
}

#[aoc(day24, part1)]
pub fn shortest_path_minutes(input: &Valley) -> u64 {
    find_path(input).unwrap() as u64
}

#[aoc(day24, part2)]
pub fn shortest_path_part2(input: &Valley) -> u64 {
    let mut valley = input.clone();
    let step1 = find_path(&valley).unwrap();
    valley.blizzards = valley.tick(step1 as usize);
    valley.start = input.end;
    valley.end = input.start;
    let step2 = find_path(&valley).unwrap();
    valley.blizzards = valley.tick(step2 as usize);
    valley.start = input.start;
    valley.end = input.end;
    let step3 = find_path(&valley).unwrap();
    (step1 + step2 + step3) as u64
}

#[cfg(test)]
mod tests {
    use super::*;

    const DAY24_EXAMPLE: &str = "#.######
#>>.<^<#
#.<..<<#
#>v.><>#
#<^v^^>#
######.#";

    #[test]
    fn test_day24_find_path() {
        let input = parse_input(DAY24_EXAMPLE);
        assert_eq!(shortest_path_minutes(&input), 18);
    }

    #[test]
    fn test_day24_find_3_paths() {
        let input = parse_input(DAY24_EXAMPLE);
        assert_eq!(shortest_path_part2(&input), 54);
    }
}
