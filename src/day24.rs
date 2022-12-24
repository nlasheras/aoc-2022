use crate::utils::Grid;
use aoc_runner_derive::aoc;
use aoc_runner_derive::aoc_generator;

#[derive(Clone)]
pub struct Blizzard {
    pos: (i32, i32),
    dir: char,
}

impl Blizzard {
    fn new(pos: (i32, i32), dir: char) -> Blizzard {
        assert!(dir == '>' || dir == '<' || dir == 'v' || dir == '^');
        Blizzard { pos: pos, dir: dir }
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
                        ret.push(blizzards.len().to_string().chars().nth(0).unwrap());
                        continue;
                    }
                    ret.push(blizzards.iter().nth(0).unwrap().dir);
                    continue;
                }
                let c = self.map.cell_at(x, y).unwrap();
                ret.push(c);
            }
            ret.push('\n');
        }
        ret
    }

    fn tick(&self, minutes: usize) -> Vec<Blizzard> {
        let mut ret = self.blizzards.clone();
        let (width, height) = self.map.size();
        for b in ret.iter_mut() {
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
    let width = rows.iter().nth(0).unwrap().len() as i32;
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

pub fn find_path(input: &Valley) -> Vec<(i32, i32)> {
    let valley = input.clone();
    println!("{}", valley.print(0, Some((1, 0))));
    println!("{}", valley.print(1, Some((1, 0))));
    println!("{}", valley.print(2, Some((1, 0))));
    Vec::new()
}

#[aoc(day24, part1)]
pub fn shortest_path_minutes(input: &Valley) -> u64 {
    find_path(input).len() as u64
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
}
