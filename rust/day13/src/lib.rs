use std::{fmt::Display, str::FromStr};

use aoc_traits::AdventOfCodeDay;

pub struct Grid {
    dim: (usize, usize),
    lines: Vec<u32>,
}

impl Display for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for line in &self.lines {
            for i in 0..self.dim.0 {
                write!(
                    f,
                    "{}",
                    if (line >> (self.dim.0 - i - 1)) & 1 == 1 {
                        '#'
                    } else {
                        '.'
                    }
                )?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl Grid {
    fn find_mirror_num(&self) -> Option<u64> {
        'outer: for i in 1..self.dim.1 {
            if self.lines[i] == self.lines[i - 1] {
                // found 1 mirror point, now check if it's a full mirror
                for (x, y) in (0..i - 1).rev().zip(i + 1..self.dim.1) {
                    if self.lines[x] != self.lines[y] {
                        continue 'outer;
                    }
                }
                return Some(i as u64);
            }
        }
        None
    }
    fn mirror_num(&self) -> u64 {
        self.find_mirror_num()
            .map(|x| x * 100)
            .or_else(|| self.transpose().find_mirror_num())
            .unwrap_or_default()
    }

    fn find_smudge_mirror_num(&self) -> Option<u64> {
        'outer: for i in 1..self.dim.1 {
            if self.lines[i] == self.lines[i - 1]
                || (self.lines[i] ^ self.lines[i - 1]).count_ones() == 1
            {
                // found 1 mirror point, now check if it's a full mirror
                let mut already_smudged = self.lines[i] != self.lines[i - 1];
                for (x, y) in (0..i - 1).rev().zip(i + 1..self.dim.1) {
                    if (self.lines[x] ^ self.lines[y]).count_ones() == 1 && already_smudged == false
                    {
                        already_smudged = true;
                    } else if self.lines[x] != self.lines[y] {
                        continue 'outer;
                    }
                }
                if !already_smudged {
                    continue;
                }
                return Some(i as u64);
            }
        }
        None
    }
    fn smudge_mirror_num(&self) -> u64 {
        self.find_smudge_mirror_num()
            .map(|x| x * 100)
            .or_else(|| self.transpose().find_smudge_mirror_num())
            .unwrap_or_default()
    }
    fn transpose(&self) -> Grid {
        let mut lines = vec![0; self.dim.0];
        for i in 0..self.dim.0 {
            for j in 0..self.dim.1 {
                lines[i] |= ((self.lines[j] >> (self.dim.0 - i - 1)) & 1) << (self.dim.1 - 1 - j);
            }
        }
        Grid {
            dim: (self.dim.1, self.dim.0),
            lines,
        }
    }
}

impl FromStr for Grid {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let x = s.lines().next().unwrap().len();
        let lines: Vec<_> = s
            .lines()
            .map(|x| {
                x.chars()
                    .map(|x| match x {
                        '.' => 0,
                        '#' => 1,
                        _ => unreachable!(),
                    })
                    .fold(0, |acc, x| (acc << 1) | x)
            })
            .collect();
        Ok(Grid {
            dim: (x, lines.len()),
            lines,
        })
    }
}

fn solve_stage1(input: &[Grid]) -> u64 {
    input.iter().map(|x| x.mirror_num()).sum()
}

fn solve_stage2(input: &[Grid]) -> u64 {
    input.iter().map(|x| x.smudge_mirror_num()).sum()
}

pub struct Day13Solver;
impl AdventOfCodeDay<'_> for Day13Solver {
    type ParsedInput = Vec<Grid>;

    type Part1Output = u64;

    type Part2Output = u64;

    fn solve_part1(input: &Self::ParsedInput) -> Self::Part1Output {
        solve_stage1(input)
    }

    fn solve_part2(input: &Self::ParsedInput) -> Self::Part2Output {
        solve_stage2(input)
    }

    fn parse_input(input: &str) -> Self::ParsedInput {
        input.split("\n\n").map(|x| x.parse().unwrap()).collect()
    }
}

#[cfg(test)]
mod tests {
    use aoc_traits::AdventOfCodeDay;

    use crate::Day13Solver;

    const TEST_INPUT: &str = "#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#";
    #[test]
    fn test_stage1() {
        let input = Day13Solver::parse_input(TEST_INPUT);
        assert_eq!(super::solve_stage1(&input), 405);
    }
    #[test]
    fn test_stage2() {
        let input = Day13Solver::parse_input(TEST_INPUT);
        assert_eq!(super::solve_stage2(&input), 400);
    }
}
