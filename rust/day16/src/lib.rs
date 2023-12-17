use std::{collections::HashSet, fmt::Display, str::FromStr};

use aoc_traits::AdventOfCodeDay;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Cell {
    Empty,
    Mirror1,
    Mirror2,
    SplitterH,
    SplitterV,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Grid {
    lines: Vec<Vec<Cell>>,
    dims: (usize, usize),
}

impl Display for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for line in &self.lines {
            for c in line {
                write!(
                    f,
                    "{}",
                    match c {
                        Cell::Mirror1 => '/',
                        Cell::Mirror2 => '\\',
                        Cell::SplitterH => '-',
                        Cell::SplitterV => '|',
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
    fn energize(&self, starting_beam: Beam) -> u64 {
        // beams with current position, and direction
        let mut beams = Vec::<Beam>::new();
        // currently energized cells, with energizer directions
        let mut energized = HashSet::<Beam>::new();
        beams.push(starting_beam);
        while !beams.is_empty() {
            let beam = beams.pop().unwrap();
            if energized.contains(&beam) {
                // already energized, ignore
                continue;
            }
            // energize
            let (beam1, beam2) = beam.step(self);
            energized.insert(beam);
            if let Some(beam) = beam1 {
                beams.push(beam);
            }
            if let Some(beam) = beam2 {
                beams.push(beam);
            }
        }
        // count distinct energized cells
        energized
            .into_iter()
            .map(|x| x.pos)
            .collect::<HashSet<_>>()
            .len() as u64
            - 1 // -1 because we also inserted the starting point 0,-1 which is invalid
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Beam {
    pos: (isize, isize),
    dir: (isize, isize),
}

impl Beam {
    fn step(&self, grid: &Grid) -> (Option<Beam>, Option<Beam>) {
        let new_pos = (self.pos.0 + self.dir.0, self.pos.1 + self.dir.1);
        // beam left the grid
        if new_pos.0 < 0
            || new_pos.1 < 0
            || new_pos.0 >= grid.dims.0 as isize
            || new_pos.1 >= grid.dims.1 as isize
        {
            return (None, None);
        }
        let new_pos = (new_pos.0, new_pos.1);

        match grid.lines[new_pos.0 as usize][new_pos.1 as usize] {
            Cell::Empty => {
                // continue on
                (
                    Some(Beam {
                        pos: new_pos,
                        dir: self.dir,
                    }),
                    None,
                )
            }
            Cell::Mirror1 => {
                // reflect at \
                (
                    Some(Beam {
                        pos: new_pos,
                        dir: (-self.dir.1, -self.dir.0),
                    }),
                    None,
                )
            }
            Cell::Mirror2 => {
                // reflect at /
                (
                    Some(Beam {
                        pos: new_pos,
                        dir: (self.dir.1, self.dir.0),
                    }),
                    None,
                )
            }
            Cell::SplitterH => {
                if self.dir.0 == 0 {
                    // continue on
                    (
                        Some(Beam {
                            pos: new_pos,
                            dir: self.dir,
                        }),
                        None,
                    )
                } else {
                    // split
                    (
                        Some(Beam {
                            pos: new_pos,
                            dir: (0, 1),
                        }),
                        Some(Beam {
                            pos: new_pos,
                            dir: (0, -1),
                        }),
                    )
                }
            }
            Cell::SplitterV => {
                if self.dir.1 == 0 {
                    // continue on
                    (
                        Some(Beam {
                            pos: new_pos,
                            dir: self.dir,
                        }),
                        None,
                    )
                } else {
                    // split
                    (
                        Some(Beam {
                            pos: new_pos,
                            dir: (1, 0),
                        }),
                        Some(Beam {
                            pos: new_pos,
                            dir: (-1, 0),
                        }),
                    )
                }
            }
        }
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
                        '/' => Cell::Mirror1,
                        '\\' => Cell::Mirror2,
                        '-' => Cell::SplitterH,
                        '|' => Cell::SplitterV,
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
    input.energize(Beam {
        pos: (0, -1), // starting from the top, moving right
        dir: (0, 1),
    })
}

fn solve_stage2(input: &Grid) -> u64 {
    // starting from the top
    (0..input.dims.1)
        .map(|x| {
            input.energize(Beam {
                pos: (-1, x as isize),
                dir: (1, 0),
            })
        })
        .chain(
            // starting from the bottom
            (0..input.dims.1).map(|x| {
                input.energize(Beam {
                    pos: (input.dims.0 as isize, x as isize),
                    dir: (-1, 0),
                })
            }),
        )
        .chain(
            // starting from the left
            (0..input.dims.0).map(|x| {
                input.energize(Beam {
                    pos: (x as isize, -1),
                    dir: (0, 1),
                })
            }),
        )
        .chain(
            // starting from the right
            (0..input.dims.0).map(|x| {
                input.energize(Beam {
                    pos: (x as isize, input.dims.1 as isize),
                    dir: (0, -1),
                })
            }),
        )
        .max()
        .unwrap()
}

pub struct Day16Solver;
impl AdventOfCodeDay<'_> for Day16Solver {
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

    use crate::Day16Solver;

    const TEST_INPUT: &str = r#".|...\....
|.-.\.....
.....|-...
........|.
..........
.........\
..../.\\..
.-.-/..|..
.|....-|.\
..//.|...."#;
    #[test]
    fn test_stage1() {
        let input = Day16Solver::parse_input(TEST_INPUT);
        assert_eq!(super::solve_stage1(&input), 46);
    }
    #[test]
    fn test_stage2() {
        let input = Day16Solver::parse_input(TEST_INPUT);
        assert_eq!(super::solve_stage2(&input), 51);
    }
}
