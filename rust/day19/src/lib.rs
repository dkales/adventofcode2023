use std::{collections::HashMap, str::FromStr};

use aoc_traits::AdventOfCodeDay;
use nom::{
    bytes::complete::tag,
    character::complete::{alpha1, digit1},
    combinator::map_res,
    multi::separated_list1,
    sequence::delimited,
    IResult,
};

#[derive(Debug)]
struct Part {
    xmas: [u64; 4],
}

impl Part {
    fn apply(&self, rules: &[Rule]) -> String {
        for rule in rules {
            match rule.ty {
                RuleType::Larger(idx, num) => {
                    if self.xmas[idx] > num {
                        return rule.target.clone();
                    }
                }
                RuleType::Smaller(idx, num) => {
                    if self.xmas[idx] < num {
                        return rule.target.clone();
                    }
                }
                RuleType::Goto => {
                    return rule.target.clone();
                }
            }
        }
        unreachable!()
    }
}

#[derive(Debug)]
struct PartCollection {
    xmas: [std::ops::Range<u64>; 4],
}

impl PartCollection {
    fn apply(&self, rules: &[Rule]) -> HashMap<String, Vec<PartCollection>> {
        let mut result: HashMap<String, Vec<PartCollection>> = HashMap::new();
        let mut xmas = self.xmas.clone();
        for rule in rules {
            match rule.ty {
                RuleType::Larger(idx, num) => {
                    let smaller = xmas[idx].start..num + 1;
                    let larger = num + 1..xmas[idx].end;
                    let mut xmas_larger = xmas.clone();
                    xmas_larger[idx] = larger;
                    xmas[idx] = smaller;
                    result
                        .entry(rule.target.clone())
                        .or_default()
                        .push(PartCollection { xmas: xmas_larger });
                }
                RuleType::Smaller(idx, num) => {
                    let smaller = xmas[idx].start..num;
                    let larger = num..xmas[idx].end;
                    let mut xmas_smaller = xmas.clone();
                    xmas_smaller[idx] = smaller;
                    xmas[idx] = larger;
                    result
                        .entry(rule.target.clone())
                        .or_default()
                        .push(PartCollection { xmas: xmas_smaller });
                }
                RuleType::Goto => {
                    result
                        .entry(rule.target.clone())
                        .or_default()
                        .push(PartCollection { xmas: xmas.clone() });
                }
            }
        }
        result
    }

    fn variants(&self) -> u64 {
        self.xmas.iter().map(|x| x.end - x.start).product()
    }
}

#[derive(Debug)]
enum RuleType {
    Larger(usize, u64),
    Smaller(usize, u64),
    Goto,
}

#[derive(Debug)]
struct Rule {
    ty: RuleType,
    target: String,
}

#[derive(Debug)]
pub struct Game {
    rules: HashMap<String, Vec<Rule>>,
    parts: Vec<Part>,
}

fn parse_u64(input: &str) -> IResult<&str, u64> {
    map_res(digit1, str::parse::<u64>)(input)
}
fn parse_rule(input: &str) -> IResult<&str, Rule> {
    if !input.contains(':') {
        let (input, name) = alpha1(input)?;
        return Ok((
            input,
            Rule {
                ty: RuleType::Goto,
                target: name.to_owned(),
            },
        ));
    }
    let idx = match input.as_bytes()[0] {
        b'x' => 0,
        b'm' => 1,
        b'a' => 2,
        b's' => 3,
        _ => unreachable!(),
    };
    let m = input.as_bytes()[1];
    let (input, num) = parse_u64(&input[2..])?;
    let ty = match m {
        b'<' => RuleType::Smaller(idx, num),
        b'>' => RuleType::Larger(idx, num),
        _ => unreachable!(),
    };
    let (input, _) = tag(":")(input)?;
    let (input, target) = alpha1(input)?;
    Ok((
        input,
        Rule {
            ty,
            target: target.to_owned(),
        },
    ))
}
fn parse_rules(input: &str) -> IResult<&str, (String, Vec<Rule>)> {
    let (input, name) = alpha1(input)?;
    let (input, rules) =
        delimited(tag("{"), separated_list1(tag(","), parse_rule), tag("}"))(input)?;
    Ok((input, (name.to_owned(), rules)))
}
fn parse_part(input: &str) -> IResult<&str, Part> {
    let (input, _) = tag("{x=")(input)?;
    let (input, x) = parse_u64(input)?;
    let (input, _) = tag(",m=")(input)?;
    let (input, m) = parse_u64(input)?;
    let (input, _) = tag(",a=")(input)?;
    let (input, a) = parse_u64(input)?;
    let (input, _) = tag(",s=")(input)?;
    let (input, s) = parse_u64(input)?;
    Ok((input, Part { xmas: [x, m, a, s] }))
}

