use std::collections::HashMap;

use aoc_traits::AdventOfCodeDay;
use color_eyre::{
    eyre::{self},
    Result,
};
use nom::{
    bytes::complete::take,
    character::complete::{digit1, line_ending, space1},
    combinator::{map, map_res},
    multi::separated_list1,
    sequence::separated_pair,
    IResult,
};

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Card {
    Joker,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
}

impl TryFrom<char> for Card {
    type Error = eyre::Error;

    fn try_from(c: char) -> std::prelude::v1::Result<Self, Self::Error> {
        match c {
            'A' => Ok(Card::Ace),
            'K' => Ok(Card::King),
            'Q' => Ok(Card::Queen),
            'J' => Ok(Card::Jack),
            'T' => Ok(Card::Ten),
            '9' => Ok(Card::Nine),
            '8' => Ok(Card::Eight),
            '7' => Ok(Card::Seven),
            '6' => Ok(Card::Six),
            '5' => Ok(Card::Five),
            '4' => Ok(Card::Four),
            '3' => Ok(Card::Three),
            '2' => Ok(Card::Two),
            _ => Err(eyre::eyre!("Invalid card: {}", c)),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Hand {
    cards: [Card; 5],
    bid: u64,
    hand_type: HandType,
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match self.hand_type.cmp(&other.hand_type) {
            std::cmp::Ordering::Equal => self.cards.cmp(&other.cards),
            o => o,
        }
    }
}

fn hand_type(cards: [Card; 5]) -> HandType {
    let mut counts: HashMap<Card, usize> = HashMap::new();
    for card in cards {
        counts.entry(card).and_modify(|c| *c += 1).or_insert(1);
    }
    let counts = counts.values().copied().collect::<Vec<_>>();
    if counts.contains(&5) {
        return HandType::FiveOfAKind;
    } else if counts.contains(&4) {
        return HandType::FourOfAKind;
    } else if counts.contains(&3) && counts.contains(&2) {
        return HandType::FullHouse;
    } else if counts.contains(&3) {
        return HandType::ThreeOfAKind;
    } else if counts.iter().filter(|c| **c == 2).count() == 2 {
        return HandType::TwoPair;
    } else if counts.contains(&2) {
        return HandType::OnePair;
    }
    HandType::HighCard
}
fn hand_type_joker(cards: [Card; 5]) -> HandType {
    let mut counts: HashMap<Card, usize> = HashMap::new();
    for card in cards {
        counts.entry(card).and_modify(|c| *c += 1).or_insert(1);
    }
    let joker_count = counts.remove(&Card::Joker).unwrap_or(0);
    let counts = counts.values().copied().collect::<Vec<_>>();
    match joker_count {
        5 => HandType::FiveOfAKind, // 5 jokers
        4 => HandType::FiveOfAKind, // 4 jokers + 1 card
        3 => {
            if counts.contains(&2) {
                HandType::FiveOfAKind// 3 jokers +1 pair
            } else {
                HandType::FourOfAKind// 3 jokers + high card
            }
        }
        2 => {
            if counts.contains(&3) {
                HandType::FiveOfAKind// 2 jokers + 3 of a kind
            } else if counts.contains(&2) {
                // this gets always mapped to four of a kind instead of full house since it is better
                return HandType::FourOfAKind; // 2 jokers + 1 pair + 1 card
            } else {
                return HandType::ThreeOfAKind; // 2 jokers + 3 high cards
            }
        }
        1 => {
            if counts.contains(&4) {
                HandType::FiveOfAKind// 1 joker + 4 of a kind
            } else if counts.contains(&3) {
                // a three of a kind always gets mapped to four of a kind instead of full house since it is better
                return HandType::FourOfAKind; // 1 joker + 3 of a kind + 1 card
            } else if counts.iter().filter(|c| **c == 2).count() == 2 {
                return HandType::FullHouse; // 1 joker + 2 pair
            } else if counts.contains(&2) {
                // 1 joker + 1 pair + 2 high cards
                // this gets always mapped to three of a kind instead of two pair since it is better
                return HandType::ThreeOfAKind;
            } else {
                return HandType::OnePair; // 1 joker + 4 high cards
            }
        }
        0 => hand_type(cards), // no jokers, just use the normal hand type
        _ => unreachable!(),
    }
}
impl Hand {
    fn new(cards: &str, bid: u64) -> Hand {
        let cards = cards
            .chars()
            .map(Card::try_from)
            .collect::<Result<Vec<_>>>()
            .unwrap()
            .try_into()
            .unwrap();

        let hand_type = hand_type(cards);

        Hand {
            cards,
            bid,
            hand_type,
        }
    }
}

fn parse_u64(input: &str) -> IResult<&str, u64> {
    map_res(digit1, str::parse::<u64>)(input)
}

fn parse_games(input: &str) -> IResult<&str, Vec<Hand>> {
    let (input, hands) = separated_list1(
        line_ending,
        map(
            separated_pair(take(5usize), space1, parse_u64),
            |(cards, bid)| Hand::new(cards, bid),
        ),
    )(input)?;

    Ok((input, hands))
}

fn parse(input: &str) -> Result<Vec<Hand>> {
    parse_games(input)
        .map_err(|e| eyre::eyre!("Failed to parse input: {}", e))
        .map(|x| x.1)
}

fn solve_stage1(input: &[Hand]) -> u64 {
    let mut hands = input.to_vec();
    hands.sort();
    hands
        .into_iter()
        .enumerate()
        .fold(0, |acc, (i, f)| acc + (i + 1) as u64 * f.bid)
}

fn solve_stage2(input: &[Hand]) -> u64 {
    let mut hands: Vec<_> = input
        .iter()
        .map(|h| {
            let mut replaced = h.cards;
            replaced.iter_mut().for_each(|x| {
                if *x == Card::Jack {
                    *x = Card::Joker;
                }
            });
            (hand_type_joker(replaced), replaced, h.bid)
        })
        .collect();
    hands.sort();
    hands
        .into_iter()
        .enumerate()
        .fold(0, |acc, (i, f)| acc + (i + 1) as u64 * f.2)
}

pub struct Day7Solver;
impl AdventOfCodeDay<'_> for Day7Solver {
    type ParsedInput = Vec<Hand>;

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

    const TEST_INPUT: &str = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";
    #[test]
    fn test_stage1() {
        let input = super::parse(TEST_INPUT).unwrap();
        assert_eq!(super::solve_stage1(&input), 6440);
    }
    #[test]
    fn test_stage2() {
        let input = super::parse(TEST_INPUT).unwrap();
        assert_eq!(super::solve_stage2(&input), 5905);
    }
}
