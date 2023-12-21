use std::{
    collections::{HashMap, VecDeque},
    str::FromStr,
};

use aoc_traits::AdventOfCodeDay;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Cell {
    Empty,
    Wall,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Grid {
    lines: Vec<Vec<Cell>>,
    dims: (usize, usize),
    start: (usize, usize),
}

impl Grid {
    fn get_dist(&self, limit: usize) -> HashMap<(isize, isize), usize> {
        let mut dists = HashMap::new();
        let mut queue = VecDeque::new();
        queue.push_front((self.start.0 as isize, self.start.1 as isize));
        dists.insert((self.start.0 as isize, self.start.1 as isize), 0);
        while let Some(point) = queue.pop_front() {
            let dist = dists[&point];
            if dist >= limit {
                continue;
            }
            for (i, j) in [
                (point.0 as isize + 1, point.1 as isize),
                (point.0 as isize - 1, point.1 as isize),
                (point.0 as isize, point.1 as isize + 1),
                (point.0 as isize, point.1 as isize - 1),
            ] {
                let (i_idx, j_idx) = (
                    i.rem_euclid(self.dims.0 as isize) as usize,
                    j.rem_euclid(self.dims.1 as isize) as usize,
                );
                if let Some(cell) = self.lines.get(i_idx).and_then(|x| x.get(j_idx)) {
                    if *cell == Cell::Wall {
                        continue;
                    }
                    if let Some(old_dist) = dists.get(&(i, j)) {
                        if *old_dist <= dist + 1 {
                            continue;
                        }
                    }
                    dists.insert((i, j), dist + 1);
                    queue.push_back((i, j));
                }
            }
        }
        dists
    }
}

impl FromStr for Grid {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut start = None;
        let lines: Vec<Vec<_>> = s
            .lines()
            .enumerate()
            .map(|(i, x)| {
                x.chars()
                    .enumerate()
                    .map(|(j, x)| match x {
                        '.' => Cell::Empty,
                        '#' => Cell::Wall,
                        'S' => {
                            start = Some((i, j));
                            Cell::Empty
                        }
                        _ => unreachable!(),
                    })
                    .collect()
            })
            .collect();
        Ok(Grid {
            dims: (lines.len(), lines[0].len()),
            lines,
            start: start.unwrap(),
        })
    }
}

fn solve_stage1(input: &Grid, steps: usize) -> u64 {
    let dists = input.get_dist(steps);
    dists.values().filter(|&&x| x % 2 == 0).count() as u64
}

fn solve_stage2(input: &Grid, steps: usize) -> u64 {
    let dists = input.get_dist(steps);
    dists.values().filter(|&&x| x % 2 == 0).count() as u64
}

pub struct Day21Solver;
impl AdventOfCodeDay<'_> for Day21Solver {
    type ParsedInput = Grid;

    type Part1Output = u64;

    type Part2Output = u64;

    fn solve_part1(input: &Self::ParsedInput) -> Self::Part1Output {
        solve_stage1(input, 64)
    }

    fn solve_part2(input: &Self::ParsedInput) -> Self::Part2Output {
        todo!()
        // solve_stage2(input, 26501365)
    }

    fn parse_input(input: &str) -> Self::ParsedInput {
        input.parse().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use aoc_traits::AdventOfCodeDay;

    use crate::Day21Solver;

    const TEST_INPUT: &str = r#"...........
.....###.#.
.###.##..#.
..#.#...#..
....#.#....
.##..S####.
.##..#...#.
.......##..
.##.#.####.
.##..##.##.
..........."#;
    #[test]
    fn test_stage1() {
        let input = Day21Solver::parse_input(TEST_INPUT);
        assert_eq!(super::solve_stage1(&input, 6), 16);
    }
    #[test]
    fn test_stage2() {
        let input = Day21Solver::parse_input(TEST_INPUT);
        assert_eq!(super::solve_stage2(&input, 6), 16);
        assert_eq!(super::solve_stage2(&input, 10), 50);
        assert_eq!(super::solve_stage2(&input, 50), 1594);
        assert_eq!(super::solve_stage2(&input, 100), 6536);
        assert_eq!(super::solve_stage2(&input, 500), 167004);
        assert_eq!(super::solve_stage2(&input, 1000), 668697);
        assert_eq!(super::solve_stage2(&input, 5000), 16733044);
    }
}
