use std::{
    collections::HashMap,
    fmt::Display,
    iter,
    str::FromStr,
    sync::{OnceLock, RwLock},
};

use aoc_traits::AdventOfCodeDay;
use rayon::iter::{IntoParallelRefMutIterator, ParallelIterator};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Spring {
    Working,
    Broken,
    Unknown,
}

impl Display for Spring {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Spring::Working => write!(f, "."),
            Spring::Broken => write!(f, "#"),
            Spring::Unknown => write!(f, "?"),
        }
    }
}

#[derive(Debug)]
pub struct Field {
    springs: Vec<Spring>,
    chunks: Vec<usize>,
}
impl FromStr for Field {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (springs, chunks) = s.split_once(' ').unwrap();
        let springs = springs
            .chars()
            .map(|c| match c {
                '.' => Spring::Working,
                '#' => Spring::Broken,
                '?' => Spring::Unknown,
                _ => unreachable!(),
            })
            .collect();
        let chunks = chunks.split(',').map(|x| x.parse().unwrap()).collect();
        Ok(Field { springs, chunks })
    }
}

fn memoizer() -> &'static RwLock<HashMap<(Vec<Spring>, Vec<usize>), u64>> {
    static ARRAY: OnceLock<RwLock<HashMap<(Vec<Spring>, Vec<usize>), u64>>> = OnceLock::new();
    ARRAY.get_or_init(|| RwLock::new(HashMap::new()))
}

fn num_valid_wrapper(springs: &mut [Spring], chunks: &[usize]) -> u64 {
    if springs.len() == 0 && chunks.len() == 0 {
        return 1;
    }
    if springs.len() == 0 && chunks.len() != 0 {
        return 0;
    }
    if let Some(val) = memoizer()
        .read()
        .unwrap()
        .get(&(springs.to_vec(), chunks.to_vec()))
        .copied()
    {
        return val;
    }
    let (s, c) = (springs.to_vec(), chunks.to_vec());
    let res = num_valid(springs, chunks);
    memoizer().write().unwrap().insert((s, c), res);
    res
}

fn num_valid(springs: &mut [Spring], chunks: &[usize]) -> u64 {
    if springs.len() == 0 && chunks.len() == 0 {
        return 1;
    }
    if springs.len() == 0 && chunks.len() != 0 {
        return 0;
    }
    // shortcut if len is exactly as needed to fit all chunks + seperator
    if springs.len() == chunks.iter().sum::<usize>() + chunks.len() - 1 {
        let mut i = 0;
        for chunk in chunks {
            for _ in 0..*chunk {
                if springs[i] == Spring::Working {
                    return 0;
                }
                i += 1;
            }
            if i < springs.len() && springs[i] == Spring::Broken {
                return 0;
            }
            i += 1;
        }

        return 1;
    }
    match springs[0] {
        // remove . prefixes
        Spring::Working => return num_valid_wrapper(&mut springs[1..], chunks),
        // # prefixes must be the size of the chunk
        Spring::Broken => {
            if chunks.len() == 0 {
                return 0;
            }
            if chunks[0] > springs.len() {
                return 0;
            }
            for i in 0..chunks[0] {
                // must be broken or unknown
                if springs[i] == Spring::Working {
                    return 0;
                }
            }
            let springs = &mut springs[chunks[0]..];
            let chunks = &chunks[1..];
            if springs.len() == 0 {
                if chunks.len() == 0 {
                    return 1;
                } else {
                    return 0;
                }
            }
            if chunks.len() == 0 {
                return if springs.iter().all(|x| *x != Spring::Broken) {
                    1
                } else {
                    0
                };
            }
            if springs[0] == Spring::Broken {
                return 0;
            }
            return num_valid_wrapper(&mut springs[1..], &chunks);
        }
        _ => (), // handle unknwns later
    };
    let springs_len = springs.len();
    if springs_len > 1 {
        match springs[springs.len() - 1] {
            // remove . suffixes
            Spring::Working => return num_valid_wrapper(&mut springs[..springs_len - 1], chunks),
            Spring::Broken => {
                if chunks.len() == 0 {
                    return 0;
                }
                if chunks[chunks.len() - 1] > springs.len() {
                    return 0;
                }
                for i in 0..chunks[chunks.len() - 1] {
                    // must be broken or unknown
                    if springs[springs_len - 1 - i] == Spring::Working {
                        return 0;
                    }
                }
                let springs = &mut springs[..springs_len - chunks[chunks.len() - 1]];
                let chunks = &chunks[..chunks.len() - 1];
                if springs.len() == 0 {
                    if chunks.len() == 0 {
                        return 1;
                    } else {
                        return 0;
                    }
                }
                if chunks.len() == 0 {
                    return if springs.iter().all(|x| *x != Spring::Broken) {
                        1
                    } else {
                        0
                    };
                }
                if springs.last().unwrap() == &Spring::Broken {
                    return 0;
                }
                let springs_len = springs.len();
                return num_valid_wrapper(&mut springs[..springs_len - 1], &chunks);
            }
            _ => (),
        }
    }

    // split unkowns into two branches, starting with the one that has the larger chunk at its side
    let mut new = springs.to_vec();
    if chunks.first().unwrap() <= chunks.last().unwrap() {
        assert!(springs[0] == Spring::Unknown);
        new[0] = Spring::Working;
        springs[0] = Spring::Broken;
    } else {
        assert!(springs[springs.len() - 1] == Spring::Unknown);
        new[springs.len() - 1] = Spring::Working;
        springs[springs.len() - 1] = Spring::Broken;
    }
    return num_valid_wrapper(&mut new, chunks) + num_valid_wrapper(springs, chunks);
}

