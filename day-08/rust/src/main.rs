use std::collections::HashMap;

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

    // dbg!(lrs);
    // dbg!(graph);

    part1(lrs.into_iter().cycle(), graph);
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
