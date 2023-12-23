use std::{str::FromStr, vec};

use aoc_traits::AdventOfCodeDay;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Cell {
    Empty,
    Wall,
    Left,
    Right,
    Down,
    Up,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Grid {
    lines: Vec<Vec<Cell>>,
    dims: (usize, usize),
}

impl Grid {
    fn walk(&self) -> u64 {
        let mut visited = vec![vec![false; self.dims.1]; self.dims.0];

        let start = (0, 1);

        self.walk_inner(start, &mut visited, 0)
    }
    fn walk_inner(&self, pos: (usize, usize), vist: &mut Vec<Vec<bool>>, len: u64) -> u64 {
        if pos == (self.dims.0 - 1, self.dims.1 - 2) {
            return len;
        }
        vist[pos.0][pos.1] = true;

        match self.get_neighbor(pos, vist) {
            [Some(x), None, None, None]
            | [None, Some(x), None, None]
            | [None, None, Some(x), None]
            | [None, None, None, Some(x)] => {
                return self.walk_inner(x, vist, len + 1);
            }
            rest => {
                let mut max = 0;
                for x in rest {
                    if let Some(pos) = x {
                        let mut v_clone = vist.clone();
                        max = max.max(self.walk_inner(pos, &mut v_clone, len + 1));
                    }
                }
                return max;
            }
        }
    }
    fn get_neighbor(&self, pos: (usize, usize), dist: &[Vec<bool>]) -> [Option<(usize, usize)>; 4] {
        let mut res = [None, None, None, None];

        if pos.0 > 0 {
            if !dist[pos.0 - 1][pos.1]
                && !matches!(self.lines[pos.0 - 1][pos.1], Cell::Wall | Cell::Down)
            {
                res[0] = Some((pos.0 - 1, pos.1))
            }
        }
        if pos.1 > 0 {
            if !dist[pos.0][pos.1 - 1]
                && !matches!(self.lines[pos.0][pos.1 - 1], Cell::Wall | Cell::Right)
            {
                res[1] = Some((pos.0, pos.1 - 1))
            }
        }
        if pos.0 < self.dims.0 - 1 {
            if !dist[pos.0 + 1][pos.1]
                && !matches!(self.lines[pos.0 + 1][pos.1], Cell::Wall | Cell::Up)
            {
                res[2] = Some((pos.0 + 1, pos.1))
            }
        }
        if pos.1 < self.dims.1 {
            if !dist[pos.0][pos.1 + 1]
                && !matches!(self.lines[pos.0][pos.1 + 1], Cell::Wall | Cell::Left)
            {
                res[3] = Some((pos.0, pos.1 + 1))
            }
        }

        res
    }
    fn walk2(&self) -> u64 {
        let mut visited = vec![vec![false; self.dims.1]; self.dims.0];

        let start = (0, 1);

        self.walk_inner2(start, &mut visited, 0)
    }
    fn walk_inner2(&self, pos: (usize, usize), vist: &mut Vec<Vec<bool>>, len: u64) -> u64 {
        if pos == (self.dims.0 - 1, self.dims.1 - 2) {
            return len;
        }
        vist[pos.0][pos.1] = true;

        match self.get_neighbor2(pos, vist) {
            [Some(x), None, None, None]
            | [None, Some(x), None, None]
            | [None, None, Some(x), None]
            | [None, None, None, Some(x)] => {
                return self.walk_inner2(x, vist, len + 1);
            }
            rest => {
                let mut max = 0;
                for x in rest {
                    if let Some(pos) = x {
                        let mut v_clone = vist.clone();
                        max = max.max(self.walk_inner2(pos, &mut v_clone, len + 1));
                    }
                }
                return max;
            }
        }
    }
    fn get_neighbor2(
        &self,
        pos: (usize, usize),
        dist: &[Vec<bool>],
    ) -> [Option<(usize, usize)>; 4] {
        let mut res = [None, None, None, None];

        if pos.0 > 0 {
            if !dist[pos.0 - 1][pos.1] && !matches!(self.lines[pos.0 - 1][pos.1], Cell::Wall) {
                res[0] = Some((pos.0 - 1, pos.1))
            }
        }
        if pos.1 > 0 {
            if !dist[pos.0][pos.1 - 1] && !matches!(self.lines[pos.0][pos.1 - 1], Cell::Wall) {
                res[1] = Some((pos.0, pos.1 - 1))
            }
        }
        if pos.0 < self.dims.0 - 1 {
            if !dist[pos.0 + 1][pos.1] && !matches!(self.lines[pos.0 + 1][pos.1], Cell::Wall) {
                res[2] = Some((pos.0 + 1, pos.1))
            }
        }
        if pos.1 < self.dims.1 {
            if !dist[pos.0][pos.1 + 1] && !matches!(self.lines[pos.0][pos.1 + 1], Cell::Wall) {
                res[3] = Some((pos.0, pos.1 + 1))
            }
        }

        res
    }
}

impl FromStr for Grid {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lines: Vec<Vec<_>> = s
            .lines()
            .map(|x| {
                x.chars()
                    .map(|x| match x {
                        '.' => Cell::Empty,
                        '#' => Cell::Wall,
                        '>' => Cell::Right,
                        '<' => Cell::Left,
                        '^' => Cell::Up,
                        'v' => Cell::Down,
                        _ => unreachable!(),
                    })
                    .collect()
            })
            .collect();
        Ok(Grid {
            dims: (lines.len(), lines[0].len()),
            lines,
        })
    }
}

fn solve_stage1(input: &Grid) -> u64 {
    input.walk()
}

fn solve_stage2(input: &Grid) -> u64 {
    input.walk2()
}

pub struct Day23Solver;
impl AdventOfCodeDay<'_> for Day23Solver {
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

    use crate::Day23Solver;

    const TEST_INPUT: &str = r#"#.#####################
#.......#########...###
#######.#########.#.###
###.....#.>.>.###.#.###
###v#####.#v#.###.#.###
###.>...#.#.#.....#...#
###v###.#.#.#########.#
###...#.#.#.......#...#
#####.#.#.#######.#.###
#.....#.#.#.......#...#
#.#####.#.#.#########v#
#.#...#...#...###...>.#
#.#.#v#######v###.###v#
#...#.>.#...>.>.#.###.#
#####v#.#.###v#.#.###.#
#.....#...#...#.#.#...#
#.#########.###.#.#.###
#...###...#...#...#.###
###.###.#.###v#####v###
#...#...#.#.>.>.#.>.###
#.###.###.#.###.#.#v###
#.....###...###...#...#
#####################.#"#;
    #[test]
    fn test_stage1() {
        let input = Day23Solver::parse_input(TEST_INPUT);
        assert_eq!(super::solve_stage1(&input), 94);
    }
    #[test]
    fn test_stage2() {
        let input = Day23Solver::parse_input(TEST_INPUT);
        assert_eq!(super::solve_stage2(&input), 154);
    }
}
