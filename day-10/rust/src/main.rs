use std::cmp::Ordering;
use std::collections::{HashMap, HashSet};
use std::fmt::Write;
use std::iter::successors;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Tile {
    Vertical,
    Horizontal,
    NorthEast,
    NorthWest,
    SouthWest,
    SouthEast,
    Ground,
    Start,
}

impl std::fmt::Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_char(self.as_char())
    }
}

impl Tile {
    fn parse(c: char) -> Self {
        match c {
            '|' => Self::Vertical,
            '-' => Self::Horizontal,
            'L' => Self::NorthEast,
            'J' => Self::NorthWest,
            '7' => Self::SouthWest,
            'F' => Self::SouthEast,
            '.' => Self::Ground,
            'S' => Self::Start,
            _ => unreachable!("bad input: {c}"),
        }
    }

    fn as_char(self) -> char {
        match self {
            Tile::Vertical => '|',
            Tile::Horizontal => '-',
            Tile::NorthEast => 'L',
            Tile::NorthWest => 'J',
            Tile::SouthWest => '7',
            Tile::SouthEast => 'F',
            Tile::Ground => '.',
            Tile::Start => 'S',
        }
    }
}

fn main() {
    // let input = include_str!("../../test_input.txt");
    // let input = include_str!("../../test_input2.txt");
    // let input = include_str!("../../test_input3.txt");
    // let input = include_str!("../../test_input4.txt");
    // let input = include_str!("../../test_input5.txt");
    // let input = include_str!("../../test_input6.txt");
    // let input = include_str!("../../test_input7.txt");
    // let input = include_str!("../../test_input8.txt");
    let input = include_str!("../../input.txt");

    let mut grid: Vec<Vec<Tile>> = input
        .lines()
        .map(|line| line.chars().map(Tile::parse).collect::<Vec<Tile>>())
        .collect();

    // put ground around the edges so we don't have to bother checking bounds
    grid.iter_mut().for_each(|row| {
        row.insert(0, Tile::Ground);
        row.push(Tile::Ground);
    });

    grid.insert(0, vec![Tile::Ground; grid.len() + 2]);
    grid.push(vec![Tile::Ground; grid.len() + 1]);

    let (y, x) = grid
        .iter()
        .enumerate()
        .find_map(|(y, line)| {
            line.iter()
                .enumerate()
                .find(|(_, tile)| **tile == Tile::Start)
                .map(|(x, _)| (y, x))
        })
        .unwrap();

    let start = Point { y, x };
    let pipe = part1(&grid, start);
    part2(grid, pipe, start);
}

fn part1(grid: &Grid, start: Point) -> HashSet<Point> {
    // Possible first steps
    let routes = start.reachable_neighbors(grid);
    let mut distances = HashMap::<Point, u32>::from_iter([(start, 0)]);
    let mut pipe: HashSet<Point> = [start].into();

    for mut route in routes {
        distances.insert(route, 1);
        let mut distance = 1;
        let mut came_from = start;
        while route != start {
            let next = route
                .reachable_neighbors(grid)
                .find(|n| *n != came_from)
                .unwrap();

            distance += 1;
            distances
                .entry(next)
                .and_modify(|dist| *dist = distance.min(*dist))
                .or_insert(distance);

            pipe.insert(route);
            came_from = route;
            route = next;
        }
    }

    let part1: u32 = distances.values().max().copied().unwrap();
    println!("part1 = {part1}");

    pipe
}

fn part2(mut grid: Grid, pipe: HashSet<Point>, start: Point) {
    remove_superfluous_tiles(&mut grid, &pipe);

    // Replace start with whatever it should be
    let what_to_replace_start_with = start.calculate_start(&grid);
    let start_tile = grid
        .iter_mut()
        .find_map(|row| row.iter_mut().find(|tile| **tile == Tile::Start))
        .unwrap();
    *start_tile = what_to_replace_start_with;

    let mut inside = HashSet::new();
    for y in 0..grid.len() {
        for x in 0..grid.len() {
            let start = Point { y, x };
            if pipe.contains(&start) {
                continue;
            }

            let mut crossings = 0;
            let mut last_cross = None;
            for point in successors(Some(start), |p| p.checked_down(&grid)) {
                match (point.lookup(&grid), last_cross) {
                    (Tile::Horizontal, _) => {
                        crossings += 1;
                        last_cross = None;
                    }
                    (Tile::NorthWest | Tile::SouthWest, None) => last_cross = Some(Direction::Left),
                    (Tile::NorthEast | Tile::SouthEast, None) => {
                        last_cross = Some(Direction::Right)
                    }
                    (Tile::NorthWest | Tile::SouthWest, Some(Direction::Right)) => {
                        crossings += 1;
                        last_cross = None;
                    }
                    (Tile::NorthEast | Tile::SouthEast, Some(Direction::Left)) => {
                        crossings += 1;
                        last_cross = None;
                    }
                    (Tile::NorthWest, Some(Direction::Left)) => last_cross = None,
                    (Tile::NorthEast, Some(Direction::Right)) => last_cross = None,
                    _ => (),
                }
            }

            if crossings % 2 == 1 {
                inside.insert(start);
            }
        }
    }

    println!("part2 = {}", inside.len());
}

