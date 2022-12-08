use std::fmt;

// Grid by Belén Albeza
// https://github.com/belen-albeza/aoc-2021/blob/main/src/utils.rs
#[derive(Debug, PartialEq, Clone)]
pub struct Grid<T> {
    pub cells: Vec<T>,
    width: usize,
    height: usize,
}

impl<T: Clone> Grid<T> {
    pub fn new(cells: &[T], width: usize) -> Self {
        let len = cells.len();
        let cells = cells.to_owned();
        Self {
            cells,
            width,
            height: len / width,
        }
    }

    pub fn size(&self) -> (usize, usize) {
        (self.width, self.height)
    }

    pub fn cell_at(&self, x: i32, y: i32) -> Option<T> {
        if x < 0 || y < 0 || x >= self.width as i32 || y >= self.height as i32 {
            return None;
        }

        let i = self.index_for(x as usize, y as usize);
        Some(self.cells[i as usize].clone())
    }

    pub fn set_at(&mut self, x: usize, y: usize, value: T) {
        if x >= self.width || y >= self.height {
            panic!("Trying to set unexisting coordinates: ({}, {})", x, y);
        }

        let i = self.index_for(x, y);
        self.cells[i] = value;
    }

    pub fn neighbors_at(&self, x: i32, y: i32) -> Vec<(T, (usize, usize))> {
        [(x, y - 1), (x + 1, y), (x, y + 1), (x - 1, y)]
            .iter()
            .filter_map(|point| {
                self.cell_at(point.0, point.1)
                    .map(|cell| (cell, (point.0 as usize, point.1 as usize)))
            })
            .collect()
    }

    pub fn neighbors8_at(&self, x: i32, y: i32) -> Vec<(T, (usize, usize))> {
        [
            (x, y - 1),
            (x + 1, y - 1),
            (x + 1, y),
            (x + 1, y + 1),
            (x, y + 1),
            (x - 1, y + 1),
            (x - 1, y),
            (x - 1, y - 1),
        ]
        .iter()
        .filter_map(|point| {
            self.cell_at(point.0, point.1)
                .map(|cell| (cell, (point.0 as usize, point.1 as usize)))
        })
        .collect()
    }

    fn index_for(&self, x: usize, y: usize) -> usize {
        y * self.width + x
    }
}

impl<T: Clone + fmt::Debug> fmt::Display for Grid<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut buffer = "".to_string();
        for y in 0..self.size().1 {
            for x in 0..self.size().0 {
                buffer.push_str(&format!("{:?}", self.cell_at(x as i32, y as i32).unwrap()));
            }
            buffer.push('\n');
        }
        writeln!(f, "{}", buffer)
    }
}