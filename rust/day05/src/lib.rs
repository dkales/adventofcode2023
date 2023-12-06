use std::ops::Range;

use aoc_traits::AdventOfCodeDay;
use color_eyre::{
    eyre::{self},
    Result,
};
use nom::{
    bytes::complete::tag,
    character::complete::{alpha1, digit1, line_ending, space1},
    combinator::{all_consuming, map_res},
    multi::{count, separated_list1},
    sequence::{delimited, terminated, tuple},
    IResult,
};

#[derive(Debug)]
pub struct MappingRange {
    range: Range<u64>,
    map_start: u64,
}

impl MappingRange {
    fn map(&self, input: u64) -> Option<u64> {
        if self.range.contains(&input) {
            Some(input - self.range.start + self.map_start)
        } else {
            None
        }
    }
}

fn apply_ranges_to_range(mapping_ranges: &[MappingRange], range: Range<u64>) -> Vec<Range<u64>> {
    let mut result = Vec::new();
    let mut ranges = vec![range];
    for map_range in mapping_ranges {
        // find the overlap of the two ranges
        ranges = ranges
            .into_iter()
            .flat_map(|r| {
                let maybe_map = map_range.range.start.max(r.start)..map_range.range.end.min(r.end);
                if maybe_map.is_empty() {
                    return vec![r];
                }

                //otherwise, map the overlap
                result.push(
                    maybe_map.start - map_range.range.start + map_range.map_start
                        ..maybe_map.end - map_range.range.start + map_range.map_start,
                );
                vec![
                    r.start..maybe_map.start, // do not map this
                    maybe_map.end..r.end,     // do not map this
                ]
            })
            .filter(|r| !r.is_empty()) // throw out empty ranges
            .collect();
    }
    // all unmapped ranges go into result
    result.extend(ranges);
    result
}
fn apply_ranges(ranges: &[MappingRange], input: u64) -> u64 {
    ranges.iter().find_map(|r| r.map(input)).unwrap_or(input)
}

fn parse_u64(input: &str) -> IResult<&str, u64> {
    map_res(digit1, str::parse::<u64>)(input)
}

fn parse_mapping(input: &str) -> IResult<&str, MappingRange> {
    let (input, (map_start, _, range_start, _, range_size)) =
        tuple((parse_u64, space1, parse_u64, space1, parse_u64))(input)?;
    Ok((
        input,
        MappingRange {
            range: range_start..(range_start + range_size),
            map_start,
        },
    ))
}

fn parse_mapping_block(input: &str) -> IResult<&str, Vec<MappingRange>> {
    let (input, (_from, _, _to, _)) = terminated(
        tuple((alpha1, tag("-to-"), alpha1, tag(" map:"))),
        line_ending,
    )(input)?;
    separated_list1(line_ending, parse_mapping)(input)
}

#[derive(Debug)]
pub struct Game {
    seeds: Vec<u64>,
    mappings: Vec<Vec<MappingRange>>,
}

fn parse_game(input: &str) -> IResult<&str, Game> {
    let (input, seeds) = delimited(
        tag("seeds: "),
        separated_list1(space1, parse_u64),
        line_ending,
    )(input)?;
    // skip a line
    let (input, _) = line_ending(input)?;

    let (input, mappings) =
        all_consuming(separated_list1(count(line_ending, 2), parse_mapping_block))(input)?;

    Ok((input, Game { seeds, mappings }))
}

fn parse(input: &str) -> Result<Game> {
    parse_game(input)
        .map_err(|e| eyre::eyre!("Failed to parse input: {}", e))
        .map(|x| x.1)
}

fn apply_mappings(mappings: &[Vec<MappingRange>], input: u64) -> u64 {
    let mut s = input;
    for mapping in mappings {
        s = apply_ranges(mapping, s);
    }
    s
}
fn apply_mappings_to_ranges(
    mappings: &[Vec<MappingRange>],
    ranges: Vec<Range<u64>>,
) -> Vec<Range<u64>> {
    let mut s = ranges;
    for mapping in mappings {
        s = s
            .into_iter()
            .flat_map(|r| apply_ranges_to_range(mapping, r))
            .collect();
    }
    s
}

fn solve_stage1(input: &Game) -> u64 {
    input
        .seeds
        .iter()
        .map(|&s| apply_mappings(&input.mappings, s))
        .min()
        .unwrap_or_default()
}

fn solve_stage2(input: &Game) -> u64 {
    let ranges = input.seeds.chunks(2).map(|s| s[0]..(s[0] + s[1])).collect();
    let mapped_ranges = apply_mappings_to_ranges(&input.mappings, ranges);
    mapped_ranges
        .iter()
        .map(|r| r.start)
        .min()
        .unwrap_or_default()
}

pub struct Day5Solver;
impl AdventOfCodeDay<'_> for Day5Solver {
    type ParsedInput = Game;

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

    const TEST_INPUT: &str = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";
    #[test]
    fn test_stage1() {
        let input = super::parse(TEST_INPUT).unwrap();
        assert_eq!(super::solve_stage1(&input), 35);
    }
    #[test]
    fn test_stage2() {
        let input = super::parse(TEST_INPUT).unwrap();
        assert_eq!(super::solve_stage2(&input), 46);
    }
}
