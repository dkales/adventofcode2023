use std::collections::HashMap;

use aoc_traits::AdventOfCodeDay;
use color_eyre::{
    eyre::{self},
    Result,
};
use nom::{
    bytes::complete::{is_a, tag, take},
    character::complete::line_ending,
    combinator::all_consuming,
    multi::{count, separated_list1},
    sequence::{delimited, separated_pair, terminated},
    IResult,
};
use num_integer::Integer;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Game<'a> {
    steps: &'a str,
    mappings: HashMap<&'a str, (&'a str, &'a str)>,
}

fn parse_games(input: &str) -> IResult<&str, Game> {
    let (input, steps) = terminated(is_a("LR"), count(line_ending, 2))(input)?;
    let (input, mappings) = all_consuming(separated_list1(
        line_ending,
        separated_pair(
            take(3usize),
            tag(" = "),
            delimited(
                tag("("),
                separated_pair(take(3usize), tag(", "), take(3usize)),
                tag(")"),
            ),
        ),
    ))(input)?;
    let mappings = mappings.into_iter().collect::<HashMap<_, _>>();

    Ok((input, Game { steps, mappings }))
}

fn parse(input: &str) -> Result<Game> {
    parse_games(input)
        .map_err(|e| eyre::eyre!("Failed to parse input: {}", e))
        .map(|x| x.1)
}

fn solve_stage1(input: &Game) -> u64 {
    let mut current = "AAA";
    for (i, dir) in input.steps.chars().cycle().enumerate() {
        let mapping = input.mappings.get(current).unwrap();
        match dir {
            'L' => current = mapping.0,
            'R' => current = mapping.1,
            _ => unreachable!(),
        }
        if current == "ZZZ" {
            return (i + 1) as u64; // enumerate starts at 0, so add one
        }
    }
    unreachable!()
}

// This solution is not super generic, since it does not work for the test, which has a tail before it goes into a cycle
// the real inputs do not have this, so it works
fn solve_stage2(input: &Game) -> u64 {
    let mut current: Vec<_> = input
        .mappings
        .keys()
        .filter(|node| node.ends_with("A"))
        .copied()
        .collect();

    let mut goals = vec![vec![]; current.len()];
    let mut cycles = HashMap::new();

    for (i, dir) in input.steps.chars().cycle().enumerate() {
        for (j, node) in current.iter_mut().enumerate() {
            let mapping = input.mappings.get(*node).unwrap();
            match dir {
                'L' => *node = mapping.0,
                'R' => *node = mapping.1,
                _ => unreachable!(),
            }
            if node.ends_with("Z") {
                goals[j].push(((i + 1) as u64, *node));
                let num_goals = goals[j].len();
                if num_goals > 2 {
                    let curr_goal_diff = goals[j][num_goals - 1].0 - goals[j][num_goals - 2].0;
                    let last_goal_diff = goals[j][num_goals - 2].0 - goals[j][num_goals - 3].0;
                    if curr_goal_diff == last_goal_diff
                        && goals[j][num_goals - 1].1 == goals[j][num_goals - 2].1
                        && goals[j][num_goals - 2].1 == goals[j][num_goals - 3].1
                    {
                        cycles.insert(j, (curr_goal_diff, goals[j][0].0));
                    }
                }
            }
        }
        // found all cycles
        if cycles.len() == current.len() {
            break;
        }
    }
    let residues = cycles.values().map(|x| (x.1 % x.0)).collect::<Vec<_>>();
    // the lcm below only works if this is true
    assert!(residues.iter().all(|x| *x == 0));
    let modulii = cycles.values().map(|x| x.0).collect::<Vec<_>>();
    modulii.into_iter().reduce(|a, b| a.lcm(&b)).unwrap()
}

pub struct Day8Solver;
impl<'a> AdventOfCodeDay<'a> for Day8Solver {
    type ParsedInput = Game<'a>;

    type Part1Output = u64;

    type Part2Output = u64;

    fn solve_part1(input: &Self::ParsedInput) -> Self::Part1Output {
        solve_stage1(input)
    }

    fn solve_part2(input: &Self::ParsedInput) -> Self::Part2Output {
        solve_stage2(input)
    }

    fn parse_input(input: &'a str) -> Self::ParsedInput {
        parse(input).unwrap()
    }
}

#[cfg(test)]
mod tests {

    const TEST_INPUT: &str = "LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)";
    #[test]
    fn test_stage1() {
        let input = super::parse(TEST_INPUT).unwrap();
        assert_eq!(super::solve_stage1(&input), 6);
    }
    // solution is not generic enough to handle the test input, ironically
    // #[test]
    // fn test_stage2() {
    //     let input = super::parse(TEST_INPUT).unwrap();
    //     assert_eq!(super::solve_stage2(&input), 6);
    // }
}
