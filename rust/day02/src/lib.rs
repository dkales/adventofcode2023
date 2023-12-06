use aoc_traits::AdventOfCodeDay;
use color_eyre::Result;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{digit1, line_ending, space1},
    combinator::{all_consuming, map_res},
    multi::separated_list1,
    sequence::separated_pair,
    IResult,
};

#[derive(Clone)]
struct Cubes {
    red: u32,
    green: u32,
    blue: u32,
}
impl Cubes {
    fn power(&self) -> u32 {
        self.red * self.green * self.blue
    }
}

#[derive(Clone)]
pub struct Game {
    id: u32,
    cube_subsets: Vec<Cubes>,
}
impl Game {
    fn is_producable_by(&self, total_cubes: &Cubes) -> bool {
        self.cube_subsets.iter().all(|o| {
            o.red <= total_cubes.red && o.blue <= total_cubes.blue && o.green <= total_cubes.green
        })
    }
    fn get_min_cubes(&self) -> Cubes {
        self.cube_subsets.iter().fold(
            Cubes {
                red: u32::MIN,
                green: u32::MIN,
                blue: u32::MIN,
            },
            |acc, o| Cubes {
                red: acc.red.max(o.red),
                green: acc.green.max(o.green),
                blue: acc.blue.max(o.blue),
            },
        )
    }
}

fn parse_cubes(input: &str) -> IResult<&str, Cubes> {
    let (input, cube) = separated_list1(
        tag(", "),
        separated_pair(
            map_res(digit1, str::parse::<u32>),
            space1,
            alt((tag("red"), tag("blue"), tag("green"))),
        ),
    )(input)?;

    Ok((
        input,
        Cubes {
            red: cube
                .iter()
                .find(|(_, color)| color == &"red")
                .map(|x| x.0)
                .unwrap_or_default(),
            green: cube
                .iter()
                .find(|(_, color)| color == &"green")
                .map(|x| x.0)
                .unwrap_or_default(),
            blue: cube
                .iter()
                .find(|(_, color)| color == &"blue")
                .map(|x| x.0)
                .unwrap_or_default(),
        },
    ))
}

fn parse_game(input: &str) -> IResult<&str, Game> {
    let (input, _) = tag("Game ")(input)?;
    let (input, id) = map_res(digit1, str::parse::<u32>)(input)?;
    let (input, _) = tag(": ")(input)?;
    let (input, cube_subsets) = separated_list1(tag("; "), parse_cubes)(input)?;
    Ok((input, Game { id, cube_subsets }))
}

fn parse_games(input: &str) -> Result<Vec<Game>> {
    let (_, games) = all_consuming(separated_list1(line_ending, parse_game))(input)
        .map_err(|e| color_eyre::eyre::eyre!("Failed to parse input: {}", e))?;
    Ok(games)
}

fn solve_stage1(games: &[Game]) -> u32 {
    let total_cubes = Cubes {
        red: 12,
        green: 13,
        blue: 14,
    };
    games
        .iter()
        .filter(|g| g.is_producable_by(&total_cubes))
        .map(|g| g.id)
        .sum::<u32>()
}
fn solve_stage2(games: &[Game]) -> u32 {
    games
        .iter()
        .map(|g| g.get_min_cubes())
        .map(|c| c.power())
        .sum::<u32>()
}

pub struct Day2Solver;
impl AdventOfCodeDay<'_> for Day2Solver {
    type ParsedInput = Vec<Game>;

    type Part1Output = u64;

    type Part2Output = u64;

    fn solve_part1(input: &Self::ParsedInput) -> Self::Part1Output {
        solve_stage1(input).into()
    }

    fn solve_part2(input: &Self::ParsedInput) -> Self::Part2Output {
        solve_stage2(input).into()
    }

    fn parse_input(input: &'_ str) -> Self::ParsedInput {
        parse_games(input).unwrap()
    }
}

#[cfg(test)]
mod tests {

    const TEST_INPUT: &str = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
    #[test]
    fn test_stage1() {
        let games = super::parse_games(TEST_INPUT).unwrap();
        assert_eq!(super::solve_stage1(&games), 8);
    }
    #[test]
    fn test_stage2() {
        let games = super::parse_games(TEST_INPUT).unwrap();
        assert_eq!(super::solve_stage2(&games), 2286);
    }
}
