use aoc_traits::AdventOfCodeDay;

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::anychar,
    combinator::{all_consuming, opt, peek, value},
    multi::separated_list1,
    IResult,
};

fn solve_stage1(input: &str) -> u32 {
    input
        .lines()
        .map(|s| {
            s.chars().find_map(|c| c.to_digit(10)).unwrap() * 10
                + s.chars().rev().find_map(|c| c.to_digit(10)).unwrap()
        })
        .sum()
}

// TBH this solution is a bit of a mess since the spec is super unclear.
// It seems that actually "oneight" should be both 1 and 8 and not just 1
// Since I first wanted to do this with nom to get used to it again, I stuck with it
// Therefore we actually just peek and try to parse, then skip a char
// It seems the whole task is not really suited to nom
// a better idea would probably to use a kind of match tree, but on byte strings
// and just look for the first and last one
fn parse_stage2_line(input: &str) -> u32 {
    let x: IResult<_, Vec<Option<u32>>> = all_consuming(separated_list1(
        anychar,
        opt(peek(alt((
            value(1, alt((tag("one"), tag("1")))),
            value(2, alt((tag("two"), tag("2")))),
            value(3, alt((tag("three"), tag("3")))),
            value(4, alt((tag("four"), tag("4")))),
            value(5, alt((tag("five"), tag("5")))),
            value(6, alt((tag("six"), tag("6")))),
            value(7, alt((tag("seven"), tag("7")))),
            value(8, alt((tag("eight"), tag("8")))),
            value(9, alt((tag("nine"), tag("9")))),
        )))),
    ))(input);
    let (_, a) = x.unwrap();

    a.iter().find(|x| x.is_some()).unwrap().unwrap() * 10
        + a.iter().rev().find(|x| x.is_some()).unwrap().unwrap()
}

fn solve_stage2(input: &str) -> u32 {
    input.lines().map(parse_stage2_line).sum()
}

#[derive(Default)]
pub struct Day1Solver;
impl<'a> AdventOfCodeDay<'a> for Day1Solver {
    type ParsedInput = &'a str;
    type Part1Output = u32;
    type Part2Output = u32;

    fn parse_input(input: &str) -> &str {
        input
    }

    fn solve_part1(input: &Self::ParsedInput) -> Self::Part1Output {
        solve_stage1(input)
    }
    fn solve_part2(input: &Self::ParsedInput) -> Self::Part2Output {
        solve_stage2(input)
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_stage1() {
        const TEST_INPUT: &str = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";
        assert_eq!(super::solve_stage1(TEST_INPUT), 142);
    }
    #[test]
    fn test_stage2() {
        const TEST_INPUT: &str = "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";
        assert_eq!(super::solve_stage2(TEST_INPUT), 281);
    }
}
