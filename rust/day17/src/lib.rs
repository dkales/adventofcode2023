use std::{
    cell::RefCell,
    collections::{BinaryHeap, HashMap, HashSet},
    fmt::Display,
    rc::Rc,
    str::FromStr,
};

use aoc_traits::AdventOfCodeDay;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Grid {
    lines: Vec<Vec<usize>>,
    dims: (usize, usize),
}

impl Display for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for line in &self.lines {
            for c in line {
                write!(f, "{c}\t")?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Node {
    inner: Rc<RefCell<InnerNode>>,
}

impl std::hash::Hash for Node {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        std::ptr::hash(&*self.inner, state)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Container {
    prio: usize,
    node: Node,
}
impl Ord for Container {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.prio.cmp(&other.prio).reverse()
    }
}
impl PartialOrd for Container {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(&other))
    }
}

impl Node {
    fn new(pos: (usize, usize), num_same_dir: (u8, u8, u8, u8)) -> Self {
        Node {
            inner: Rc::new(RefCell::new(InnerNode {
                pos,
                num_same_dir,
                neighbors: HashSet::new(),
            })),
        }
    }

    fn pos(&self) -> (usize, usize) {
        self.inner.borrow().pos
    }
    fn num_same_dir(&self) -> (u8, u8, u8, u8) {
        self.inner.borrow().num_same_dir
    }

