use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone, Copy)]
enum Direction {
    Left,
    Right,
}

impl Direction {
    fn parse(c: char) -> Self {
        match c {
            'L' => Self::Left,
            'R' => Self::Right,
            _ => unreachable!("bad input: {c}"),
        }
    }
}

fn main() {
    // let input = include_str!("../../test_input.txt");
    // let input = include_str!("../../test_input2.txt");
    // let input = include_str!("../../test_input3.txt");
    let input = include_str!("../../input.txt");

    let (lrs, graph) = input.split_once("\n\n").unwrap();
    let lrs: Vec<Direction> = lrs.chars().map(Direction::parse).collect();

    let graph: Graph = graph
        .lines()
        .map(|line| {
            let node = &line[0..3];
            let left = &line[7..10];
            let right = &line[12..15];

            (node, (left, right))
        })
        .collect();

    part1(lrs.clone().into_iter().cycle(), graph.clone());
    part2(lrs.into_iter().enumerate().cycle(), graph);
}

type Graph<'a> = HashMap<&'a str, (&'a str, &'a str)>;

fn part1<I: Iterator<Item = Direction>>(mut directions: I, graph: Graph) {
    let mut i = 0;
    let mut node = "AAA";

    while node != "ZZZ" {
        i += 1;
        let (left, right) = graph.get(node).unwrap();
        node = match directions.next().unwrap() {
            Direction::Left => left,
            Direction::Right => right,
        };
    }

    println!("part1 = {i}");
}

fn part2<I: Iterator<Item = (usize, Direction)>>(mut directions: I, graph: Graph) {
    let mut i = 0;
    let mut nodes: Vec<&str> = graph
        .keys()
        .copied()
        .filter(|node| node.ends_with('A'))
        .collect();

    // Positions that we've already seen
    // Keep track of the position and where we were through the direction iter
    let mut seen: Vec<HashSet<(&str, usize)>> = nodes
        .iter()
        .copied()
        .map(|node| HashSet::from([(node, 0)]))
        .collect();

    // What index we saw the first loop at
    let mut seen_indices = vec![None; nodes.len()];

    while !nodes.iter().all(|node| node.ends_with('Z')) {
        let (di, direction) = directions.next().unwrap();

        for node in nodes.iter_mut() {
            let (left, right) = graph.get(node).unwrap();
            *node = match direction {
                Direction::Left => left,
                Direction::Right => right,
            }
        }

        // For every path we're taking through
        for (index, node) in nodes.iter().copied().enumerate() {
            // Skip if we've already figured out how often this node loops
            if seen_indices[index].is_some() {
                continue;
            }

            // Check if we've looped
            if !seen.get_mut(index).unwrap().insert((node, di)) {
                // Record the minimum times needed to loop
                seen_indices[index].get_or_insert(i);
            }
        }

        if seen_indices.iter().all(Option::is_some) {
            break;
        }

        i += 1;
    }

    let diffs = seen_indices
        .into_iter()
        .map(|index| index.unwrap_or(i))
        .map(|index| index - index % 263);

    let part2 = diffs.fold(1, lcm);

    println!("part2 = {part2}");
}

fn lcm(a: u64, b: u64) -> u64 {
    (a * b) / gcd(a, b)
}

fn gcd(a: u64, b: u64) -> u64 {
    match b {
        0 => a,
        _ => gcd(b, a % b),
    }
}
