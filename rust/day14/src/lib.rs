use std::{fmt::Display, str::FromStr};

use aoc_traits::AdventOfCodeDay;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Cell {
    Rock,
    Wall,
    Empty,
}

#[derive(Debug, Clone, PartialEq, Eq)]
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
    fn shift_north(&self) -> Grid {
        let mut res = self.lines.clone();
        let x = self.lines[0].len();
        let y = self.lines.len();

        for i in 1..y {
            for j in 0..x {
                if res[i][j] == Cell::Rock && res[i - 1][j] == Cell::Empty {
                    let mut goal = i - 1;
                    while goal > 0 && res[goal - 1][j] == Cell::Empty {
                        goal -= 1;
                    }
                    res[i][j] = Cell::Empty;
                    res[goal][j] = Cell::Rock;
                }
            }
        }

        Grid { lines: res }
    }

    fn count_load(&self) -> usize {
        let y = self.lines.len();
        self.lines
            .iter()
            .enumerate()
            .map(|(i, x)| x.iter().filter(|x| **x == Cell::Rock).count() * (y - i))
            .sum()
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
    let north = input.shift_north();
    // println!("{}", north);
    north.count_load() as u64
}

fn solve_stage2(input: &Grid) -> u64 {
    0
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
        assert_eq!(super::solve_stage2(&input), 400);
    }
}