    fn gen_neighbors(&mut self, graph: &mut Graph, grid: &Grid) {
        if !self.inner.borrow().neighbors.is_empty() {
            return;
        }
        let pos = self.pos();
        let num_same_dir = self.num_same_dir();
        match num_same_dir {
            (x, 0, 0, 0) if x >= 1 => {
                // going up
                if pos.0 > 0 && x < 3 {
                    if let Some(node) = graph.get(&(pos.0 - 1, pos.1, x + 1, 0, 0, 0)) {
                        self.inner.borrow_mut().neighbors.insert(node.clone());
                    } else {
                        let node = Node::new((pos.0 - 1, pos.1), (x + 1, 0, 0, 0));
                        self.inner.borrow_mut().neighbors.insert(node.clone());
                        graph.insert((pos.0 - 1, pos.1, x + 1, 0, 0, 0), node);
                    }
                }
                // going left
                if pos.1 > 0 {
                    if let Some(node) = graph.get(&(pos.0, pos.1 - 1, 0, 0, 1, 0)) {
                        self.inner.borrow_mut().neighbors.insert(node.clone());
                    } else {
                        let node = Node::new((pos.0, pos.1 - 1), (0, 0, 1, 0));
                        self.inner.borrow_mut().neighbors.insert(node.clone());
                        graph.insert((pos.0, pos.1 - 1, 0, 0, 1, 0), node);
                    }
                }
                // going right
                if pos.1 < grid.dims.1 - 1 {
                    if let Some(node) = graph.get(&(pos.0, pos.1 + 1, 0, 0, 0, 1)) {
                        self.inner.borrow_mut().neighbors.insert(node.clone());
                    } else {
                        let node = Node::new((pos.0, pos.1 + 1), (0, 0, 0, 1));
                        self.inner.borrow_mut().neighbors.insert(node.clone());
                        graph.insert((pos.0, pos.1 + 1, 0, 0, 0, 1), node);
                    }
                }
            }
            (0, x, 0, 0) if x >= 1 => {
                // going down
                if pos.0 < grid.dims.0 - 1 && x < 3 {
                    if let Some(node) = graph.get(&(pos.0 + 1, pos.1, 0, x + 1, 0, 0)) {
                        self.inner.borrow_mut().neighbors.insert(node.clone());
                    } else {
                        let node = Node::new((pos.0 + 1, pos.1), (0, x + 1, 0, 0));
                        self.inner.borrow_mut().neighbors.insert(node.clone());
                        graph.insert((pos.0 + 1, pos.1, 0, x + 1, 0, 0), node);
                    }
                }
                // going left
                if pos.1 > 0 {
                    if let Some(node) = graph.get(&(pos.0, pos.1 - 1, 0, 0, 1, 0)) {
                        self.inner.borrow_mut().neighbors.insert(node.clone());
                    } else {
                        let node = Node::new((pos.0, pos.1 - 1), (0, 0, 1, 0));
                        self.inner.borrow_mut().neighbors.insert(node.clone());
                        graph.insert((pos.0, pos.1 - 1, 0, 0, 1, 0), node);
                    }
                }
                // going right
                if pos.1 < grid.dims.1 - 1 {
                    if let Some(node) = graph.get(&(pos.0, pos.1 + 1, 0, 0, 0, 1)) {
                        self.inner.borrow_mut().neighbors.insert(node.clone());
                    } else {
                        let node = Node::new((pos.0, pos.1 + 1), (0, 0, 0, 1));
                        self.inner.borrow_mut().neighbors.insert(node.clone());
                        graph.insert((pos.0, pos.1 + 1, 0, 0, 0, 1), node);
                    }
                }
            }
            (0, 0, x, 0) if x >= 1 => {
                // going left
                if pos.1 > 0 && x < 3 {
                    if let Some(node) = graph.get(&(pos.0, pos.1 - 1, 0, 0, x + 1, 0)) {
                        self.inner.borrow_mut().neighbors.insert(node.clone());
                    } else {
                        let node = Node::new((pos.0, pos.1 - 1), (0, 0, x + 1, 0));
                        self.inner.borrow_mut().neighbors.insert(node.clone());
                        graph.insert((pos.0, pos.1 - 1, 0, 0, x + 1, 0), node);
                    }
                }
                // going up
                if pos.0 > 0 {
                    if let Some(node) = graph.get(&(pos.0 - 1, pos.1, 1, 0, 0, 0)) {
                        self.inner.borrow_mut().neighbors.insert(node.clone());
                    } else {
                        let node = Node::new((pos.0 - 1, pos.1), (1, 0, 0, 0));
                        self.inner.borrow_mut().neighbors.insert(node.clone());
                        graph.insert((pos.0 - 1, pos.1, 1, 0, 0, 0), node);
                    }
                }
                // going down
                if pos.0 < grid.dims.0 - 1 {
                    if let Some(node) = graph.get(&(pos.0 + 1, pos.1, 0, 1, 0, 0)) {
                        self.inner.borrow_mut().neighbors.insert(node.clone());
                    } else {
                        let node = Node::new((pos.0 + 1, pos.1), (0, 1, 0, 0));
                        self.inner.borrow_mut().neighbors.insert(node.clone());
                        graph.insert((pos.0 + 1, pos.1, 0, 1, 0, 0), node);
                    }
                }
            }
            (0, 0, 0, x) if x >= 1 => {
                // going right
                if pos.1 < grid.dims.1 - 1 && x < 3 {
                    if let Some(node) = graph.get(&(pos.0, pos.1 + 1, 0, 0, 0, x + 1)) {
                        self.inner.borrow_mut().neighbors.insert(node.clone());
                    } else {
                        let node = Node::new((pos.0, pos.1 + 1), (0, 0, 0, x + 1));
                        self.inner.borrow_mut().neighbors.insert(node.clone());
                        graph.insert((pos.0, pos.1 + 1, 0, 0, 0, x + 1), node);
                    }
                }
                // going up
                if pos.0 > 0 {
                    if let Some(node) = graph.get(&(pos.0 - 1, pos.1, 1, 0, 0, 0)) {
                        self.inner.borrow_mut().neighbors.insert(node.clone());
                    } else {
                        let node = Node::new((pos.0 - 1, pos.1), (1, 0, 0, 0));
                        self.inner.borrow_mut().neighbors.insert(node.clone());
                        graph.insert((pos.0 - 1, pos.1, 1, 0, 0, 0), node);
                    }
                }
                // going down
                if pos.0 < grid.dims.0 - 1 {
                    if let Some(node) = graph.get(&(pos.0 + 1, pos.1, 0, 1, 0, 0)) {
                        self.inner.borrow_mut().neighbors.insert(node.clone());
                    } else {
                        let node = Node::new((pos.0 + 1, pos.1), (0, 1, 0, 0));
                        self.inner.borrow_mut().neighbors.insert(node.clone());
                        graph.insert((pos.0 + 1, pos.1, 0, 1, 0, 0), node);
                    }
                }
            }
            (0, 0, 0, 0) => {
                // the very first node
                // go down
                let node = Node::new((pos.0 + 1, pos.1), (0, 1, 0, 0));
                self.inner.borrow_mut().neighbors.insert(node.clone());
                graph.insert((pos.0 + 1, pos.1, 0, 1, 0, 0), node);
                // go right
                let node = Node::new((pos.0, pos.1 + 1), (0, 0, 0, 1));
                self.inner.borrow_mut().neighbors.insert(node.clone());
                graph.insert((pos.0, pos.1 + 1, 0, 0, 0, 1), node);
            }
            rest => {
                dbg!(rest);
                panic!("unreachable?")
            }
        }
    }

