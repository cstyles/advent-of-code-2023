use std::{cmp::Ordering, collections::HashSet};

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
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Point {
    y: usize,
    x: usize,
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

    fn can_move_to(&self, other: &Self, grid: &Grid) -> bool {
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
    let input = include_str!("../../input.txt");

    let grid: Grid = input
        .lines()
        .map(|line| line.chars().map(Tile::parse).collect())
        .collect();

    // dbg!(grid);

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
            .neighbors(&grid)
            .filter(|neighbor| point.can_move_to(neighbor, &grid))
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

    // let part1 = todo;
    println!("part1 = {part1}");
}

struct StackItem {
    point: Point,
    visited: HashSet<Point>,
}
