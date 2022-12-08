use aoc_runner_derive::aoc;
use aoc_runner_derive::aoc_generator;
use std::cmp;

use crate::utils::Grid;

#[aoc_generator(day8)]
pub fn parse_input(input: &str) -> Grid<u64> {
    let vec: Vec<Vec<u64>> = input
        .lines()
        .map(|s| {
            s.chars()
                .map(|c| c.to_string().parse::<u64>().unwrap())
                .collect()
        })
        .collect();

    let width = vec.first().unwrap().len();
    let cells = vec.into_iter().flatten().collect::<Vec<u64>>();
    Grid::new(&cells, width)
}

fn get_visible_trees(grid: &Grid<u64>, pos: (usize, usize), dir: (i32, i32)) -> Vec<u64> {
    let mut trees = Vec::new();
    let mut x = pos.0 as i32;
    let mut y = pos.1 as i32;
    while let Some(cell) = grid.cell_at(x + dir.0, y + dir.1) {
        trees.push(cell);
        x += dir.0;
        y += dir.1;
    }
    trees
}

fn is_visible_direction(grid: &Grid<u64>, pos: (usize, usize), dir: (i32, i32)) -> bool {
    let height = grid.cell_at(pos.0 as i32, pos.1 as i32).unwrap();
    get_visible_trees(grid, pos, dir).into_iter().filter(|t| *t >= height).count() == 0
}

fn is_visible(grid: &Grid<u64>, pos: (usize, usize)) -> bool {
    is_visible_direction(grid, pos, (0, -1))
        || is_visible_direction(grid, pos, (0, 1))
        || is_visible_direction(grid, pos, (-1, 0))
        || is_visible_direction(grid, pos, (1, 0))
}

#[aoc(day8, part1)]
pub fn count_visible(grid: &Grid<u64>) -> u64 {
    let (width, height) = grid.size();
    let mut sum = 0;
    for y in 0..height {
        for x in 0..width {
            if is_visible(grid, (x, y)) {
                sum += 1
            }
        }
    }
    sum
}

fn get_viewing_distance(grid: &Grid<u64>, pos: (usize, usize), dir: (i32, i32)) -> u64 {
    let height = grid.cell_at(pos.0 as i32, pos.1 as i32).unwrap();
    let trees = get_visible_trees(grid, pos, dir);
    if let Some(position) = trees.iter().position(|t| *t >= height) {
        return (position + 1) as u64
    }
    trees.len() as u64
}

fn get_scenic_score(grid: &Grid<u64>, pos: (usize, usize)) -> u64 {
    get_viewing_distance(grid, pos, (0, -1))
        * get_viewing_distance(grid, pos, (0, 1))
        * get_viewing_distance(grid, pos, (-1, 0))
        * get_viewing_distance(grid, pos, (1, 0))
}

#[aoc(day8, part2)]
pub fn find_highest_scenic(grid: &Grid<u64>) -> u64 {
    let mut max = 0;
    let (width, height) = grid.size();
    for y in 0..height {
        for x in 0..width {
            max = cmp::max(max, get_scenic_score(grid, (x, y)))
        }
    }
    max
}


#[cfg(test)]
mod tests {
    use super::*;

    const DAY08_EXAMPLE: &str = "30373
25512
65332
33549
35390";

    #[test]
    fn test_day8_part1() {
        let input = parse_input(DAY08_EXAMPLE);
        assert_eq!(count_visible(&input), 21);
    }

    #[test]
    fn test_day8_part2() {
        let input = parse_input(DAY08_EXAMPLE);
        assert_eq!(find_highest_scenic(&input), 8);
    }
}
