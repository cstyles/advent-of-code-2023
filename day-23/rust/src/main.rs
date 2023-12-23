use std::cmp::Ordering;
use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Tile {
    Path,
    Forest,
    UpSlope,
    DownSlope,
    LeftSlope,
    RightSlope,
}

impl Tile {
    fn parse(c: char) -> Self {
        match c {
            '.' => Self::Path,
            '#' => Self::Forest,
            '^' => Self::UpSlope,
            'v' => Self::DownSlope,
            '<' => Self::LeftSlope,
            '>' => Self::RightSlope,
            _ => unreachable!("bad input: {c}"),
        }
    }

    fn as_char(self) -> char {
        match self {
            Self::Path => '.',
            Self::Forest => '#',
            Self::UpSlope => '^',
            Self::DownSlope => 'v',
            Self::LeftSlope => '<',
            Self::RightSlope => '>',
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
struct Point {
    y: usize,
    x: usize,
}

impl std::fmt::Debug for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "(y = {}, x = {})", self.y, self.x)
    }
}

impl Point {
    fn up(&self) -> Option<Self> {
        self.y.checked_sub(1).map(|y| Point { y, ..*self })
    }

    fn down(&self, grid: &Grid) -> Option<Self> {
        match self.y < grid.len() - 1 {
            false => None,
            true => Some(Point {
                y: self.y + 1,
                ..*self
            }),
        }
    }

    fn left(&self) -> Option<Self> {
        self.x.checked_sub(1).map(|x| Point { x, ..*self })
    }

    fn right(&self, grid: &Grid) -> Option<Self> {
        match self.x < grid[0].len() - 1 {
            false => None,
            true => Some(Point {
                x: self.x + 1,
                ..*self
            }),
        }
    }

    fn neighbors(&self, grid: &Grid) -> impl Iterator<Item = Self> {
        [self.up(), self.down(grid), self.left(), self.right(grid)]
            .into_iter()
            .flatten()
    }

    fn lookup(&self, grid: &Grid) -> Tile {
        grid[self.y][self.x]
    }

    fn can_move_to<const PART_TWO: bool>(&self, other: &Self, grid: &Grid) -> bool {
        if PART_TWO {
            match (self.direction_to(other), other.lookup(grid)) {
                (_, Tile::Forest) => false,
                (
                    _,
                    Tile::Path
                    | Tile::UpSlope
                    | Tile::DownSlope
                    | Tile::LeftSlope
                    | Tile::RightSlope,
                ) => true,
            }
        } else {
            match (self.direction_to(other), other.lookup(grid)) {
                (_, Tile::Forest) => false,
                (_, Tile::Path) => true,
                (Direction::Up, Tile::UpSlope) => true,
                (Direction::Up, _) => false,
                (Direction::Down, Tile::DownSlope) => true,
                (Direction::Down, _) => false,
                (Direction::Left, Tile::LeftSlope) => true,
                (Direction::Left, _) => false,
                (Direction::Right, Tile::RightSlope) => true,
                (Direction::Right, _) => false,
            }
        }
    }

    fn direction_to(&self, other: &Self) -> Direction {
        match (self.y.cmp(&other.y), self.x.cmp(&other.x)) {
            (Ordering::Greater, _) => Direction::Up,
            (Ordering::Less, _) => Direction::Down,
            (_, Ordering::Less) => Direction::Right,
            (_, Ordering::Greater) => Direction::Left,
            _ => unreachable!(),
        }
    }
}

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

type Grid = Vec<Vec<Tile>>;

fn main() {
    // let input = include_str!("../../test_input.txt");
    // let input = include_str!("../../test_input2.txt");
    let input = include_str!("../../input.txt");

    let grid: Grid = input
        .lines()
        .map(|line| line.chars().map(Tile::parse).collect())
        .collect();

    part1(&grid);
    part2(grid);
}

fn part1(grid: &Grid) {
    let mut stack = vec![StackItem {
        point: Point { y: 1, x: 1 },
        visited: [].into(),
    }];

    let mut part1 = 0;
    while let Some(StackItem { point, mut visited }) = stack.pop() {
        if !visited.insert(point) {
            unreachable!("revisited {point:?}");
        }

        let targets: Vec<Point> = point
            .neighbors(grid)
            .filter(|neighbor| point.can_move_to::<false>(neighbor, grid))
            .filter(|neighbor| !visited.contains(neighbor))
            .collect();

        if targets.is_empty() {
            part1 = part1.max(visited.len());
        } else if targets.len() == 1 {
            stack.push(StackItem {
                point: targets[0],
                visited,
            });
        } else {
            for target in targets {
                stack.push(StackItem {
                    point: target,
                    visited: visited.clone(),
                });
            }
        }
    }

    println!("part1 = {part1}");
}

struct StackItem {
    point: Point,
    visited: HashSet<Point>,
}