    fn gen_neighbors2(&mut self, graph: &mut Graph, grid: &Grid) {
        if !self.inner.borrow().neighbors.is_empty() {
            return;
        }
        let pos = self.pos();
        let num_same_dir = self.num_same_dir();
        match num_same_dir {
            (x, 0, 0, 0) if x >= 1 && x < 4 => {
                // going up
                if pos.0 > 0 {
                    if let Some(node) = graph.get(&(pos.0 - 1, pos.1, x + 1, 0, 0, 0)) {
                        self.inner.borrow_mut().neighbors.insert(node.clone());
                    } else {
                        let node = Node::new((pos.0 - 1, pos.1), (x + 1, 0, 0, 0));
                        self.inner.borrow_mut().neighbors.insert(node.clone());
                        graph.insert((pos.0 - 1, pos.1, x + 1, 0, 0, 0), node);
                    }
                }
            }
            (x, 0, 0, 0) if x >= 4 => {
                // going up
                if pos.0 > 0 && x < 10 {
                    if let Some(node) = graph.get(&(pos.0 - 1, pos.1, x + 1, 0, 0, 0)) {
                        self.inner.borrow_mut().neighbors.insert(node.clone());
                    } else {
                        let node = Node::new((pos.0 - 1, pos.1), (x + 1, 0, 0, 0));
                        self.inner.borrow_mut().neighbors.insert(node.clone());
                        graph.insert((pos.0 - 1, pos.1, x + 1, 0, 0, 0), node);
                    }
                }
                // going left
                if pos.1 > 0 {
                    if let Some(node) = graph.get(&(pos.0, pos.1 - 1, 0, 0, 1, 0)) {
                        self.inner.borrow_mut().neighbors.insert(node.clone());
                    } else {
                        let node = Node::new((pos.0, pos.1 - 1), (0, 0, 1, 0));
                        self.inner.borrow_mut().neighbors.insert(node.clone());
                        graph.insert((pos.0, pos.1 - 1, 0, 0, 1, 0), node);
                    }
                }
                // going right
                if pos.1 < grid.dims.1 - 1 {
                    if let Some(node) = graph.get(&(pos.0, pos.1 + 1, 0, 0, 0, 1)) {
                        self.inner.borrow_mut().neighbors.insert(node.clone());
                    } else {
                        let node = Node::new((pos.0, pos.1 + 1), (0, 0, 0, 1));
                        self.inner.borrow_mut().neighbors.insert(node.clone());
                        graph.insert((pos.0, pos.1 + 1, 0, 0, 0, 1), node);
                    }
                }
            }
            (0, x, 0, 0) if x >= 1 && x < 4 => {
                // going down
                if pos.0 < grid.dims.0 - 1 {
                    if let Some(node) = graph.get(&(pos.0 + 1, pos.1, 0, x + 1, 0, 0)) {
                        self.inner.borrow_mut().neighbors.insert(node.clone());
                    } else {
                        let node = Node::new((pos.0 + 1, pos.1), (0, x + 1, 0, 0));
                        self.inner.borrow_mut().neighbors.insert(node.clone());
                        graph.insert((pos.0 + 1, pos.1, 0, x + 1, 0, 0), node);
                    }
                }
            }
            (0, x, 0, 0) if x >= 4 => {
                // going down
                if pos.0 < grid.dims.0 - 1 && x < 10 {
                    if let Some(node) = graph.get(&(pos.0 + 1, pos.1, 0, x + 1, 0, 0)) {
                        self.inner.borrow_mut().neighbors.insert(node.clone());
                    } else {
                        let node = Node::new((pos.0 + 1, pos.1), (0, x + 1, 0, 0));
                        self.inner.borrow_mut().neighbors.insert(node.clone());
                        graph.insert((pos.0 + 1, pos.1, 0, x + 1, 0, 0), node);
                    }
                }
                // going left
                if pos.1 > 0 {
                    if let Some(node) = graph.get(&(pos.0, pos.1 - 1, 0, 0, 1, 0)) {
                        self.inner.borrow_mut().neighbors.insert(node.clone());
                    } else {
                        let node = Node::new((pos.0, pos.1 - 1), (0, 0, 1, 0));
                        self.inner.borrow_mut().neighbors.insert(node.clone());
                        graph.insert((pos.0, pos.1 - 1, 0, 0, 1, 0), node);
                    }
                }
                // going right
                if pos.1 < grid.dims.1 - 1 {
                    if let Some(node) = graph.get(&(pos.0, pos.1 + 1, 0, 0, 0, 1)) {
                        self.inner.borrow_mut().neighbors.insert(node.clone());
                    } else {
                        let node = Node::new((pos.0, pos.1 + 1), (0, 0, 0, 1));
                        self.inner.borrow_mut().neighbors.insert(node.clone());
                        graph.insert((pos.0, pos.1 + 1, 0, 0, 0, 1), node);
                    }
                }
            }
            (0, 0, x, 0) if x >= 1 && x < 4 => {
                // going left
                if pos.1 > 0 {
                    if let Some(node) = graph.get(&(pos.0, pos.1 - 1, 0, 0, x + 1, 0)) {
                        self.inner.borrow_mut().neighbors.insert(node.clone());
                    } else {
                        let node = Node::new((pos.0, pos.1 - 1), (0, 0, x + 1, 0));
                        self.inner.borrow_mut().neighbors.insert(node.clone());
                        graph.insert((pos.0, pos.1 - 1, 0, 0, x + 1, 0), node);
                    }
                }
            }
            (0, 0, x, 0) if x >= 4 => {
                // going left
                if pos.1 > 0 && x < 10 {
                    if let Some(node) = graph.get(&(pos.0, pos.1 - 1, 0, 0, x + 1, 0)) {
                        self.inner.borrow_mut().neighbors.insert(node.clone());
                    } else {
                        let node = Node::new((pos.0, pos.1 - 1), (0, 0, x + 1, 0));
                        self.inner.borrow_mut().neighbors.insert(node.clone());
                        graph.insert((pos.0, pos.1 - 1, 0, 0, x + 1, 0), node);
                    }
                }
                // going up
                if pos.0 > 0 {
                    if let Some(node) = graph.get(&(pos.0 - 1, pos.1, 1, 0, 0, 0)) {
                        self.inner.borrow_mut().neighbors.insert(node.clone());
                    } else {
                        let node = Node::new((pos.0 - 1, pos.1), (1, 0, 0, 0));
                        self.inner.borrow_mut().neighbors.insert(node.clone());
                        graph.insert((pos.0 - 1, pos.1, 1, 0, 0, 0), node);
                    }
                }
                // going down
                if pos.0 < grid.dims.0 - 1 {
                    if let Some(node) = graph.get(&(pos.0 + 1, pos.1, 0, 1, 0, 0)) {
                        self.inner.borrow_mut().neighbors.insert(node.clone());
                    } else {
                        let node = Node::new((pos.0 + 1, pos.1), (0, 1, 0, 0));
                        self.inner.borrow_mut().neighbors.insert(node.clone());
                        graph.insert((pos.0 + 1, pos.1, 0, 1, 0, 0), node);
                    }
                }
            }
            (0, 0, 0, x) if x >= 1 && x < 4 => {
                // going right
                if pos.1 < grid.dims.1 - 1 {
                    if let Some(node) = graph.get(&(pos.0, pos.1 + 1, 0, 0, 0, x + 1)) {
                        self.inner.borrow_mut().neighbors.insert(node.clone());
                    } else {
                        let node = Node::new((pos.0, pos.1 + 1), (0, 0, 0, x + 1));
                        self.inner.borrow_mut().neighbors.insert(node.clone());
                        graph.insert((pos.0, pos.1 + 1, 0, 0, 0, x + 1), node);
                    }
                }
            }
            (0, 0, 0, x) if x >= 4 => {
                // going right
                if pos.1 < grid.dims.1 - 1 && x < 10 {
                    if let Some(node) = graph.get(&(pos.0, pos.1 + 1, 0, 0, 0, x + 1)) {
                        self.inner.borrow_mut().neighbors.insert(node.clone());
                    } else {
                        let node = Node::new((pos.0, pos.1 + 1), (0, 0, 0, x + 1));
                        self.inner.borrow_mut().neighbors.insert(node.clone());
                        graph.insert((pos.0, pos.1 + 1, 0, 0, 0, x + 1), node);
                    }
                }
                // going up
                if pos.0 > 0 {
                    if let Some(node) = graph.get(&(pos.0 - 1, pos.1, 1, 0, 0, 0)) {
                        self.inner.borrow_mut().neighbors.insert(node.clone());
                    } else {
                        let node = Node::new((pos.0 - 1, pos.1), (1, 0, 0, 0));
                        self.inner.borrow_mut().neighbors.insert(node.clone());
                        graph.insert((pos.0 - 1, pos.1, 1, 0, 0, 0), node);
                    }
                }
                // going down
                if pos.0 < grid.dims.0 - 1 {
                    if let Some(node) = graph.get(&(pos.0 + 1, pos.1, 0, 1, 0, 0)) {
                        self.inner.borrow_mut().neighbors.insert(node.clone());
                    } else {
                        let node = Node::new((pos.0 + 1, pos.1), (0, 1, 0, 0));
                        self.inner.borrow_mut().neighbors.insert(node.clone());
                        graph.insert((pos.0 + 1, pos.1, 0, 1, 0, 0), node);
                    }
                }
            }
            (0, 0, 0, 0) => {
                // the very first node
                // go down
                let node = Node::new((pos.0 + 1, pos.1), (0, 1, 0, 0));
                self.inner.borrow_mut().neighbors.insert(node.clone());
                graph.insert((pos.0 + 1, pos.1, 0, 1, 0, 0), node);
                // go right
                let node = Node::new((pos.0, pos.1 + 1), (0, 0, 0, 1));
                self.inner.borrow_mut().neighbors.insert(node.clone());
                graph.insert((pos.0, pos.1 + 1, 0, 0, 0, 1), node);
            }
            rest => {
                dbg!(rest);
                panic!("unreachable?")
            }
        }
    }
}

