use std::fmt::{Display, Formatter, Result};

use crate::common;
use crate::vectors::Vec2;

#[derive(Clone, Debug, PartialEq)]
enum CellState {
    Floor,
    EmptySeat,
    OccupiedSeat,
}

impl Display for CellState {
    fn fmt(&self, f: &mut Formatter) -> Result {
        return write!(f, "{}", match self {
            CellState::Floor => ".",
            CellState::EmptySeat => "L",
            CellState::OccupiedSeat => "#"
        });
    }
}

#[derive(Clone, Debug)]
struct Grid {
    height: usize,
    width: usize,
    cells: Vec<CellState>,
}

impl Display for Grid {
    fn fmt(&self, f: &mut Formatter) -> Result {
        for (i, cell) in self.cells.iter().enumerate() {
            let _ = write!(f, "{}", cell);
            if (i + 1) % self.width == 0 {
                let _ = write!(f, "\n");
            }
        }
        return write!(f, "");
    }
}

impl Grid {
    fn parse(lines: Vec<String>) -> Grid {
        return Grid {
            height: lines.len(),
            width: lines[0].len(),
            cells: lines.iter()
                .flat_map(|line| line.chars().into_iter())
                .map(|c| match c {
                    'L' => CellState::EmptySeat,
                    '#' => CellState::OccupiedSeat,
                    _ => CellState::Floor
                })
                .collect(),
        };
    }

    pub fn count_occupied(&self, index: usize, max_dist: i32) -> i32 {
        let x = (index % self.width) as i32;
        let y = (index / self.width) as i32;
        let pos = Vec2::new(x, y);

        return
            self.get_occupied(pos.clone(), Vec2::new(-1, -1), max_dist)
                + self.get_occupied(pos.clone(), Vec2::new(0, -1), max_dist)
                + self.get_occupied(pos.clone(), Vec2::new(1, -1), max_dist)
                + self.get_occupied(pos.clone(), Vec2::new(-1, 0), max_dist)
                + self.get_occupied(pos.clone(), Vec2::new(1, 0), max_dist)
                + self.get_occupied(pos.clone(), Vec2::new(-1, 1), max_dist)
                + self.get_occupied(pos.clone(), Vec2::new(0, 1), max_dist)
                + self.get_occupied(pos.clone(), Vec2::new(1, 1), max_dist);
    }

    pub fn get_occupied(&self, pos: Vec2<i32>, dir: Vec2<i32>, max_dist: i32) -> i32 {
        let mut cur = pos.clone();
        for _ in 0..max_dist {
            cur = cur.clone() + dir.clone();
            if cur.x < 0 || cur.x >= self.width as i32 || cur.y < 0 || cur.y >= self.height as i32 {
                return 0;
            }
            match self.cells[cur.y as usize * self.width + cur.x as usize] {
                CellState::OccupiedSeat => return 1,
                CellState::EmptySeat => return 0,
                _ => {}
            }
        }
        return 0;
    }

    pub fn next(&self, min_occupied: i32, max_dist: i32) -> (Grid, bool) {
        let mut new_cells: Vec<CellState> = Vec::new();
        let mut changed = false;
        for (i, cell) in self.cells.iter().enumerate() {
            let occupied_neighbors = self.count_occupied(i, max_dist);
            let new_cell_state = match cell {
                CellState::Floor => CellState::Floor,
                CellState::EmptySeat => if occupied_neighbors == 0 { CellState::OccupiedSeat } else { CellState::EmptySeat },
                CellState::OccupiedSeat => if occupied_neighbors >= min_occupied { CellState::EmptySeat } else { CellState::OccupiedSeat },
            };
            if new_cell_state != *cell {
                changed = true;
            }
            new_cells.push(new_cell_state);
        }
        return (Grid {
            width: self.width,
            height: self.height,
            cells: new_cells,
        }, changed);
    }

    pub fn count_occupied_in_vec(cells: &Vec<CellState>) -> usize {
        return cells.into_iter()
            .filter(|n| **n == CellState::OccupiedSeat)
            .count();
    }
}

pub fn part_one() {
    println!("--- Part One ---");

    let mut grid = Grid::parse(common::read_strings("./data/dec_11.txt"));
    loop {
        let (new_grid, changed) = grid.next(4, 1);
        if !changed {
            break;
        }
        grid = new_grid;
    }
    println!("Result: {:?}", Grid::count_occupied_in_vec(&grid.cells));
}

pub fn part_two() {
    println!("--- Part Two ---");

    let mut grid = Grid::parse(common::read_strings("./data/dec_11.txt"));
    loop {
        let (new_grid, changed) = grid.next(5, 999);
        if !changed {
            break;
        }
        grid = new_grid;
    }
    println!("Result: {:?}", Grid::count_occupied_in_vec(&grid.cells));
}
