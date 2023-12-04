use aoc_traits::AdventOfCodeDay;
use color_eyre::Result;
use nom::{
    bytes::complete::tag,
    character::complete::{digit1, line_ending, space1},
    combinator::{all_consuming, map_res},
    multi::separated_list1,
    sequence::{delimited, separated_pair, terminated},
    IResult,
};

#[derive(Debug)]
pub struct Card {
    _id: u32,
    winning_numbers: Vec<u32>,
    numbers: Vec<u32>,
}

impl Card {
    fn win_value(&self) -> u32 {
        let num_winners = self
            .numbers
            .iter()
            .filter(|x| self.winning_numbers.contains(x))
            .count();

        match num_winners {
            0 => 0,
            x => 1 << (x - 1),
        }
    }

    fn winners(&self) -> u32 {
        self.numbers
            .iter()
            .filter(|x| self.winning_numbers.contains(x))
            .count() as u32
    }
}

fn parse_u32(input: &str) -> IResult<&str, u32> {
    map_res(digit1, str::parse::<u32>)(input)
}

fn parse_card(input: &str) -> IResult<&str, Card> {
    let (input, _) = terminated(tag("Card"), space1)(input)?;
    let (input, id) = parse_u32(input)?;
    let (input, _) = tag(":")(input)?;
    let (input, _) = space1(input)?;
    let (input, (winning_numbers, numbers)) = separated_pair(
        separated_list1(space1, parse_u32),
        delimited(space1, tag("|"), space1),
        separated_list1(space1, parse_u32),
    )(input)?;
    Ok((
        input,
        Card {
            _id: id,
            winning_numbers,
            numbers,
        },
    ))
}

fn parse_cards(input: &str) -> Result<Vec<Card>> {
    let (_, cards) = all_consuming(separated_list1(line_ending, parse_card))(input)
        .map_err(|e| color_eyre::eyre::eyre!("Failed to parse input: {}", e))?;
    Ok(cards)
}

fn solve_stage1(cards: &[Card]) -> u32 {
    cards.iter().map(|g| g.win_value()).sum::<u32>()
}
fn solve_stage2(cards: &[Card]) -> u32 {
    let mut winners: Vec<_> = cards.iter().map(|g| (1, g.winners())).collect();
    for i in 0..winners.len() {
        let wins = winners[i].1;
        let count = winners[i].0;
        for j in 0..wins as usize {
            if let Some(card) = winners.get_mut(i + j + 1) {
                card.0 += count;
            }
        }
    }
    winners.iter().map(|c| c.0).sum()
}

pub struct Day4Solver;
impl AdventOfCodeDay<'_> for Day4Solver {
    type ParsedInput = Vec<Card>;

    type Part1Output = u64;

    type Part2Output = u64;

    fn solve_part1(input: &Self::ParsedInput) -> Self::Part1Output {
        solve_stage1(input).into()
    }

    fn solve_part2(input: &Self::ParsedInput) -> Self::Part2Output {
        solve_stage2(input).into()
    }

    fn parse_input(input: &'_ str) -> Self::ParsedInput {
        parse_cards(input).unwrap()
    }
}

#[cfg(test)]
mod tests {

    const TEST_INPUT: &str = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";
    #[test]
    fn test_stage1() {
        let cards = super::parse_cards(TEST_INPUT).unwrap();
        assert_eq!(super::solve_stage1(&cards), 13);
    }
    #[test]
    fn test_stage2() {
        let cards = super::parse_cards(TEST_INPUT).unwrap();
        assert_eq!(super::solve_stage2(&cards), 30);
    }
}
