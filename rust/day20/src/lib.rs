use std::{
    collections::{HashMap, VecDeque},
    vec,
};

use aoc_traits::AdventOfCodeDay;
use color_eyre::{
    eyre::{self},
    Result,
};
use nom::{
    bytes::complete::tag,
    character::complete::{alpha1, line_ending},
    combinator::map,
    multi::separated_list1,
    sequence::terminated,
    IResult,
};
use num_integer::Integer;

#[derive(Debug, Clone)]
pub struct State {
    gates: HashMap<u32, Gate>,
}

impl State {
    fn press_button(&self, to_search: (u32, u32, bool)) -> (State, (u64, u64), bool) {
        let mut new_state = self.clone();
        let (mut lows, mut highs) = (0, 0);
        let mut queue = VecDeque::new();
        let mut hit_search = false;
        queue.push_back((0, 0, false));
        while let Some((sender, target, pulse)) = queue.pop_front() {
            if pulse {
                highs += 1;
            } else {
                lows += 1;
            }
            if sender == to_search.0 && target == to_search.1 && pulse == to_search.2 {
                hit_search = true;
            }
            let gate = new_state.gates.get_mut(&target);
            match gate {
                Some(Gate {
                    gate_type: GateType::FlipFlop { state },
                    outputs,
                    ..
                }) => {
                    if pulse {
                        continue;
                    }
                    *state = !*state;
                    for output in outputs.iter() {
                        queue.push_back((target, *output, *state));
                    }
                }
                Some(Gate {
                    gate_type: GateType::Conjunction { input_states },
                    outputs,
                    ..
                }) => {
                    assert!(input_states.insert(sender, pulse).is_some()); // we don't get new signals
                    let new_pulse = input_states.values().all(|x| *x);
                    for output in outputs.iter() {
                        queue.push_back((target, *output, !new_pulse));
                    }
                }
                Some(Gate {
                    gate_type: GateType::Broadcaster,
                    outputs,
                    ..
                }) => {
                    for output in outputs.iter() {
                        queue.push_back((target, *output, pulse));
                    }
                }
                None => {
                    //println!("Hit noop-gate {}", target);
                }
            }
        }
        (new_state, (lows, highs), hit_search)
    }
}

#[derive(Debug, Clone)]
enum GateType {
    FlipFlop { state: bool },
    Conjunction { input_states: HashMap<u32, bool> },
    Broadcaster,
}

#[derive(Debug, Clone)]
struct Gate {
    id: u32,
    gate_type: GateType,
    outputs: Vec<u32>,
    inputs: Vec<u32>,
}

fn name_to_id(name: &str) -> u32 {
    if name == "broadcaster" {
        return 0;
    }
    name.chars()
        .fold(0, |acc, c| acc * 26 + (c as u32 - 'A' as u32))
}

fn parse_gate(input: &str) -> IResult<&str, Gate> {
    let (gate_type, offset) = match input.as_bytes()[0] {
        b'%' => (GateType::FlipFlop { state: false }, 1),
        b'&' => (
            GateType::Conjunction {
                input_states: HashMap::new(),
            },
            1,
        ),
        b'b' => (GateType::Broadcaster, 0),
        _ => unreachable!(),
    };
    let input = &input[offset..];
    let (input, id) = terminated(map(alpha1, name_to_id), tag(" -> "))(input)?;
    let (input, outputs) = separated_list1(tag(", "), map(alpha1, name_to_id))(input)?;
    let gate = Gate {
        id,
        gate_type,
        outputs,
        inputs: vec![],
    };
    Ok((input, gate))
}

fn find_hits(input: &State, to_search: (u32, u32, bool)) -> Option<usize> {
    let mut game_state = input.clone();
    let mut res = vec![];
    for i in 1..100000 {
        let (new_state, _, hit) = game_state.press_button(to_search);
        game_state = new_state;
        if hit {
            res.push(i);
            if res.len() > 3
                && res[res.len() - 1] - res[res.len() - 2]
                    == res[res.len() - 2] - res[res.len() - 3]
                && res[res.len() - 2] - res[res.len() - 3]
                    == res[res.len() - 3] - res[res.len() - 4]
            {
                return Some(res[res.len() - 1] - res[res.len() - 2]);
            }
        }
    }

    None
}

fn parse_game(input: &str) -> IResult<&str, State> {
    let (input, gates) = separated_list1(line_ending, parse_gate)(input)?;
    let mut gate_map: HashMap<u32, Gate> = gates.clone().into_iter().map(|x| (x.id, x)).collect();

    for gate in gates {
        for output in gate.outputs {
            if let Some(g) = gate_map.get_mut(&output) {
                g.inputs.push(gate.id);
                if let GateType::Conjunction {
                    ref mut input_states,
                } = g.gate_type
                {
                    input_states.insert(gate.id, false);
                }
            }
        }
    }

    Ok((input, State { gates: gate_map }))
}

fn parse(input: &str) -> Result<State> {
    parse_game(input)
        .map_err(|e| eyre::eyre!("Failed to parse input: {}", e))
        .map(|x| x.1)
}

fn solve_stage1(input: &State) -> u64 {
    let mut lows = 0;
    let mut highs = 0;
    let mut game_state = input.clone();

    for _ in 0..1000 {
        let (new_state, (new_lows, new_highs), _) = game_state.press_button((0, 0, false));
        game_state = new_state;
        lows += new_lows;
        highs += new_highs;
    }

    lows * highs
}

fn solve_stage2(input: &State) -> u64 {
    let target = name_to_id("rx");
    //dbg!(&input.gates);

    for gate in input.gates.values() {
        // find the single gate that outputs to rx
        if gate.outputs.contains(&target) {
            // this is just a conjunction gate with conjunction input
            assert!(matches!(gate.gate_type, GateType::Conjunction { .. }));

            // we get the period of each input
            let periods: Vec<usize> = gate
                .inputs
                .iter()
                .map(|x| find_hits(input, (*x, gate.id, true)).unwrap())
                .collect();

            // and get the LCM
            return periods.into_iter().fold(1, |acc, x| acc.lcm(&x)) as u64;
        }
    }

    0
}

pub struct Day20Solver;
impl AdventOfCodeDay<'_> for Day20Solver {
    type ParsedInput = State;

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

    const TEST_INPUT: &str = r#"broadcaster -> a, b, c
%a -> b
%b -> c
%c -> inv
&inv -> a"#;
    const TEST_INPUT2: &str = r#"broadcaster -> a
%a -> inv, con
&inv -> b
%b -> con
&con -> output"#;
    #[test]
    fn test_stage1() {
        let input = super::parse(TEST_INPUT).unwrap();
        assert_eq!(super::solve_stage1(&input), 32000000);
    }
    #[test]
    fn test_stage1_2() {
        let input = super::parse(TEST_INPUT2).unwrap();
        assert_eq!(super::solve_stage1(&input), 11687500);
    }
}
