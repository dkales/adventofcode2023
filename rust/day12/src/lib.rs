use std::{collections::VecDeque, str::FromStr};

use aoc_traits::AdventOfCodeDay;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Spring {
    Working,
    Broken,
    Unknown,
}

pub struct Field {
    springs: Vec<Spring>,
    chunks: Vec<usize>,
}
impl FromStr for Field {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (springs, chunks) = s.split_once(' ').unwrap();
        let springs = springs
            .chars()
            .map(|c| match c {
                '.' => Spring::Working,
                '#' => Spring::Broken,
                '?' => Spring::Unknown,
                _ => unreachable!(),
            })
            .collect();
        let chunks = chunks.split(',').map(|x| x.parse().unwrap()).collect();
        Ok(Field { springs, chunks })
    }
}

fn is_valid(springs: &[Spring], chunks: &[usize]) -> bool {
    let mut found_chunks = Vec::with_capacity(chunks.len());
    let mut cur = None;
    for (i, s) in springs.iter().enumerate() {
        if *s == Spring::Working && cur.is_some() {
            found_chunks.push(i - cur.unwrap());
            cur = None;
        } else if *s == Spring::Broken && cur.is_none() {
            cur = Some(i);
        }
    }
    if cur != None {
        found_chunks.push(springs.len() - cur.unwrap());
    }
    found_chunks == chunks
}

impl Field {
    fn possible_solutions(&self) -> u64 {
        let variants = self
            .springs
            .iter()
            .filter(|x| **x == Spring::Unknown)
            .count();
        let variants = 1usize << variants;
        let mut solutions = VecDeque::new();
        solutions.push_back(self.springs.clone());
        while solutions.len() != variants {
            let mut current = solutions.pop_front().unwrap();
            let pos = current.iter().position(|x| *x == Spring::Unknown).unwrap();
            let mut new = current.clone();
            current[pos] = Spring::Working;
            new[pos] = Spring::Broken;
            solutions.push_back(current);
            solutions.push_back(new);
        }

        solutions
            .into_iter()
            .filter(|x| is_valid(x, &self.chunks))
            .count() as u64
    }
}

fn solve_stage1(input: &[Field]) -> u64 {
    input.iter().map(|x| x.possible_solutions()).sum()
}

fn solve_stage2(_input: &[Field]) -> u64 {
    0
}

pub struct Day12Solver;
impl AdventOfCodeDay<'_> for Day12Solver {
    type ParsedInput = Vec<Field>;

    type Part1Output = u64;

    type Part2Output = u64;

    fn solve_part1(input: &Self::ParsedInput) -> Self::Part1Output {
        solve_stage1(input)
    }

    fn solve_part2(input: &Self::ParsedInput) -> Self::Part2Output {
        solve_stage2(input)
    }

    fn parse_input(input: &str) -> Self::ParsedInput {
        input.lines().map(|x| x.parse().unwrap()).collect()
    }
}

#[cfg(test)]
mod tests {
    use aoc_traits::AdventOfCodeDay;

    use crate::Day12Solver;

    const TEST_INPUT: &str = "???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1";
    #[test]
    fn test_stage1() {
        let input = Day12Solver::parse_input(TEST_INPUT);
        assert_eq!(super::solve_stage1(&input), 21);
    }
    // #[test]
    // only works for the 100 factor
    // fn test_stage2() {
    //     let input = Day11Solver::parse_input(TEST_INPUT);
    //     assert_eq!(super::solve_stage2(&input), 8410);
    // }
}