type Graph = HashMap<(usize, usize, u8, u8, u8, u8), Node>;

#[derive(Debug, Clone, PartialEq, Eq)]
struct InnerNode {
    pos: (usize, usize),
    // up, down, left, right
    num_same_dir: (u8, u8, u8, u8),
    neighbors: HashSet<Node>,
}

impl Grid {
    fn dijkstra(&self) -> u64 {
        let current = Node::new((0, 0), (0, 0, 0, 0));
        let mut graph: Graph = HashMap::new();
        let mut dist = HashMap::<Node, usize>::new();

        let mut queue: BinaryHeap<Container> = BinaryHeap::new();
        queue.push(Container {
            prio: 0,
            node: current.clone(),
        });
        dist.insert(current, 0);
        while let Some(Container {
            prio: p,
            node: mut n,
        }) = queue.pop()
        {
            if let Some(d) = dist.get(&n) {
                if *d != p {
                    continue;
                }
            }
            n.gen_neighbors(&mut graph, self);
            let inner = n.inner.borrow();
            for neighbor in inner.neighbors.iter() {
                {
                    let pos = neighbor.pos();
                    let new_distance = p + self.lines[pos.0][pos.1];
                    let d = dist.get(neighbor).copied().unwrap_or(usize::MAX);
                    if new_distance < d {
                        dist.insert(neighbor.clone(), new_distance);
                        queue.push(Container {
                            prio: new_distance,
                            node: neighbor.clone(),
                        });
                    }
                }
            }
        }

        dist.into_iter()
            .filter(|(n, _)| n.pos() == (self.dims.0 - 1, self.dims.1 - 1))
            .map(|(_, d)| d)
            .min()
            .unwrap() as u64
    }
    fn dijkstra2(&self) -> u64 {
        let current = Node::new((0, 0), (0, 0, 0, 0));
        let mut graph: Graph = HashMap::new();
        let mut dist = HashMap::<Node, usize>::new();

        let mut queue: BinaryHeap<Container> = BinaryHeap::new();
        queue.push(Container {
            prio: 0,
            node: current.clone(),
        });
        dist.insert(current, 0);
        while let Some(Container {
            prio: p,
            node: mut n,
        }) = queue.pop()
        {
            if let Some(d) = dist.get(&n) {
                if *d != p {
                    continue;
                }
            }
            n.gen_neighbors2(&mut graph, self);
            let inner = n.inner.borrow();
            for neighbor in inner.neighbors.iter() {
                {
                    let pos = neighbor.pos();
                    let new_distance = p + self.lines[pos.0][pos.1];
                    let d = dist.get(neighbor).copied().unwrap_or(usize::MAX);
                    if new_distance < d {
                        dist.insert(neighbor.clone(), new_distance);
                        queue.push(Container {
                            prio: new_distance,
                            node: neighbor.clone(),
                        });
                    }
                }
            }
        }

        dist.into_iter()
            .filter(|(n, _)| {
                n.pos() == (self.dims.0 - 1, self.dims.1 - 1)
                    && (n.num_same_dir().0 > 3
                        || n.num_same_dir().1 > 3
                        || n.num_same_dir().2 > 3
                        || n.num_same_dir().3 > 3)
            })
            .map(|(_, d)| d)
            .min()
            .unwrap() as u64
    }
}

