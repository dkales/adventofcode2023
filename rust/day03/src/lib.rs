use std::{rc::Rc, str::FromStr};

use aoc_traits::AdventOfCodeDay;
use color_eyre::eyre::{Report, Result};

#[derive(Debug)]
enum Cell {
    Number(Rc<u32>),
    Symbol(char),
    Empty,
}

#[derive(Debug)]
pub struct Grid {
    cells: Vec<Vec<Cell>>,
}
impl Grid {
    fn get_cell(&self, row: usize, col: usize) -> Option<&Cell> {
        self.cells.get(row).and_then(|r| r.get(col))
    }
    fn has_symbol_neighbor(&self, row: usize, col: usize) -> bool {
        let mut has_symbol_neighbor = false;
        for row_idx in (row.saturating_sub(1))..=(row + 1) {
            for col_idx in (col.saturating_sub(1))..=(col + 1) {
                if row_idx == row && col_idx == col {
                    continue;
                }
                if let Some(Cell::Symbol(_)) = self.get_cell(row_idx, col_idx) {
                    has_symbol_neighbor = true;
                }
            }
        }
        has_symbol_neighbor
    }
    fn find_part_nums(&self) -> Vec<u32> {
        let mut part_nums: Vec<u32> = Vec::new();

        for (row_idx, row) in self.cells.iter().enumerate() {
            let mut already_done_num = false;
            for (col_idx, col) in row.iter().enumerate() {
                match col {
                    Cell::Number(n) => {
                        if !already_done_num && self.has_symbol_neighbor(row_idx, col_idx) {
                            part_nums.push(**n);
                            already_done_num = true;
                        }
                    }
                    _ => already_done_num = false,
                }
            }
        }

        part_nums
    }
    fn get_number_neighbors(&self, row: usize, col: usize) -> Vec<Rc<u32>> {
        let mut neighbors: Vec<Rc<u32>> = Vec::new();
        for row_idx in (row.saturating_sub(1))..=(row + 1) {
            for col_idx in (col.saturating_sub(1))..=(col + 1) {
                if row_idx == row && col_idx == col {
                    continue;
                }
                if let Some(Cell::Number(n)) = self.get_cell(row_idx, col_idx) {
                    neighbors.push(n.clone());
                }
            }
        }
        neighbors.dedup_by(|a, b| Rc::ptr_eq(a, b));
        neighbors
    }
    fn find_gears(&self) -> Vec<u32> {
        let mut gears: Vec<u32> = Vec::new();

        for (row_idx, row) in self.cells.iter().enumerate() {
            for (col_idx, col) in row.iter().enumerate() {
                if let Cell::Symbol('*') = col {
                    let neighbors = self.get_number_neighbors(row_idx, col_idx);
                    if neighbors.len() == 2 {
                        gears.push(*neighbors[0] * *neighbors[1]);
                    }
                }
            }
        }

        gears
    }
}

impl FromStr for Grid {
    type Err = Report;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut cells: Vec<Vec<Cell>> = s
            .lines()
            .map(|l| {
                l.chars()
                    .map(|c| match c {
                        '.' => Cell::Empty,
                        '0'..='9' => Cell::Number(Rc::new(c.to_digit(10).unwrap())),
                        _ => Cell::Symbol(c),
                    })
                    .collect()
            })
            .collect();

        // then, fuse numbers
        for row in cells.iter_mut() {
            let mut current_nums: Vec<&mut Rc<u32>> = Vec::new();
            for cell in row.iter_mut() {
                match cell {
                    Cell::Number(n) => current_nums.push(n),
                    _ => {
                        if current_nums.len() > 1 {
                            let sum =
                                Rc::new(current_nums.iter().fold(0u32, |acc, x| 10 * acc + ***x));

                            for n in current_nums.iter_mut() {
                                **n = sum.clone();
                            }
                        }
                        current_nums.clear();
                    }
                }
            }
            if current_nums.len() > 1 {
                let sum = Rc::new(current_nums.iter().fold(0u32, |acc, x| 10 * acc + ***x));

                for n in current_nums.iter_mut() {
                    **n = sum.clone();
                }
            }
        }
        Ok(Grid { cells })
    }
}

#[cfg(test)]
fn solve_stage1(input: &str) -> Result<u32> {
    let grid = Grid::from_str(input)?;
    Ok(grid.find_part_nums().iter().sum())
}

#[cfg(test)]
fn solve_stage2(input: &str) -> Result<u32> {
    let grid = Grid::from_str(input)?;
    Ok(grid.find_gears().iter().sum())
}

pub struct Day3Solver;

impl AdventOfCodeDay<'_> for Day3Solver {
    type ParsedInput = Grid;

    type Part1Output = u32;

    type Part2Output = u32;

    fn parse_input(input: &str) -> Self::ParsedInput {
        Grid::from_str(input).unwrap()
    }

    fn solve_part1(input: &Self::ParsedInput) -> Self::Part1Output {
        input.find_part_nums().iter().sum()
    }

    fn solve_part2(input: &Self::ParsedInput) -> Self::Part2Output {
        input.find_gears().iter().sum()
    }
}

#[cfg(test)]
mod tests {

    const TEST_INPUT: &str = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";
    #[test]
    fn test_stage1() {
        assert_eq!(super::solve_stage1(TEST_INPUT).unwrap(), 4361);
    }
    #[test]
    fn test_stage2() {
        assert_eq!(super::solve_stage2(TEST_INPUT).unwrap(), 467835);
    }
}
