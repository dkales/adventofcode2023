use std::{collections::HashMap, fmt::Display, str::FromStr};

use aoc_traits::AdventOfCodeDay;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Cell {
    Rock,
    Wall,
    Empty,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Grid {
    lines: Vec<Vec<Cell>>,
}

impl Display for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for line in &self.lines {
            for c in line {
                write!(
                    f,
                    "{}",
                    match c {
                        Cell::Rock => 'O',
                        Cell::Wall => '#',
                        Cell::Empty => '.',
                    }
                )?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl Grid {
    fn shift_north(&mut self) {
        let x = self.lines[0].len();
        let y = self.lines.len();

        for i in 1..y {
            for j in 0..x {
                if self.lines[i][j] == Cell::Rock && self.lines[i - 1][j] == Cell::Empty {
                    let mut goal = i - 1;
                    while goal > 0 && self.lines[goal - 1][j] == Cell::Empty {
                        goal -= 1;
                    }
                    self.lines[i][j] = Cell::Empty;
                    self.lines[goal][j] = Cell::Rock;
                }
            }
        }
    }
    fn shift_south(&mut self) {
        let x = self.lines[0].len();
        let y = self.lines.len();

        for i in (0..y - 1).rev() {
            for j in 0..x {
                if self.lines[i][j] == Cell::Rock && self.lines[i + 1][j] == Cell::Empty {
                    let mut goal = i + 1;
                    while goal < y - 1 && self.lines[goal + 1][j] == Cell::Empty {
                        goal += 1;
                    }
                    self.lines[i][j] = Cell::Empty;
                    self.lines[goal][j] = Cell::Rock;
                }
            }
        }
    }
    fn shift_west(&mut self) {
        let x = self.lines[0].len();
        let y = self.lines.len();

        for i in 0..y {
            for j in 1..x {
                if self.lines[i][j] == Cell::Rock && self.lines[i][j - 1] == Cell::Empty {
                    let mut goal = j - 1;
                    while goal > 0 && self.lines[i][goal - 1] == Cell::Empty {
                        goal -= 1;
                    }
                    self.lines[i][j] = Cell::Empty;
                    self.lines[i][goal] = Cell::Rock;
                }
            }
        }
    }
    fn shift_east(&mut self) {
        let x = self.lines[0].len();
        let y = self.lines.len();

        for i in 0..y {
            for j in (0..x - 1).rev() {
                if self.lines[i][j] == Cell::Rock && self.lines[i][j + 1] == Cell::Empty {
                    let mut goal = j + 1;
                    while goal < x - 1 && self.lines[i][goal + 1] == Cell::Empty {
                        goal += 1;
                    }
                    self.lines[i][j] = Cell::Empty;
                    self.lines[i][goal] = Cell::Rock;
                }
            }
        }
    }

    fn count_load(&self) -> usize {
        let y = self.lines.len();
        self.lines
            .iter()
            .enumerate()
            .map(|(i, x)| x.iter().filter(|x| **x == Cell::Rock).count() * (y - i))
            .sum()
    }

    fn cycle(&mut self) {
        self.shift_north();
        self.shift_west();
        self.shift_south();
        self.shift_east();
    }

    fn cycle_n(&mut self, n: usize) {
        let mut cycles = HashMap::new();
        cycles.insert(self.clone(), 0);
        for i in 0..n {
            self.cycle();
            if let Some(c) = cycles.insert(self.clone(), i + 1) {
                let cycle_len = i + 1 - c;
                let remaining = n - i - 1;
                let remaining = remaining % cycle_len;
                for _ in 0..remaining {
                    self.cycle();
                }
                break;
            }
        }
    }
}

impl FromStr for Grid {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lines: Vec<_> = s
            .lines()
            .map(|x| {
                x.chars()
                    .map(|x| match x {
                        '.' => Cell::Empty,
                        '#' => Cell::Wall,
                        'O' => Cell::Rock,
                        _ => unreachable!(),
                    })
                    .collect()
            })
            .collect();
        Ok(Grid { lines })
    }
}

fn solve_stage1(input: &Grid) -> u64 {
    let mut input = input.clone();
    input.shift_north();
    // println!("{}", north);
    input.count_load() as u64
}

fn solve_stage2(input: &Grid) -> u64 {
    let mut input = input.clone();
    input.cycle_n(1_000_000_000);
    input.count_load() as u64
}

pub struct Day14Solver;
impl AdventOfCodeDay<'_> for Day14Solver {
    type ParsedInput = Grid;

    type Part1Output = u64;

    type Part2Output = u64;

    fn solve_part1(input: &Self::ParsedInput) -> Self::Part1Output {
        solve_stage1(input)
    }

    fn solve_part2(input: &Self::ParsedInput) -> Self::Part2Output {
        solve_stage2(input)
    }

    fn parse_input(input: &str) -> Self::ParsedInput {
        input.parse().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use aoc_traits::AdventOfCodeDay;

    use crate::Day14Solver;

    const TEST_INPUT: &str = "O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....";
    #[test]
    fn test_stage1() {
        let input = Day14Solver::parse_input(TEST_INPUT);
        assert_eq!(super::solve_stage1(&input), 136);
    }
    #[test]
    fn test_stage2() {
        let input = Day14Solver::parse_input(TEST_INPUT);
        assert_eq!(super::solve_stage2(&input), 64);
    }
}
