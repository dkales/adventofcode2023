use std::str::FromStr;

use aoc_traits::AdventOfCodeDay;

pub struct Space {
    galaxies: Vec<(usize, usize)>,
    empty_rows: Vec<bool>,
    empty_cols: Vec<bool>,
}

impl FromStr for Space {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let input: Vec<Vec<_>> = s.lines().map(|l| l.chars().collect()).collect();
        let mut galaxies = Vec::new();
        for (x, row) in input.iter().enumerate() {
            for (y, col) in row.iter().enumerate() {
                if *col == '#' {
                    galaxies.push((x, y));
                }
            }
        }
        let (_x_dim, y_dim) = (input.len(), input[0].len());
        let empty_rows = input
            .iter()
            .map(|row| row.iter().all(|c| *c == '.'))
            .collect::<Vec<_>>();
        let empty_cols = input.iter().fold(vec![true; y_dim], |acc, row| {
            row.iter()
                .enumerate()
                .map(|(i, c)| (*c == '.') && acc[i])
                .collect::<Vec<_>>()
        });
        Ok(Space {
            galaxies,
            empty_rows,
            empty_cols,
        })
    }
}

fn solve_stage1(input: &Space) -> u64 {
    input
        .galaxies
        .iter()
        .enumerate()
        .flat_map(|(i, x)| {
            input.galaxies.iter().skip(i + 1).map(|y| {
                let (start_x, end_x) = if x.0 < y.0 { (x.0, y.0) } else { (y.0, x.0) };
                let (start_y, end_y) = if x.1 < y.1 { (x.1, y.1) } else { (y.1, x.1) };
                let x_boost = input.empty_rows[start_x..end_x]
                    .iter()
                    .filter(|x| **x)
                    .count();
                let y_boost = input.empty_cols[start_y..end_y]
                    .iter()
                    .filter(|x| **x)
                    .count();
                end_x - start_x + end_y - start_y + x_boost + y_boost
            })
        })
        .sum::<usize>() as u64
}

fn solve_stage2(input: &Space) -> u64 {
    input
        .galaxies
        .iter()
        .enumerate()
        .flat_map(|(i, x)| {
            input.galaxies.iter().skip(i + 1).map(|y| {
                let (start_x, end_x) = if x.0 < y.0 { (x.0, y.0) } else { (y.0, x.0) };
                let (start_y, end_y) = if x.1 < y.1 { (x.1, y.1) } else { (y.1, x.1) };
                let x_boost = input.empty_rows[start_x..end_x]
                    .iter()
                    .filter(|x| **x)
                    .count()
                    * 999_999;
                let y_boost = input.empty_cols[start_y..end_y]
                    .iter()
                    .filter(|x| **x)
                    .count()
                    * 999_999;
                end_x - start_x + end_y - start_y + x_boost + y_boost
            })
        })
        .sum::<usize>() as u64
}

pub struct Day11Solver;
impl AdventOfCodeDay<'_> for Day11Solver {
    type ParsedInput = Space;

    type Part1Output = u64;

    type Part2Output = u64;

    fn solve_part1(input: &Self::ParsedInput) -> Self::Part1Output {
        solve_stage1(input)
    }

    fn solve_part2(input: &Self::ParsedInput) -> Self::Part2Output {
        solve_stage2(input)
    }

    fn parse_input(input: &str) -> Self::ParsedInput {
        Space::from_str(input).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use aoc_traits::AdventOfCodeDay;

    use crate::Day11Solver;

    const TEST_INPUT: &str = "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....";
    #[test]
    fn test_stage1() {
        let input = Day11Solver::parse_input(TEST_INPUT);
        assert_eq!(super::solve_stage1(&input), 374);
    }
    // #[test]
    // only works for the 100 factor
    // fn test_stage2() {
    //     let input = Day11Solver::parse_input(TEST_INPUT);
    //     assert_eq!(super::solve_stage2(&input), 8410);
    // }
}