impl Field {
    fn unfold(&self) -> Field {
        let springs = self
            .springs
            .iter()
            .chain(iter::once(&Spring::Unknown))
            .cycle()
            .take(self.springs.len() * 5 + 4)
            .copied()
            .collect();
        let chunks = self
            .chunks
            .iter()
            .cycle()
            .take(self.chunks.len() * 5)
            .copied()
            .collect();
        Field { springs, chunks }
    }
}

fn solve_stage1(input: &[Field]) -> u64 {
    input
        .iter()
        .map(|x| {
            let mut springs = x.springs.clone();
            num_valid(&mut springs, &x.chunks)
        })
        .sum()
}

fn solve_stage2(input: &[Field]) -> u64 {
    let mut unfolded = input.iter().map(|x| x.unfold()).collect::<Vec<_>>();
    unfolded
        .par_iter_mut()
        .map(|x| num_valid(&mut x.springs, &x.chunks))
        .sum()
}

pub struct Day12Solver;
impl AdventOfCodeDay<'_> for Day12Solver {
    type ParsedInput = Vec<Field>;

    type Part1Output = u64;

    type Part2Output = u64;

    fn solve_part1(input: &Self::ParsedInput) -> Self::Part1Output {
        solve_stage1(input)
    }

    fn solve_part2(input: &Self::ParsedInput) -> Self::Part2Output {
        solve_stage2(input)
    }

    fn parse_input(input: &str) -> Self::ParsedInput {
        input.lines().map(|x| x.parse().unwrap()).collect()
    }
}

#[cfg(test)]
mod tests {
    use aoc_traits::AdventOfCodeDay;

    use crate::Day12Solver;

    const TEST_INPUT: &str = "???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1";
    #[test]
    fn test_stage1() {
        let input = Day12Solver::parse_input(TEST_INPUT);
        assert_eq!(super::solve_stage1(&input), 21);
    }
    #[test]
    fn test_stage2() {
        let input = Day12Solver::parse_input(TEST_INPUT);
        assert_eq!(super::solve_stage2(&input), 525152);
    }
}
