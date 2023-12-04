use aoc_traits::AdventOfCodeDay;
use day01::Day1Solver as Solver;
fn main() {
    let input = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/input.txt")).trim();
    let parsed_input = Solver::parse_input(input);
    let stage1_solution = Solver::solve_part1(&parsed_input);
    println!("Stage 1: {}", stage1_solution);
    let stage2_solution = Solver::solve_part2(&parsed_input);
    println!("Stage 2: {}", stage2_solution);
}
