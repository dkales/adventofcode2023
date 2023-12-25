#![allow(clippy::needless_range_loop)]
use core::panic;
use std::collections::HashSet;

use aoc_traits::AdventOfCodeDay;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Brick {
    id: usize,
    start: (usize, usize, usize),
    end: (usize, usize, usize),
    resting_on: HashSet<usize>,
}

impl Brick {
    fn drop_if_possible(&mut self, grid: &mut [Vec<(usize, usize)>]) -> bool {
        let mut drop_distance = usize::MAX;
        for i in self.start.0..=self.end.0 {
            for j in self.start.1..=self.end.1 {
                if grid[i][j].0 >= self.start.2 {
                    panic!("Overlapping Bricks");
                }
                drop_distance = drop_distance.min(self.start.2 - (grid[i][j].0 + 1))
            }
        }
        self.start.2 -= drop_distance;
        self.end.2 -= drop_distance;

        for i in self.start.0..=self.end.0 {
            for j in self.start.1..=self.end.1 {
                if grid[i][j].0 + 1 == self.start.2 {
                    self.resting_on.insert(grid[i][j].1);
                }
                grid[i][j] = (self.end.2, self.id);
            }
        }
        drop_distance > 0
    }
}

pub struct Input {
    bricks: Vec<Brick>,
    dims: (usize, usize, usize),
}

fn solve_stage1(input: &Input) -> u64 {
    let mut grid = vec![vec![(0, 0); input.dims.1 + 1]; input.dims.0 + 1];
    let mut sorted = input.bricks.clone();
    sorted.sort_by_key(|x| x.end.2);

    for brick in sorted.iter_mut() {
        brick.drop_if_possible(&mut grid);
    }

    let single_supports = sorted.iter().fold(HashSet::<usize>::new(), |mut acc, x| {
        if x.resting_on.len() == 1 && !x.resting_on.contains(&0) {
            acc.extend(x.resting_on.iter());
        }
        acc
    });
    (sorted.len() - single_supports.len()) as u64 // we also insert the ground
}

fn solve_stage2(input: &Input) -> u64 {
    let mut grid = vec![vec![(0, 0); input.dims.1 + 1]; input.dims.0 + 1];
    let mut sorted = input.bricks.clone();
    sorted.sort_by_key(|x| x.end.2);

    for brick in sorted.iter_mut() {
        brick.drop_if_possible(&mut grid);
    }

    (1..=sorted.len())
        .map(|x| {
            let mut dropping = HashSet::new();
            dropping.insert(x);
            for brick in sorted.iter() {
                if brick.resting_on.difference(&dropping).count() == 0 {
                    dropping.insert(brick.id);
                }
            }
            dropping.len() - 1 // we inserted ourselves
        })
        .sum::<usize>() as u64
}

pub struct Day22Solver;
impl AdventOfCodeDay<'_> for Day22Solver {
    type ParsedInput = Input;

    type Part1Output = u64;

    type Part2Output = u64;

    fn solve_part1(input: &Self::ParsedInput) -> Self::Part1Output {
        solve_stage1(input)
    }

    fn solve_part2(input: &Self::ParsedInput) -> Self::Part2Output {
        solve_stage2(input)
    }

    fn parse_input(input: &str) -> Self::ParsedInput {
        let mut dims = (0, 0, 0);
        let mut id = 0;
        let bricks = input
            .lines()
            .map(|x| {
                let (first, second) = x.split_once('~').unwrap();
                let mut start = first.split(',').map(|x| x.parse().unwrap());
                let mut end = second.split(',').map(|x| x.parse().unwrap());
                let start = (
                    start.next().unwrap(),
                    start.next().unwrap(),
                    start.next().unwrap(),
                );
                let end = (
                    end.next().unwrap(),
                    end.next().unwrap(),
                    end.next().unwrap(),
                );
                dims.0 = dims.0.max(start.0).max(end.0);
                dims.1 = dims.1.max(start.1).max(end.1);
                dims.2 = dims.2.max(start.2).max(end.2);
                assert!(start.0 <= end.0);
                assert!(start.1 <= end.1);
                assert!(start.2 <= end.2);
                id += 1;
                Brick {
                    start,
                    end,
                    id,
                    resting_on: Default::default(),
                }
            })
            .collect();
        Input { bricks, dims }
    }
}

#[cfg(test)]
mod tests {
    use aoc_traits::AdventOfCodeDay;

    use crate::Day22Solver;

    const TEST_INPUT: &str = r#"1,0,1~1,2,1
0,0,2~2,0,2
0,2,3~2,2,3
0,0,4~0,2,4
2,0,5~2,2,5
0,1,6~2,1,6
1,1,8~1,1,9"#;
    #[test]
    fn test_stage1() {
        let input = Day22Solver::parse_input(TEST_INPUT);
        assert_eq!(super::solve_stage1(&input), 5);
    }
    #[test]
    fn test_stage2() {
        let input = Day22Solver::parse_input(TEST_INPUT);
        assert_eq!(super::solve_stage2(&input), 7);
    }
}
