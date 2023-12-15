use std::collections::VecDeque;

use aoc_traits::AdventOfCodeDay;

fn hash(input: &str) -> u8 {
    input
        .bytes()
        .fold(0, |acc, x| (acc.wrapping_add(x).wrapping_mul(17)))
}

pub struct Day15Solver;
impl<'a> AdventOfCodeDay<'a> for Day15Solver {
    type ParsedInput = &'a str;

    type Part1Output = u64;

    type Part2Output = u64;

    fn solve_part1(input: &Self::ParsedInput) -> Self::Part1Output {
        input.split(',').map(|x| hash(x) as u64).sum()
    }

    fn solve_part2(input: &Self::ParsedInput) -> Self::Part2Output {
        let mut boxes = vec![VecDeque::<(&str, u8)>::new(); 256];
        input.split(',').for_each(|x| {
            let op = x.find(|c| c == '=' || c == '-').unwrap();
            let label = &x[..op];
            let hash = hash(label);
            match x.as_bytes()[op] {
                b'=' => {
                    let num: u8 = x[op + 1..].parse().unwrap();
                    if let Some(i) = boxes[hash as usize].iter_mut().find(|(l, _)| l == &label) {
                        i.1 = num;
                    } else {
                        boxes[hash as usize].push_back((label, num));
                    }
                }
                b'-' => {
                    if let Some(i) = boxes[hash as usize].iter().position(|(l, _)| l == &label) {
                        boxes[hash as usize].remove(i);
                    }
                }
                _ => unreachable!(),
            }
        });
        boxes
            .into_iter()
            .enumerate()
            .map(|(i, x)| {
                x.into_iter()
                    .enumerate()
                    .map(|(j, (_, x))| x as u64 * (i + 1) as u64 * (j + 1) as u64)
                    .sum::<u64>()
            })
            .sum()
    }

    fn parse_input(input: &'a str) -> Self::ParsedInput {
        input
    }
}

#[cfg(test)]
mod tests {
    use super::Day15Solver;
    use aoc_traits::AdventOfCodeDay;

    const TEST_INPUT: &str = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";
    #[test]
    fn test_stage1() {
        let input = Day15Solver::parse_input(TEST_INPUT);
        assert_eq!(Day15Solver::solve_part1(&input), 1320);
    }
    #[test]
    fn test_stage2() {
        let input = Day15Solver::parse_input(TEST_INPUT);
        assert_eq!(Day15Solver::solve_part2(&input), 145);
    }
}
