use aoc_traits::AdventOfCodeDay;
use color_eyre::{
    eyre::{self},
    Result,
};
use nom::{
    bytes::complete::tag,
    character::complete::{digit1, line_ending, space1},
    combinator::{map_res, opt},
    multi::separated_list1,
    sequence::{delimited, terminated},
    IResult,
};

#[derive(Debug)]
pub struct Game {
    time: u64,
    distance: u64,
}
impl Game {
    fn ways_to_beat(&self) -> u64 {
        (1..self.time)
            .map(|t| (self.time - t) * t)
            .filter(|x| *x > self.distance)
            .count() as u64
    }
}

fn parse_u64(input: &str) -> IResult<&str, u64> {
    map_res(digit1, str::parse::<u64>)(input)
}

fn parse_games(input: &str) -> IResult<&str, Vec<Game>> {
    let (input, times) = delimited(
        terminated(tag("Time:"), space1),
        separated_list1(space1, parse_u64),
        line_ending,
    )(input)?;
    let (input, dists) = delimited(
        terminated(tag("Distance:"), space1),
        separated_list1(space1, parse_u64),
        opt(line_ending),
    )(input)?;
    // skip a line
    let games = times
        .into_iter()
        .zip(dists)
        .map(|(t, r)| Game {
            time: t,
            distance: r,
        })
        .collect();

    Ok((input, games))
}

fn merge_games(input: &[Game]) -> Game {
    let (time, dist) = input
        .iter()
        .fold((String::new(), String::new()), |(t, d), g| {
            (t + &g.time.to_string(), d + &g.distance.to_string())
        });
    Game {
        time: time.parse().unwrap(),
        distance: dist.parse().unwrap(),
    }
}

fn parse(input: &str) -> Result<Vec<Game>> {
    parse_games(input)
        .map_err(|e| eyre::eyre!("Failed to parse input: {}", e))
        .map(|x| x.1)
}

fn solve_stage1(input: &[Game]) -> u64 {
    input.iter().map(|g| g.ways_to_beat()).product()
}

fn solve_stage2(input: &[Game]) -> u64 {
    let game = merge_games(input);
    game.ways_to_beat()
}

pub struct Day6Solver;
impl AdventOfCodeDay<'_> for Day6Solver {
    type ParsedInput = Vec<Game>;

    type Part1Output = u64;

    type Part2Output = u64;

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

    const TEST_INPUT: &str = "Time:      7  15   30
Distance:  9  40  200";
    #[test]
    fn test_stage1() {
        let input = super::parse(TEST_INPUT).unwrap();
        assert_eq!(super::solve_stage1(&input), 288);
    }
    #[test]
    fn test_stage2() {
        let input = super::parse(TEST_INPUT).unwrap();
        assert_eq!(super::solve_stage2(&input), 46);
    }
}