impl FromStr for Game {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (rules, parts) = s.split_once("\n\n").unwrap();
        let rules = rules
            .lines()
            .map(|line| parse_rules(line).unwrap().1)
            .collect::<HashMap<_, _>>();
        let parts = parts
            .lines()
            .map(|line| parse_part(line).unwrap().1)
            .collect();

        Ok(Game { rules, parts })
    }
}

pub struct Day19Solver;
impl AdventOfCodeDay<'_> for Day19Solver {
    type ParsedInput = Game;

    type Part1Output = u64;

    type Part2Output = u64;

    fn solve_part1(input: &Self::ParsedInput) -> Self::Part1Output {
        let Game { rules, parts } = input;
        parts
            .iter()
            .filter(|part| {
                let mut current_rule = String::from("in");
                loop {
                    current_rule = part.apply(&rules[&current_rule]);
                    if current_rule == "R" {
                        return false;
                    } else if current_rule == "A" {
                        return true;
                    }
                }
            })
            .map(|part| part.xmas.iter().sum::<u64>())
            .sum()
    }

    fn solve_part2(input: &Self::ParsedInput) -> Self::Part2Output {
        let parts = PartCollection {
            xmas: [1..4001, 1..4001, 1..4001, 1..4001],
        };
        let mut mappings = HashMap::new();
        mappings.insert("in".to_owned(), vec![parts]);
        while let Some(x) = mappings.keys().find(|&x| x != "R" && x != "A").cloned() {
            let cur_maps = mappings.remove(&x).unwrap();
            for cur_map in cur_maps {
                let new_mappings = cur_map.apply(&input.rules[&x]);
                for (k, v) in new_mappings {
                    mappings.entry(k).or_default().extend(v);
                }
            }
        }
        mappings["A"].iter().map(|x| x.variants()).sum()
    }

    fn parse_input(input: &str) -> Self::ParsedInput {
        input.parse().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use aoc_traits::AdventOfCodeDay;

    use crate::Day19Solver;

    const TEST_INPUT: &str = r#"px{a<2006:qkq,m>2090:A,rfg}
pv{a>1716:R,A}
lnx{m>1548:A,A}
rfg{s<537:gd,x>2440:R,A}
qs{s>3448:A,lnx}
qkq{x<1416:A,crn}
crn{x>2662:A,R}
in{s<1351:px,qqz}
qqz{s>2770:qs,m<1801:hdj,R}
gd{a>3333:R,R}
hdj{m>838:A,pv}

{x=787,m=2655,a=1222,s=2876}
{x=1679,m=44,a=2067,s=496}
{x=2036,m=264,a=79,s=2244}
{x=2461,m=1339,a=466,s=291}
{x=2127,m=1623,a=2188,s=1013}"#;
    #[test]
    fn test_stage1() {
        let input = Day19Solver::parse_input(TEST_INPUT);
        assert_eq!(Day19Solver::solve_part1(&input), 19114);
    }
    #[test]
    fn test_stage2() {
        let input = Day19Solver::parse_input(TEST_INPUT);
        assert_eq!(Day19Solver::solve_part2(&input), 167409079868000);
    }
}
