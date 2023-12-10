use std::{str::FromStr, vec};

use aoc_traits::AdventOfCodeDay;
use color_eyre::{
    eyre::{self, Error},
    Result,
};

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Game {
    grid: Vec<Vec<Cell>>,
    start: (usize, usize),
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Cell {
    Empty,
    Start,
    NS,
    WE,
    NE,
    NW,
    SE,
    SW,
}

impl Game {
    fn next_cell(&self, from: (usize, usize), cur: (usize, usize)) -> Option<(usize, usize)> {
        let at_north_border = cur.0 == 0;
        let at_south_border = cur.0 == self.grid.len() - 1;
        let at_west_border = cur.1 == 0;
        let at_east_border = cur.1 == self.grid[0].len() - 1;
        let coming_from_north = from.0 + 1 == cur.0 && from.1 == cur.1;
        let coming_from_south = from.0 == cur.0 + 1 && from.1 == cur.1;
        let coming_from_west = from.0 == cur.0 && from.1 + 1 == cur.1;
        let coming_from_east = from.0 == cur.0 && from.1 == cur.1 + 1;
        match self.grid[cur.0][cur.1] {
            // NS, moving down
            Cell::NS if !at_south_border && coming_from_north => Some((cur.0 + 1, cur.1)),
            // NS, moving up
            Cell::NS if !at_north_border && coming_from_south => Some((cur.0 - 1, cur.1)),
            // WE, moving right
            Cell::WE if !at_east_border && coming_from_west => Some((cur.0, cur.1 + 1)),
            // WE, moving left
            Cell::WE if !at_west_border && coming_from_east => Some((cur.0, cur.1 - 1)),
            // NE, moving up
            Cell::NE if !at_north_border && coming_from_east => Some((cur.0 - 1, cur.1)),
            // NE, moving right
            Cell::NE if !at_east_border && coming_from_north => Some((cur.0, cur.1 + 1)),
            // NW, moving up
            Cell::NW if !at_north_border && coming_from_west => Some((cur.0 - 1, cur.1)),
            // NW, moving left
            Cell::NW if !at_west_border && coming_from_north => Some((cur.0, cur.1 - 1)),
            // SE, moving down
            Cell::SE if !at_south_border && coming_from_east => Some((cur.0 + 1, cur.1)),
            // SE, moving right
            Cell::SE if !at_east_border && coming_from_south => Some((cur.0, cur.1 + 1)),
            // SW, moving down
            Cell::SW if !at_south_border && coming_from_west => Some((cur.0 + 1, cur.1)),
            // SW, moving left
            Cell::SW if !at_west_border && coming_from_south => Some((cur.0, cur.1 - 1)),
            _ => None,
        }
    }
}

impl FromStr for Game {
    type Err = Error;

    fn from_str(s: &str) -> std::prelude::v1::Result<Self, Self::Err> {
        let mut start = (0, 0);
        let grid = s
            .lines()
            .enumerate()
            .map(|(i, l)| {
                l.chars()
                    .enumerate()
                    .map(|(j, c)| match c {
                        '.' => Ok(Cell::Empty),
                        '-' => Ok(Cell::WE),
                        '|' => Ok(Cell::NS),
                        'S' => {
                            start = (i, j);
                            Ok(Cell::Start)
                        }
                        'L' => Ok(Cell::NE),
                        'J' => Ok(Cell::NW),
                        'F' => Ok(Cell::SE),
                        '7' => Ok(Cell::SW),
                        _ => Err(eyre::eyre!("Invalid char: {}", c)),
                    })
                    .collect::<Result<Vec<_>, _>>()
            })
            .collect::<Result<Vec<_>, _>>()?;

        Ok(Game { grid, start })
    }
}

fn solve_stage1(input: &Game) -> i64 {
    let mut loop_len = 0;
    for offset in [(0, 1), (1, 0), (0, -1), (-1, 0)] {
        loop_len = 0;
        let mut cur = (
            (input.start.0 as isize + offset.0) as usize,
            (input.start.1 as isize + offset.1) as usize,
        );
        let mut from = input.start;
        while let Some(next) = input.next_cell(from, cur) {
            from = cur;
            cur = next;
            loop_len += 1;
        }
        if cur == input.start {
            break;
        }
    }
    (loop_len + 1) / 2
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Phase2Cell {
    Wall,
    Outside,
    Unknown,
}

fn solve_stage2(input: &Game) -> i64 {
    let mut loop_path = vec![];
    for offset in [(0, 1), (1, 0), (0, -1), (-1, 0)] {
        let mut from = input.start;
        let mut cur = (
            (input.start.0 as isize + offset.0) as usize,
            (input.start.1 as isize + offset.1) as usize,
        );
        loop_path = vec![from, cur];
        while let Some(next) = input.next_cell(from, cur) {
            from = cur;
            cur = next;
            loop_path.push(cur);
        }
        if cur == input.start {
            break;
        }
    }
    // now that we have the loop, find all enclosed cells
    // extend the playing field by a factor of 2, to have spaces in between
    let (x_dim, y_dim) = (input.grid.len() * 2 + 1, input.grid[0].len() * 2 + 1);
    let mut grid = vec![vec![Phase2Cell::Unknown; y_dim]; x_dim];
    // mark all walls
    for x in loop_path.windows(2) {
        let (x1, y1) = (x[0].0 * 2 + 1, x[0].1 * 2 + 1);
        let (x3, y3) = (x[1].0 * 2 + 1, x[1].1 * 2 + 1);
        let (x2, y2) = ((x1 + x3) / 2, (y1 + y3) / 2);
        grid[x1][y1] = Phase2Cell::Wall;
        grid[x2][y2] = Phase2Cell::Wall;
        grid[x3][y3] = Phase2Cell::Wall;
    }
    //print_grid(&grid);
    //print_grid_ext(&grid);
    // mark all border cells as outside
    #[allow(clippy::needless_range_loop)] // i find the loops to be clearer
    for i in 0..x_dim {
        if grid[i][0] == Phase2Cell::Unknown {
            grid[i][0] = Phase2Cell::Outside;
        }
        if grid[i][y_dim - 1] == Phase2Cell::Unknown {
            grid[i][y_dim - 1] = Phase2Cell::Outside;
        }
    }
    for i in 0..y_dim {
        if grid[0][i] == Phase2Cell::Unknown {
            grid[0][i] = Phase2Cell::Outside;
        }
        if grid[x_dim - 1][i] == Phase2Cell::Unknown {
            grid[x_dim - 1][i] = Phase2Cell::Outside;
        }
    }
    // propagate outside cells
    let mut changed = true;
    while changed {
        changed = false;
        // we handled the border already, its either outside or wall
        for i in 1..x_dim - 1 {
            for j in 1..y_dim - 1 {
                if grid[i][j] == Phase2Cell::Unknown {
                    // if an unknown cell is touching an inside cell, it's inside
                    // same for outside
                    let mut touching_outside = false;
                    for offset in [(0, 1), (1, 0), (0, -1), (-1, 0)] {
                        let (x, y) = (
                            (i as isize + offset.0) as usize,
                            (j as isize + offset.1) as usize,
                        );
                        if grid[x][y] == Phase2Cell::Outside {
                            touching_outside = true;
                        }
                    }
                    if touching_outside {
                        grid[i][j] = Phase2Cell::Outside;
                        changed = true;
                    }
                }
            }
        }
    }
    //print_grid(&grid);
    //print_grid_ext(&grid);
    // all unknown cells are inside, but filter out the extended cells
    grid.iter()
        .skip(1)
        .step_by(2)
        .flat_map(|x| x.iter().skip(1).step_by(2))
        .filter(|&&x| x == Phase2Cell::Unknown)
        .count() as i64
}

#[allow(unused)]
fn print_grid_ext(grid: &[Vec<Phase2Cell>]) {
    for row in grid {
        for cell in row {
            match cell {
                Phase2Cell::Wall => print!("#"),
                Phase2Cell::Outside => print!("O"),
                Phase2Cell::Unknown => print!("?"),
            }
        }
        println!();
    }
}

#[allow(unused)]
fn print_grid(grid: &[Vec<Phase2Cell>]) {
    for row in grid.iter().skip(1).step_by(2) {
        for cell in row.iter().skip(1).step_by(2) {
            match cell {
                Phase2Cell::Wall => print!("#"),
                Phase2Cell::Outside => print!("O"),
                Phase2Cell::Unknown => print!("?"),
            }
        }
        println!();
    }
}

pub struct Day10Solver;
impl AdventOfCodeDay<'_> for Day10Solver {
    type ParsedInput = Game;

    type Part1Output = i64;

    type Part2Output = i64;

    fn solve_part1(input: &Self::ParsedInput) -> Self::Part1Output {
        solve_stage1(input)
    }

    fn solve_part2(input: &Self::ParsedInput) -> Self::Part2Output {
        solve_stage2(input)
    }

    fn parse_input(input: &'_ str) -> Self::ParsedInput {
        Game::from_str(input).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use crate::Game;

    #[test]
    fn test_stage1() {
        const TEST_INPUT: &str = "-L|F7
7S-7|
L|7||
-L-J|
L|-JF";
        let input = Game::from_str(TEST_INPUT).unwrap();
        assert_eq!(super::solve_stage1(&input), 4);
    }
    #[test]
    fn test_stage1_alt() {
        const TEST_INPUT: &str = "7-F7-
.FJ|7
SJLL7
|F--J
LJ.LJ";
        let input = Game::from_str(TEST_INPUT).unwrap();
        assert_eq!(super::solve_stage1(&input), 8);
    }
    #[test]
    fn test_stage2() {
        const TEST_INPUT: &str = "...........
.S-------7.
.|F-----7|.
.||.....||.
.||.....||.
.|L-7.F-J|.
.|..|.|..|.
.L--J.L--J.
...........";
        let input = Game::from_str(TEST_INPUT).unwrap();
        assert_eq!(super::solve_stage2(&input), 4);
    }
    #[test]
    fn test_stage2_alt() {
        const TEST_INPUT: &str = ".F----7F7F7F7F-7....
.|F--7||||||||FJ....
.||.FJ||||||||L7....
FJL7L7LJLJ||LJ.L-7..
L--J.L7...LJS7F-7L7.
....F-J..F7FJ|L7L7L7
....L7.F7||L7|.L7L7|
.....|FJLJ|FJ|F7|.LJ
....FJL-7.||.||||...
....L---J.LJ.LJLJ...";
        let input = Game::from_str(TEST_INPUT).unwrap();
        assert_eq!(super::solve_stage2(&input), 8);
    }
    #[test]
    fn test_stage2_alt2() {
        const TEST_INPUT: &str = "FF7FSF7F7F7F7F7F---7
L|LJ||||||||||||F--J
FL-7LJLJ||||||LJL-77
F--JF--7||LJLJ7F7FJ-
L---JF-JLJ.||-FJLJJ7
|F|F-JF---7F7-L7L|7|
|FFJF7L7F-JF7|JL---7
7-L-JL7||F7|L7F-7F7|
L.L7LFJ|||||FJL7||LJ
L7JLJL-JLJLJL--JLJ.L";
        let input = Game::from_str(TEST_INPUT).unwrap();
        assert_eq!(super::solve_stage2(&input), 10);
    }
}
