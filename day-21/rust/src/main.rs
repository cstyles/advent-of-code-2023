use std::collections::{hash_map::Entry, HashMap, HashSet, VecDeque};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Tile {
    Garden,
    Rock,
}

impl Tile {
    fn parse(c: char) -> Self {
        match c {
            '.' => Self::Garden,
            'S' => Self::Garden,
            '#' => Self::Rock,
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

    fn reachable_neighbors<'a>(&'a self, grid: &'a Grid) -> impl Iterator<Item = Self> + 'a {
        [self.up(), self.down(grid), self.left(), self.right(grid)]
            .into_iter()
            .flatten()
            .filter(|point| point.lookup(grid) == Tile::Garden)
    }

    fn lookup(&self, grid: &Grid) -> Tile {
        grid[self.y][self.x]
    }

    fn manhattan_distance(&self, other: &Self) -> usize {
        self.y.abs_diff(other.y) + self.x.abs_diff(other.x)
    }
}

type Grid = Vec<Vec<Tile>>;

fn main() {
    // let input = include_str!("../../test_input.txt");
    let input = include_str!("../../input.txt");

    let grid: Grid = input
        .lines()
        .map(|line| line.chars().map(Tile::parse).collect())
        .collect();

    let start = input
        .lines()
        .enumerate()
        .find_map(|(y, line)| line.chars().position(|c| c == 'S').map(|x| Point { y, x }))
        .unwrap();

    let mut reachable_tiles: HashMap<Point, u32> = [].into();
    let mut queue: VecDeque<QueueItem> = [QueueItem {
        point: start,
        distance: 0,
    }]
    .into();

    while let Some(QueueItem { point, distance }) = queue.pop_front() {
        match reachable_tiles.entry(point) {
            Entry::Occupied(_) => continue,
            Entry::Vacant(entry) => {
                entry.insert(distance);
                queue.extend(point.reachable_neighbors(&grid).map(|point| QueueItem {
                    point,
                    distance: distance + 1,
                }))
            }
        };
    }

    let part1 = reachable_tiles
        .into_iter()
        .filter(|(_, distance)| {
            // let distance = point.manhattan_distance(&start);
            distance % 2 == 0 && *distance <= 64
        })
        .count();

    println!("part1 = {part1:?}");
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct QueueItem {
    point: Point,
    distance: u32,
}