impl FromStr for Grid {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lines: Vec<Vec<_>> = s
            .lines()
            .map(|x| x.chars().map(|x| x as usize - '0' as usize).collect())
            .collect();
        Ok(Grid {
            dims: (lines.len(), lines[0].len()),
            lines,
        })
    }
}

fn solve_stage1(input: &Grid) -> u64 {
    input.dijkstra()
}

fn solve_stage2(input: &Grid) -> u64 {
    input.dijkstra2()
}

pub struct Day17Solver;
impl AdventOfCodeDay<'_> for Day17Solver {
    type ParsedInput = Grid;

    type Part1Output = u64;

    type Part2Output = u64;

    fn solve_part1(input: &Self::ParsedInput) -> Self::Part1Output {
        solve_stage1(input)
    }

    fn solve_part2(input: &Self::ParsedInput) -> Self::Part2Output {
        solve_stage2(input)
    }

    fn parse_input(input: &str) -> Self::ParsedInput {
        input.parse().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use aoc_traits::AdventOfCodeDay;

    use crate::Day17Solver;

    const TEST_INPUT: &str = r#"2413432311323
3215453535623
3255245654254
3446585845452
4546657867536
1438598798454
4457876987766
3637877979653
4654967986887
4564679986453
1224686865563
2546548887735
4322674655533"#;
    #[test]
    fn test_stage1() {
        let input = Day17Solver::parse_input(TEST_INPUT);
        assert_eq!(super::solve_stage1(&input), 102);
    }
    #[test]
    fn test_stage2() {
        let input = Day17Solver::parse_input(TEST_INPUT);
        assert_eq!(super::solve_stage2(&input), 94);
    }
    #[test]
    fn test_stage2_2() {
        let input = Day17Solver::parse_input(
            "111111111111
999999999991
999999999991
999999999991
999999999991",
        );
        assert_eq!(super::solve_stage2(&input), 71);
    }
}
