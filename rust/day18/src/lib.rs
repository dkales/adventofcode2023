use std::{collections::VecDeque, fmt::Display, str::FromStr};

use aoc_traits::AdventOfCodeDay;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Grid {
    lines: Vec<Vec<u8>>,
    dims: (usize, usize),
}

impl Display for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for line in &self.lines {
            for c in line {
                write!(f, "{}", c)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct BuildInstructions {
    instr: Vec<BuildInstruction>,
    dims: (usize, usize),
    offset: (usize, usize),
}

pub struct Input {
    pub stage1: BuildInstructions,
    pub stage2: BuildInstructions,
}

impl FromStr for Input {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let instr: Vec<_> = s
            .lines()
            .map(|x| {
                let mut parts = x.split(' ');
                let dir = parts.next().unwrap().chars().next().unwrap();
                let mut coords = parts.next().unwrap().split(',');
                let y = coords.next().unwrap().parse().unwrap();
                BuildInstruction { dir, y }
            })
            .collect();

        let (bounds, cur) = instr.iter().fold(
            ((0, 0, 0, 0), (0isize, 0isize)),
            |((top, bot, left, right), cur), instr| {
                let new_cur = match instr.dir {
                    'U' => (cur.0 - instr.y as isize, cur.1),
                    'D' => (cur.0 + instr.y as isize, cur.1),
                    'L' => (cur.0, cur.1 - instr.y as isize),
                    'R' => (cur.0, cur.1 + instr.y as isize),
                    _ => unreachable!(),
                };
                (
                    (
                        top.min(new_cur.0),
                        bot.max(new_cur.0),
                        left.min(new_cur.1),
                        right.max(new_cur.1),
                    ),
                    new_cur,
                )
            },
        );
        // we come back to the start
        assert!(cur == (0, 0));

        let instr2: Vec<_> = s
            .lines()
            .map(|x| {
                let (_, rest) = x.split_once('#').unwrap();
                let rest = rest.trim_end_matches(')');
                let dir = match rest.chars().next_back().unwrap() {
                    '0' => 'R',
                    '1' => 'D',
                    '2' => 'L',
                    '3' => 'U',
                    _ => unreachable!(),
                };
                let y = usize::from_str_radix(&rest[..rest.len() - 1], 16).unwrap();
                BuildInstruction { dir, y }
            })
            .collect();

        let (bounds2, cur2) = instr2.iter().fold(
            ((0, 0, 0, 0), (0isize, 0isize)),
            |((top, bot, left, right), cur), instr| {
                let new_cur = match instr.dir {
                    'U' => (cur.0 - instr.y as isize, cur.1),
                    'D' => (cur.0 + instr.y as isize, cur.1),
                    'L' => (cur.0, cur.1 - instr.y as isize),
                    'R' => (cur.0, cur.1 + instr.y as isize),
                    _ => unreachable!(),
                };
                (
                    (
                        top.min(new_cur.0),
                        bot.max(new_cur.0),
                        left.min(new_cur.1),
                        right.max(new_cur.1),
                    ),
                    new_cur,
                )
            },
        );
        // we come back to the start
        assert!(cur2 == (0, 0));

        Ok(Input {
            stage1: BuildInstructions {
                instr,
                dims: (
                    (bounds.1 - bounds.0) as usize + 1,
                    (bounds.3 - bounds.2) as usize + 1,
                ),
                offset: ((-bounds.0 as usize), (-bounds.2) as usize),
            },
            stage2: BuildInstructions {
                instr: instr2,
                dims: (
                    (bounds2.1 - bounds2.0) as usize + 1,
                    (bounds2.3 - bounds2.2) as usize + 1,
                ),
                offset: ((-bounds2.0 as usize), (-bounds2.2) as usize),
            },
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct BuildInstruction {
    dir: char,
    y: usize,
}

fn solve(input: &BuildInstructions) -> u64 {
    println!("{:?}", input.dims);
    let mut grid = Grid {
        lines: vec![vec![0; input.dims.1]; input.dims.0],
        dims: input.dims,
    };
    let mut cur = input.offset;
    grid.lines[cur.0][cur.1] = 1;
    for inst in &input.instr {
        match inst.dir {
            'U' => {
                for _ in 0..inst.y {
                    cur.0 -= 1;
                    grid.lines[cur.0][cur.1] = 1;
                }
            }
            'D' => {
                for _ in 0..inst.y {
                    cur.0 += 1;
                    grid.lines[cur.0][cur.1] = 1;
                }
            }
            'L' => {
                for _ in 0..inst.y {
                    cur.1 -= 1;
                    grid.lines[cur.0][cur.1] = 1;
                }
            }
            'R' => {
                for _ in 0..inst.y {
                    cur.1 += 1;
                    grid.lines[cur.0][cur.1] = 1;
                }
            }
            _ => unreachable!(),
        }
    }
    let mut queue = VecDeque::with_capacity((input.dims.0 + input.dims.1) * 2);
    for i in 0..input.dims.0 {
        queue.push_back((i, 0));
        queue.push_back((i, input.dims.1 - 1));
    }
    for j in 0..input.dims.1 {
        queue.push_back((0, j));
        queue.push_back((input.dims.0 - 1, j));
    }

    while let Some(point) = queue.pop_front() {
        if grid.lines[point.0][point.1] == 0 {
            //mark outside
            grid.lines[point.0][point.1] = 2;
            // add neighbors to queue
            if point.0 > 0 {
                queue.push_back((point.0 - 1, point.1));
            }
            if point.1 > 0 {
                queue.push_back((point.0, point.1 - 1));
            }
            if point.0 < input.dims.0 - 1 {
                queue.push_back((point.0 + 1, point.1));
            }
            if point.1 < input.dims.1 - 1 {
                queue.push_back((point.0, point.1 + 1));
            }
        }
    }

    grid.lines.iter().flatten().filter(|x| **x != 2).count() as u64
}

fn solve_stage1(input: &Input) -> u64 {
    solve(&input.stage1)
}

fn solve_stage2(input: &Input) -> u64 {
    solve(&input.stage2)
}

pub struct Day18Solver;
impl AdventOfCodeDay<'_> for Day18Solver {
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
        input.parse().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use aoc_traits::AdventOfCodeDay;

    use crate::Day18Solver;

    const TEST_INPUT: &str = r#"R 6 (#70c710)
D 5 (#0dc571)
L 2 (#5713f0)
D 2 (#d2c081)
R 2 (#59c680)
D 2 (#411b91)
L 5 (#8ceee2)
U 2 (#caa173)
L 1 (#1b58a2)
U 2 (#caa171)
R 2 (#7807d2)
U 3 (#a77fa3)
L 2 (#015232)
U 2 (#7a21e3)"#;
    #[test]
    fn test_stage1() {
        let input = Day18Solver::parse_input(TEST_INPUT);
        assert_eq!(super::solve_stage1(&input), 62);
    }
    #[test]
    fn test_stage2() {
        let input = Day18Solver::parse_input(TEST_INPUT);
        assert_eq!(super::solve_stage2(&input), 952408144115);
    }
}
