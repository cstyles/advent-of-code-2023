use std::{cmp::Ordering, collections::HashMap};

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
}

fn main() {
    // let input = include_str!("../../test_input.txt");
    // let input = include_str!("../../test_input2.txt");
    let input = include_str!("../../input.txt");

    let mut grid: Vec<Vec<Tile>> = input
        .lines()
        .map(|line| line.chars().map(Tile::parse).collect::<Vec<Tile>>())
        .collect();

    // put ground around the edges so we don't have to bother checking bounds
    grid.insert(0, vec![Tile::Ground; grid.len() + 2]);
    grid.push(vec![Tile::Ground; grid.len() + 2]);

    grid.iter_mut().for_each(|row| {
        row.insert(0, Tile::Ground);
        row.push(Tile::Ground);
    });

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

    // TODO: replace start?

    let start = Point { y, x };
    part1(&grid, start);
}

fn part1(grid: Grid, start: Point) {
    // Possible first steps
    let routes = start.reachable_neighbors(grid);
    let mut distances = HashMap::<Point, u32>::from_iter([(start, 0)]);

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

            came_from = route;
            route = next;
        }
    }

    let part1: u32 = distances.values().max().copied().unwrap();
    println!("part1 = {part1}");
}

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
struct Point {
    y: usize,
    x: usize,
}

impl std::fmt::Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // minus 1 to account for padding :(
        write!(f, "({y}, {x})", y = self.y - 1, x = self.x - 1)
    }
}

impl std::fmt::Debug for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self}")
    }
}

type Grid<'a> = &'a [Vec<Tile>];

impl Point {
    fn lookup(&self, grid: Grid) -> Tile {
        grid[self.y][self.x]
    }

    fn neighbors(&self) -> [Self; 4] {
        [self.up(), self.down(), self.left(), self.right()]
    }

    fn reachable_neighbors<'g, 's: 'g>(
        &'s self,
        grid: &'g [Vec<Tile>],
    ) -> impl Iterator<Item = Self> + 'g {
        self.neighbors()
            .into_iter()
            .filter(|neighbor| self.reachable(neighbor, grid))
    }

    fn reachable(&self, other: &Self, grid: Grid) -> bool {
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

    // fn move_(&self, direction: Direction) -> Self {
    //     match direction {
    //         Direction::Up => self.up(),
    //         Direction::Down => self.down(),
    //         Direction::Left => self.left(),
    //         Direction::Right => self.right(),
    //     }
    // }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}