fn remove_superfluous_tiles(grid: &mut [Vec<Tile>], pipe: &HashSet<Point>) {
    for (y, row) in grid.iter_mut().enumerate() {
        for (x, tile) in row.iter_mut().enumerate() {
            let point = Point { y, x };
            if !pipe.contains(&point) {
                *tile = Tile::Ground;
            }
        }
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
struct Point {
    y: usize,
    x: usize,
}

impl std::fmt::Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // minus 1 to account for padding :(
        // write!(f, "({y}, {x})", y = self.y - 1, x = self.x - 1)
        write!(f, "({y}, {x})", y = self.y, x = self.x)
    }
}

impl std::fmt::Debug for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self}")
    }
}

type Grid = Vec<Vec<Tile>>;

impl Point {
    fn lookup(&self, grid: &Grid) -> Tile {
        grid[self.y][self.x]
    }

    fn neighbors(&self) -> [Self; 4] {
        [self.up(), self.down(), self.left(), self.right()]
    }

    fn reachable_neighbors<'g, 's: 'g>(
        &'s self,
        grid: &'g Grid,
    ) -> impl Iterator<Item = Self> + 'g {
        self.neighbors()
            .into_iter()
            .filter(|neighbor| self.reachable(neighbor, grid))
    }

    fn reachable(&self, other: &Self, grid: &Grid) -> bool {
        let direction = self.diff(other);

        #[allow(clippy::match_like_matches_macro)]
        match (direction, self.lookup(grid), other.lookup(grid)) {
            (
                Direction::Up,
                Tile::Vertical | Tile::NorthEast | Tile::NorthWest | Tile::Start,
                Tile::Vertical | Tile::SouthEast | Tile::SouthWest | Tile::Start,
            ) => true,
            (
                Direction::Down,
                Tile::Vertical | Tile::SouthEast | Tile::SouthWest | Tile::Start,
                Tile::Vertical | Tile::NorthEast | Tile::NorthWest | Tile::Start,
            ) => true,
            (
                Direction::Left,
                Tile::Horizontal | Tile::NorthWest | Tile::SouthWest | Tile::Start,
                Tile::Horizontal | Tile::NorthEast | Tile::SouthEast | Tile::Start,
            ) => true,
            (
                Direction::Right,
                Tile::Horizontal | Tile::NorthEast | Tile::SouthEast | Tile::Start,
                Tile::Horizontal | Tile::NorthWest | Tile::SouthWest | Tile::Start,
            ) => true,
            _ => false,
        }
    }

    fn up(&self) -> Self {
        Self {
            y: self.y - 1,
            ..*self
        }
    }

    fn down(&self) -> Self {
        Self {
            y: self.y + 1,
            ..*self
        }
    }

    fn left(&self) -> Self {
        Self {
            x: self.x - 1,
            ..*self
        }
    }

    fn right(&self) -> Self {
        Self {
            x: self.x + 1,
            ..*self
        }
    }

    fn checked_down(&self, grid: &Grid) -> Option<Self> {
        let y = self.y + 1;
        match y >= grid.len() {
            true => None,
            false => Some(Self { y, ..*self }),
        }
    }

    /// Returns the direction to move in to go from self to other.
    ///
    /// Assumes the points are neighbors.
    fn diff(&self, other: &Self) -> Direction {
        match (self.y.cmp(&other.y), self.x.cmp(&other.x)) {
            (Ordering::Less, _) => Direction::Down,
            (Ordering::Greater, _) => Direction::Up,
            (_, Ordering::Less) => Direction::Right,
            (_, Ordering::Greater) => Direction::Left,
            _ => unreachable!("bad diff: ({self:?}, {other:?})"),
        }
    }

    fn calculate_start(&self, grid: &Grid) -> Tile {
        let up = matches!(
            self.up().lookup(grid),
            Tile::Vertical | Tile::SouthWest | Tile::SouthEast
        );
        let down = matches!(
            self.down().lookup(grid),
            Tile::Vertical | Tile::NorthWest | Tile::NorthEast
        );
        let left = matches!(
            self.left().lookup(grid),
            Tile::Horizontal | Tile::NorthEast | Tile::SouthEast
        );
        let right = matches!(
            self.right().lookup(grid),
            Tile::Horizontal | Tile::NorthWest | Tile::SouthWest
        );

        match (up, down, left, right) {
            (true, true, false, false) => Tile::Vertical,
            (true, false, true, false) => Tile::NorthWest,
            (true, false, false, true) => Tile::NorthEast,
            (false, true, true, false) => Tile::SouthWest,
            (false, true, false, true) => Tile::SouthEast,
            (false, false, true, true) => Tile::Horizontal,
            _ => unreachable!(":("),
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[allow(dead_code)]
fn debug(grid: &Grid) {
    for row in grid {
        for tile in row {
            print!("{tile}");
        }
        println!();
    }
}
