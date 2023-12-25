use std::collections::HashMap;

use aoc_traits::AdventOfCodeDay;
use rand::{seq::IteratorRandom, thread_rng};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Connection<'a> {
    name: &'a str,
    connections: Vec<&'a str>,
}

impl Connection<'_> {}

fn parse<'a>(s: &'a str) -> Connection<'a> {
    let (name, conns) = s.split_once(": ").unwrap();
    Connection {
        name,
        connections: conns.split(" ").collect(),
    }
}

fn solve_stage1<'a>(input: &[Connection<'a>]) -> u64 {
    let mut graph_adj: HashMap<String, Vec<String>> = HashMap::new();

    for conn in input {
        graph_adj
            .entry(conn.name.to_owned())
            .or_default()
            .extend(conn.connections.iter().map(|&x| x.to_owned()));
        for c in &conn.connections {
            graph_adj
                .entry((*c).to_owned())
                .or_default()
                .push(conn.name.to_owned());
        }
    }
    loop {
        let graph = graph_adj.clone();
        let res = karger(graph);
        if res.values().next().unwrap().len() == 3 {
            // found the min cut of 3
            return res
                .keys()
                .map(|x| x.chars().filter(|&x| x == '-').count() as u64 + 1)
                .product();
        }
    }
}

fn karger(mut graph: HashMap<String, Vec<String>>) -> HashMap<String, Vec<String>> {
    let mut rng = thread_rng();
    while graph.len() > 2 {
        let a = graph.keys().choose(&mut rng).cloned().unwrap();
        let b = graph[&a].iter().choose(&mut rng).cloned().unwrap();
        // merge a & b into new node
        let new = format!("{}-{}", a, b);
        let mut new_edges = graph.remove(&a).unwrap();
        new_edges.extend(graph.remove(&b).unwrap());
        // remove the edges between a&b
        let new_edges = new_edges
            .into_iter()
            .filter(|x| *x != a && *x != b)
            .collect();
        graph.insert(new.clone(), new_edges);
        for (_, v) in graph.iter_mut() {
            for x in v.iter_mut() {
                if *x == a || *x == b {
                    *x = new.clone();
                }
            }
        }
    }
    graph
}

pub struct Day25Solver;
impl<'a> AdventOfCodeDay<'a> for Day25Solver {
    type ParsedInput = Vec<Connection<'a>>;

    type Part1Output = u64;

    type Part2Output = u64;

    fn solve_part1(input: &Self::ParsedInput) -> Self::Part1Output {
        solve_stage1(input)
    }

    fn solve_part2(_input: &Self::ParsedInput) -> Self::Part2Output {
        todo!()
    }

    fn parse_input(input: &'a str) -> Self::ParsedInput {
        input.lines().map(parse).collect()
    }
}

#[cfg(test)]
mod tests {
    use aoc_traits::AdventOfCodeDay;

    use crate::Day25Solver;

    const TEST_INPUT: &str = r#"jqt: rhn xhk nvd
rsh: frs pzl lsr
xhk: hfx
cmg: qnr nvd lhk bvb
rhn: xhk bvb hfx
bvb: xhk hfx
pzl: lsr hfx nvd
qnr: nvd
ntq: jqt hfx bvb xhk
nvd: lhk
lsr: lhk
rzs: qnr cmg lsr rsh
frs: qnr lhk lsr"#;
    #[test]
    fn test_stage1() {
        let input = Day25Solver::parse_input(TEST_INPUT);
        assert_eq!(super::solve_stage1(&input), 54);
    }
}