fn part2(grid: Grid) {
    let graph = Graph::build(&grid);

    let start = Point { y: 0, x: 1 };
    let destination = Point {
        y: grid.len() - 1,
        x: grid[0].len() - 2,
    };

    let mut stack = vec![ExploreGraphStackItem {
        vertex: start,
        visited: [].into(),
        weight: 0,
    }];

    let edges_for_point: HashMap<Point, Vec<Edge>> = graph
        .vertices
        .iter()
        .map(|vertex| {
            (
                *vertex,
                graph
                    .edges
                    .iter()
                    .filter(|edge| edge.contains(*vertex))
                    .copied()
                    .collect(),
            )
        })
        .collect();

    let final_edge = edges_for_point.get(&destination).unwrap()[0];
    let vertex_before_destination = final_edge.other_end(destination);

    let mut part2 = 0;
    while let Some(mut explore_item) = stack.pop() {
        let possible_edges: Vec<Edge> = if explore_item.vertex == vertex_before_destination {
            // Always go toward the end because otherwise we'll get stuck.
            vec![final_edge]
        } else {
            edges_for_point
                .get(&explore_item.vertex)
                .unwrap()
                .iter()
                .filter(|edge| {
                    !(explore_item.visited.contains(&edge.start)
                        && explore_item.visited.contains(&edge.end))
                })
                .copied()
                .collect()
        };

        match possible_edges.len() {
            0 => {
                if explore_item.vertex == destination {
                    part2 = part2.max(explore_item.weight);
                }
            }
            1 => {
                let edge = possible_edges[0];
                explore_item
                    .visited
                    .insert(edge.other_end(explore_item.vertex));
                stack.push(ExploreGraphStackItem {
                    vertex: edge.other_end(explore_item.vertex),
                    visited: explore_item.visited,
                    weight: explore_item.weight + edge.weight,
                });
            }
            _ => {
                // Reduce clones by moving the last item in `possible_edges`.
                let length = possible_edges.len();
                let mut iter = possible_edges.into_iter();
                for edge in iter.by_ref().take(length - 1) {
                    let mut visited = explore_item.visited.clone();
                    visited.insert(edge.other_end(explore_item.vertex));
                    stack.push(ExploreGraphStackItem {
                        vertex: edge.other_end(explore_item.vertex),
                        visited,
                        weight: explore_item.weight + edge.weight,
                    });
                }

                let edge = iter.next().unwrap();
                let mut visited = explore_item.visited;
                visited.insert(edge.other_end(explore_item.vertex));
                stack.push(ExploreGraphStackItem {
                    vertex: edge.other_end(explore_item.vertex),
                    visited,
                    weight: explore_item.weight + edge.weight,
                });
            }
        };
    }

    println!("part2 = {part2}");
}

#[derive(Debug, Default)]
struct Graph {
    vertices: HashSet<Point>,
    edges: HashSet<Edge>,
}

impl Graph {
    fn build(grid: &Grid) -> Graph {
        let mut graph = Graph::default();

        let start = Point { y: 0, x: 1 };
        graph.vertices.insert(start);

        let destination = Point {
            y: grid.len() - 1,
            x: grid[0].len() - 2,
        };
        graph.vertices.insert(destination);

        let mut visited: HashSet<Point> = [destination].into();
        let mut stack = vec![BuildGraphStackItem {
            started_at: start,
            current_point: start,
            distance: 0,
        }];

        while let Some(BuildGraphStackItem {
            started_at,
            current_point,
            distance,
        }) = stack.pop()
        {
            match visited.insert(current_point) {
                true => (),
                false => {
                    if graph.vertices.contains(&current_point) && started_at != current_point {
                        // Looped around to an existing vertex.
                        graph
                            .edges
                            .insert(Edge::new(started_at, current_point, distance));
                    }

                    continue;
                }
            }

            let neighbors: Vec<Point> = current_point
                .neighbors(grid)
                .filter(|p| current_point.can_move_to::<true>(p, grid))
                .collect();

            match neighbors.len() {
                // Keep marching down the path.
                1 | 2 => stack.extend(neighbors.into_iter().map(|neighbor| BuildGraphStackItem {
                    started_at,
                    current_point: neighbor,
                    distance: distance + 1,
                })),
                // Reached an intersection.
                3 | 4 => {
                    graph.vertices.insert(current_point);
                    graph
                        .edges
                        .insert(Edge::new(started_at, current_point, distance));

                    stack.extend(neighbors.into_iter().map(|p| BuildGraphStackItem {
                        started_at: current_point,
                        current_point: p,
                        distance: 1,
                    }));
                }
                _ => unreachable!("{neighbors:?}"),
            }
        }

        graph
    }
}

#[derive(Debug, Copy, Clone, Default, PartialEq, Eq, Hash)]
struct Edge {
    start: Point,
    end: Point,
    weight: u32,
}

impl Edge {
    // Make a "canonical" edge. Passing a & b in either order should always
    // yield the same `Edge`.
    fn new(a: Point, b: Point, weight: u32) -> Self {
        match a < b {
            true => Self {
                start: a,
                end: b,
                weight,
            },
            false => Self {
                start: b,
                end: a,
                weight,
            },
        }
    }

    fn contains(&self, point: Point) -> bool {
        self.start == point || self.end == point
    }

    fn other_end(&self, vertex: Point) -> Point {
        match self.start == vertex {
            true => self.end,
            false => self.start,
        }
    }
}

struct BuildGraphStackItem {
    started_at: Point,
    current_point: Point,
    distance: u32,
}

struct ExploreGraphStackItem {
    vertex: Point,
    visited: HashSet<Point>,
    weight: u32,
}

#[allow(dead_code)]
fn debug(grid: &Grid, visited: HashSet<Point>) {
    for (y, row) in grid.iter().enumerate() {
        for (x, tile) in row.iter().enumerate() {
            let point = Point { y, x };
            if visited.contains(&point) {
                print!("O");
            } else {
                print!("{}", tile.as_char());
            }
        }
        println!();
    }
}
