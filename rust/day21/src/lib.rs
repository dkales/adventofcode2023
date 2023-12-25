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
                (point.0 + 1, point.1),
                (point.0 - 1, point.1),
                (point.0, point.1 + 1),
                (point.0, point.1 - 1),
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
    let offset = input.start.0;
    let grid_size = input.dims.0;
    assert!((steps - offset) % grid_size == 0);
    let num_total_grids = (steps - offset) / grid_size;
    let reachable: Vec<_> = (0..4)
        .map(|i| {
            let small_step = offset + i * grid_size;
            let dists = input.get_dist(small_step);
            dists
                .values()
                .filter(|&&x| x % 2 == (small_step & 1))
                .count() as u64
        })
        .collect();

    // first derivative
    let diffs = reachable
        .windows(2)
        .map(|x| x[1] - x[0])
        .collect::<Vec<_>>();
    // second derivative
    let diffs2 = diffs.windows(2).map(|x| x[1] - x[0]).collect::<Vec<_>>();

    // is constant
    assert!(diffs2.iter().all(|&x| x == diffs2[0]));

    // to lazy to get a formula for this, do it the pyramid way
    let mut v = vec![0; num_total_grids];
    v[0] = diffs[0];
    for i in 1..num_total_grids {
        v[i] = v[i - 1] + diffs2[0];
    }
    let mut total_reachable = vec![0; num_total_grids + 1];
    total_reachable[0] = reachable[0];
    for i in 1..num_total_grids + 1 {
        total_reachable[i] = total_reachable[i - 1] + v[i - 1];
    }

    *total_reachable.last().unwrap()
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
        solve_stage2(input, 26501365)
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
}
