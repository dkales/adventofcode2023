use std::cell::OnceCell;

use aoc_traits::AdventOfCodeDay;
use color_eyre::{
    eyre::{self},
    Result,
};
use nom::{
    bytes::complete::tag,
    character::complete::{digit1, line_ending, space1},
    combinator::{all_consuming, map, map_res, opt, recognize},
    multi::separated_list1,
    sequence::tuple,
    IResult,
};

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Game {
    values: Vec<i64>,
    sequences: OnceCell<Vec<Vec<i64>>>,
}

impl Game {
    fn seqs(&self) -> &Vec<Vec<i64>> {
        self.sequences.get_or_init(|| {
            let mut seqs = vec![self.values.clone()];
            while !seqs.last().unwrap().iter().all(|&x| x == 0) {
                let new_seq = seqs
                    .last()
                    .unwrap()
                    .as_slice()
                    .windows(2)
                    .map(|x| x[1] - x[0])
                    .collect();
                seqs.push(new_seq);
            }
            seqs
        })
    }
    fn extend(&self) -> i64 {
        self.seqs()
            .iter()
            .rev()
            .skip(1)
            .fold(0, |acc, x| acc + x.last().unwrap())
    }
    fn extend_back(&self) -> i64 {
        self.seqs()
            .iter()
            .rev()
            .skip(1)
            .fold(0, |acc, x| x.first().unwrap() - acc)
    }
}

fn parse_i64(input: &str) -> IResult<&str, i64> {
    map_res(recognize(tuple((opt(tag("-")), digit1))), str::parse::<i64>)(input)
}

fn parse_games(input: &str) -> IResult<&str, Vec<Game>> {
    let (input, hands) = all_consuming(separated_list1(
        line_ending,
        map(separated_list1(space1, parse_i64), |values| Game {
            values,
            sequences: OnceCell::new(),
        }),
    ))(input)?;

    Ok((input, hands))
}

fn parse(input: &str) -> Result<Vec<Game>> {
    parse_games(input)
        .map_err(|e| eyre::eyre!("Failed to parse input: {}", e))
        .map(|x| x.1)
}

fn solve_stage1(input: &[Game]) -> i64 {
    input.iter().map(|g| g.extend()).sum()
}

fn solve_stage2(input: &[Game]) -> i64 {
    input.iter().map(|g| g.extend_back()).sum()
}

pub struct Day9Solver;
impl AdventOfCodeDay<'_> for Day9Solver {
    type ParsedInput = Vec<Game>;

    type Part1Output = i64;

    type Part2Output = i64;

    fn solve_part1(input: &Self::ParsedInput) -> Self::Part1Output {
        solve_stage1(input)
    }

    fn solve_part2(input: &Self::ParsedInput) -> Self::Part2Output {
        solve_stage2(input)
    }

    fn parse_input(input: &'_ str) -> Self::ParsedInput {
        parse(input).unwrap()
    }
}

#[cfg(test)]
mod tests {

    const TEST_INPUT: &str = "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45";
    #[test]
    fn test_stage1() {
        let input = super::parse(TEST_INPUT).unwrap();
        assert_eq!(super::solve_stage1(&input), 114);
    }
    #[test]
    fn test_stage2() {
        let input = super::parse(TEST_INPUT).unwrap();
        assert_eq!(super::solve_stage2(&input), 2);
    }
}
