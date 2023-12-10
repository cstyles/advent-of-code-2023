use std::cmp::Ordering;
use std::collections::{HashMap, HashSet, VecDeque};
use std::fmt::Write;

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
    // try to escape
    // dbg!(grounds);

    // debug(&grid);
    remove_superfluous_tiles(&mut grid, &pipe);
    // println!();
    // debug(&grid);
    // println!();

    // check grounds after removing superfluous tiles
    let grounds: HashSet<Point> = grid
        .iter()
        .enumerate()
        .flat_map(|(y, row)| {
            row.iter()
                .enumerate()
                .map(move |(x, tile)| (Point { y, x }, tile))
        })
        .filter(|(_, tile)| **tile == Tile::Ground)
        .map(|(point, _)| point)
        .filter(|point| point.x != 0 && point.x != 140 && point.y != 0 && point.y != 140)
        .collect();

    // Replace start with whatever it should be
    let what_to_replace_start_with = start.calculate_start(&grid);
    let start_tile = grid
        .iter_mut()
        .find_map(|row| row.iter_mut().find(|tile| **tile == Tile::Start))
        .unwrap();
    *start_tile = what_to_replace_start_with;
    // debug(&grid);
    // println!();

    let start = QueuePoint {
        point: Point { y: 0, x: 0 },
        between: None,
    };
    let mut queue = VecDeque::from([start]);
    let mut seen = HashSet::new();

    // while let Some(queue_point) = queue.pop_front() {
    while let Some(queue_point) = queue.pop_back() {
        if !seen.insert(queue_point.point) {
            continue;
        }

        // dbg!(queue_point);

        let neighbors = queue_point.neighbors_animal(&grid);

        for neighbor in neighbors {
            match neighbor {
                Reachable::Unreachable => continue,
                Reachable::Reachable(neighbor) => queue.push_back(neighbor),
            }
        }
    }

    let inside: HashSet<Point> = grounds.difference(&seen).copied().collect();
    // println!();
    // dbg!(&inside);
    println!("part2 = {}", inside.len());

    // let test = Point { y: 6, x: 7 };
    // let neighbors: Vec<_> = test.reachable_neighbors_animal(&grid).collect();
    // dbg!(neighbors);
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

#[allow(dead_code)]
fn debug(grid: &Grid) {
    for row in grid {
        for tile in row {
            print!("{tile}");
        }
        println!();
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
struct QueuePoint {
    point: Point,
    between: Option<Direction>,
}

impl QueuePoint {
    fn checked_up(&self) -> Option<Self> {
        self.point.y.checked_sub(1).map(|y| Self {
            point: Point { y, ..self.point },
            ..*self
        })
    }

    fn checked_down(&self, grid: &Grid) -> Option<Self> {
        let y = self.point.y + 1;
        match y >= grid.len() {
            true => None,
            false => Some(Self {
                point: Point { y, ..self.point },
                ..*self
            }),
        }
    }

    fn checked_left(&self) -> Option<Self> {
        self.point.x.checked_sub(1).map(|x| Self {
            point: Point { x, ..self.point },
            ..*self
        })
    }

    fn checked_right(&self, grid: &Grid) -> Option<Self> {
        let x = self.point.x + 1;
        match x >= grid.len() {
            true => None,
            false => Some(Self {
                point: Point { x, ..self.point },
                ..*self
            }),
        }
    }

    fn neighbors_animal<'g, 's: 'g>(
        &'s self,
        grid: &'g Grid,
    ) -> impl Iterator<Item = Reachable> + 'g {
        [
            self.checked_up(),
            self.checked_down(grid),
            self.checked_left(),
            self.checked_right(grid),
        ]
        .into_iter()
        .flatten()
        .map(|neighbor| self.reachable_animal(&neighbor, grid))
    }

    // fn reachable_neighbors_animal<'g, 's: 'g>(
    //     &'s self,
    //     grid: &'g Grid,
    // ) -> impl Iterator<Item = Reachable> + 'g {
    //     self.neighbors_animal(grid)
    //         .map(|neighbor| self.reachable_animal(neighbor, grid))
    // }

    fn with(self, between: Option<Direction>) -> Self {
        Self { between, ..self }
    }

    fn reachable_animal(&self, other: &Self, grid: &Grid) -> Reachable {
        let direction = self.point.diff(&other.point);

        // TODO: Start?
        #[allow(clippy::match_like_matches_macro)]
        match (
            direction,
            self.point.lookup(grid),
            other.point.lookup(grid),
            self.between,
        ) {
            // Unreachable cases
            (_, Tile::Start, _, _) => unreachable!("start should be gone"),
            (_, _, Tile::Start, _) => unreachable!("start should be gone"),
            (
                _,
                Tile::NorthEast
                | Tile::NorthWest
                | Tile::SouthWest
                | Tile::SouthEast
                | Tile::Vertical
                | Tile::Horizontal,
                _,
                None,
            ) => unreachable!("between must be set"),
            (_, Tile::Vertical, _, Some(Direction::Up | Direction::Down)) => {
                unreachable!("bad between")
            }
            (_, Tile::Horizontal, _, Some(Direction::Left | Direction::Right)) => {
                unreachable!("bad between")
            }
            (
                Direction::Right,
                Tile::Vertical,
                Tile::Horizontal | Tile::NorthWest | Tile::SouthWest,
                _,
            ) => unreachable!("bad map"),
            (
                Direction::Left,
                Tile::Vertical,
                Tile::Horizontal | Tile::NorthEast | Tile::SouthEast,
                _,
            ) => unreachable!("bad map"),
            (
                Direction::Up,
                Tile::Horizontal,
                Tile::Vertical | Tile::SouthWest | Tile::SouthEast,
                _,
            ) => unreachable!("bad map"),
            (
                Direction::Down,
                Tile::Horizontal,
                Tile::Vertical | Tile::NorthWest | Tile::NorthEast,
                _,
            ) => unreachable!("bad map"),
            (
                Direction::Up,
                Tile::NorthWest,
                Tile::Horizontal | Tile::NorthWest | Tile::NorthEast,
                _,
            ) => unreachable!("bad map"),
            (
                Direction::Down,
                Tile::NorthWest,
                Tile::Vertical | Tile::NorthWest | Tile::NorthEast,
                _,
            ) => unreachable!("bad map"),
            (
                Direction::Left,
                Tile::NorthWest,
                Tile::Vertical | Tile::NorthWest | Tile::SouthWest,
                _,
            ) => unreachable!("bad map"),
            (
                Direction::Right,
                Tile::NorthWest,
                Tile::Horizontal | Tile::NorthWest | Tile::SouthWest,
                _,
            ) => unreachable!("bad map"),
            (
                Direction::Up,
                Tile::SouthWest,
                Tile::Vertical | Tile::SouthWest | Tile::SouthEast,
                _,
            ) => unreachable!("bad map"),
            (
                Direction::Down,
                Tile::SouthWest,
                Tile::Horizontal | Tile::SouthWest | Tile::SouthEast,
                _,
            ) => unreachable!("bad map"),
            (
                Direction::Left,
                Tile::SouthWest,
                Tile::Vertical | Tile::NorthWest | Tile::SouthWest,
                _,
            ) => unreachable!("bad map"),
            (
                Direction::Right,
                Tile::SouthWest,
                Tile::Horizontal | Tile::NorthWest | Tile::SouthWest,
                _,
            ) => unreachable!("bad map"),
            (
                Direction::Up,
                Tile::SouthEast,
                Tile::Vertical | Tile::SouthWest | Tile::SouthEast,
                _,
            ) => unreachable!("bad map"),
            (
                Direction::Down,
                Tile::SouthEast,
                Tile::Horizontal | Tile::SouthWest | Tile::SouthEast,
                _,
            ) => unreachable!("bad map"),
            (
                Direction::Left,
                Tile::SouthEast,
                Tile::Horizontal | Tile::NorthEast | Tile::SouthEast,
                _,
            ) => unreachable!("bad map"),
            (
                Direction::Right,
                Tile::SouthEast,
                Tile::Vertical | Tile::NorthEast | Tile::SouthEast,
                _,
            ) => unreachable!("bad map"),
            (
                Direction::Up,
                Tile::NorthEast,
                Tile::Horizontal | Tile::NorthWest | Tile::NorthEast,
                _,
            ) => unreachable!("bad map"),
            (
                Direction::Down,
                Tile::NorthEast,
                Tile::Vertical | Tile::NorthWest | Tile::NorthEast,
                _,
            ) => unreachable!("bad map"),
            (
                Direction::Left,
                Tile::NorthEast,
                Tile::Horizontal | Tile::NorthEast | Tile::SouthEast,
                _,
            ) => unreachable!("bad map"),
            (
                Direction::Right,
                Tile::NorthEast,
                Tile::Vertical | Tile::NorthEast | Tile::SouthEast,
                _,
            ) => unreachable!("bad map"),

            // Always allowed to move into Vertical & Horizontal tiles from the correct directions.
            (Direction::Up | Direction::Down, Tile::Vertical, Tile::Vertical, _) => {
                Reachable::Reachable(*other)
            }
            (Direction::Left | Direction::Right, Tile::Horizontal, Tile::Horizontal, _) => {
                Reachable::Reachable(*other)
            }

            // Same as above but rounding a corner updates `between`.
            (
                Direction::Up,
                Tile::NorthWest,
                Tile::Vertical,
                Some(Direction::Down | Direction::Right),
            ) => Reachable::Reachable(other.with(Some(Direction::Right))),
            (
                Direction::Up,
                Tile::NorthWest,
                Tile::Vertical,
                Some(Direction::Up | Direction::Left),
            ) => Reachable::Reachable(other.with(Some(Direction::Left))),
            (
                Direction::Up,
                Tile::NorthEast,
                Tile::Vertical,
                Some(Direction::Down | Direction::Left),
            ) => Reachable::Reachable(other.with(Some(Direction::Left))),
            (
                Direction::Up,
                Tile::NorthEast,
                Tile::Vertical,
                Some(Direction::Up | Direction::Right),
            ) => Reachable::Reachable(other.with(Some(Direction::Right))),
            (
                Direction::Down,
                Tile::SouthWest,
                Tile::Vertical,
                Some(Direction::Up | Direction::Right),
            ) => Reachable::Reachable(other.with(Some(Direction::Right))),
            (
                Direction::Down,
                Tile::SouthWest,
                Tile::Vertical,
                Some(Direction::Down | Direction::Left),
            ) => Reachable::Reachable(other.with(Some(Direction::Left))),
            (
                Direction::Down,
                Tile::SouthEast,
                Tile::Vertical,
                Some(Direction::Down | Direction::Right),
            ) => Reachable::Reachable(other.with(Some(Direction::Right))),
            (
                Direction::Down,
                Tile::SouthEast,
                Tile::Vertical,
                Some(Direction::Up | Direction::Left),
            ) => Reachable::Reachable(other.with(Some(Direction::Left))),
            (
                Direction::Right,
                Tile::SouthEast,
                Tile::Horizontal,
                Some(Direction::Up | Direction::Left),
            ) => Reachable::Reachable(other.with(Some(Direction::Up))),
            (
                Direction::Right,
                Tile::SouthEast,
                Tile::Horizontal,
                Some(Direction::Down | Direction::Right),
            ) => Reachable::Reachable(other.with(Some(Direction::Down))),
            (
                Direction::Right,
                Tile::NorthEast,
                Tile::Horizontal,
                Some(Direction::Up | Direction::Right),
            ) => Reachable::Reachable(other.with(Some(Direction::Up))),
            (
                Direction::Right,
                Tile::NorthEast,
                Tile::Horizontal,
                Some(Direction::Down | Direction::Left),
            ) => Reachable::Reachable(other.with(Some(Direction::Down))),
            (
                Direction::Left,
                Tile::SouthWest,
                Tile::Horizontal,
                Some(Direction::Up | Direction::Right),
            ) => Reachable::Reachable(other.with(Some(Direction::Up))),
            (
                Direction::Left,
                Tile::SouthWest,
                Tile::Horizontal,
                Some(Direction::Down | Direction::Left),
            ) => Reachable::Reachable(other.with(Some(Direction::Down))),
            (
                Direction::Left,
                Tile::NorthWest,
                Tile::Horizontal,
                Some(Direction::Up | Direction::Left),
            ) => Reachable::Reachable(other.with(Some(Direction::Up))),
            (
                Direction::Left,
                Tile::NorthWest,
                Tile::Horizontal,
                Some(Direction::Down | Direction::Right),
            ) => Reachable::Reachable(other.with(Some(Direction::Down))),

            // Always allowed to move from Vertical & Horizontal along the axis.
            (Direction::Up | Direction::Down, Tile::Vertical, _, _) => Reachable::Reachable(*other),
            (Direction::Left | Direction::Right, Tile::Horizontal, _, _) => {
                Reachable::Reachable(*other)
            }

            // Never allowed to move into Vertical & Horizontal tiles from the wrong directions.
            (Direction::Up | Direction::Down, _, Tile::Horizontal, _) => Reachable::Unreachable,
            (Direction::Left | Direction::Right, _, Tile::Vertical, _) => Reachable::Unreachable,

            // Not allowed to cross over Horizontal / Vertical
            (Direction::Up, Tile::Horizontal, _, Some(Direction::Down)) => Reachable::Unreachable,
            (Direction::Down, Tile::Horizontal, _, Some(Direction::Up)) => Reachable::Unreachable,
            (Direction::Left, Tile::Vertical, _, Some(Direction::Right)) => Reachable::Unreachable,
            (Direction::Right, Tile::Vertical, _, Some(Direction::Left)) => Reachable::Unreachable,

            // Always allowed to leave a Vertical / Horizontal if moving the correct way.
            (Direction::Up, Tile::Horizontal, _, Some(Direction::Up)) => {
                Reachable::Reachable(other.with(Some(Direction::Down)))
            }
            (Direction::Down, Tile::Horizontal, _, Some(Direction::Down)) => {
                Reachable::Reachable(other.with(Some(Direction::Up)))
            }
            (Direction::Left, Tile::Vertical, _, Some(Direction::Left)) => {
                Reachable::Reachable(other.with(Some(Direction::Right)))
            }
            (Direction::Right, Tile::Vertical, _, Some(Direction::Right)) => {
                Reachable::Reachable(other.with(Some(Direction::Left)))
            }

            // Not allowed to cross over elbow from wrong side.
            (Direction::Up, Tile::SouthWest, _, Some(Direction::Down | Direction::Left)) => {
                Reachable::Unreachable
            }
            (Direction::Up, Tile::SouthEast, _, Some(Direction::Down | Direction::Right)) => {
                Reachable::Unreachable
            }
            (Direction::Down, Tile::NorthWest, _, Some(Direction::Up | Direction::Left)) => {
                Reachable::Unreachable
            }
            (Direction::Down, Tile::NorthEast, _, Some(Direction::Up | Direction::Right)) => {
                Reachable::Unreachable
            }
            (Direction::Left, Tile::NorthEast, _, Some(Direction::Up | Direction::Right)) => {
                Reachable::Unreachable
            }
            (Direction::Left, Tile::SouthEast, _, Some(Direction::Down | Direction::Right)) => {
                Reachable::Unreachable
            }
            (Direction::Right, Tile::NorthWest, _, Some(Direction::Up | Direction::Left)) => {
                Reachable::Unreachable
            }
            (Direction::Right, Tile::SouthWest, _, Some(Direction::Down | Direction::Left)) => {
                Reachable::Unreachable
            }

            // Moving from Ground into a gap will set the `between` value.
            (Direction::Up, Tile::Ground, Tile::NorthWest, _) => {
                Reachable::Reachable(other.with(Some(Direction::Right)))
            }
            (Direction::Up, Tile::Ground, Tile::NorthEast, _) => {
                Reachable::Reachable(other.with(Some(Direction::Left)))
            }
            (Direction::Down, Tile::Ground, Tile::SouthWest, _) => {
                Reachable::Reachable(other.with(Some(Direction::Right)))
            }
            (Direction::Down, Tile::Ground, Tile::SouthEast, _) => {
                Reachable::Reachable(other.with(Some(Direction::Left)))
            }
            (Direction::Right, Tile::Ground, Tile::SouthEast, _) => {
                Reachable::Reachable(other.with(Some(Direction::Up)))
            }
            (Direction::Right, Tile::Ground, Tile::NorthEast, _) => {
                Reachable::Reachable(other.with(Some(Direction::Down)))
            }
            (Direction::Left, Tile::Ground, Tile::SouthWest, _) => {
                Reachable::Reachable(other.with(Some(Direction::Up)))
            }
            (Direction::Left, Tile::Ground, Tile::NorthWest, _) => {
                Reachable::Reachable(other.with(Some(Direction::Down)))
            }

            // Always allowed to move from Ground into Ground.
            (_, Tile::Ground, Tile::Ground, _) => Reachable::Reachable(*other),

            // Always allowed to move to Ground if between matches (and clear between).
            (Direction::Up, _, Tile::Ground, Some(Direction::Up)) => {
                Reachable::Reachable(other.with(None))
            }
            (Direction::Down, _, Tile::Ground, Some(Direction::Down)) => {
                Reachable::Reachable(other.with(None))
            }
            (Direction::Left, _, Tile::Ground, Some(Direction::Left)) => {
                Reachable::Reachable(other.with(None))
            }
            (Direction::Right, _, Tile::Ground, Some(Direction::Right)) => {
                Reachable::Reachable(other.with(None))
            }

            // Allowed to move from elbow to Ground in some cases (and clear between).
            (Direction::Left, Tile::SouthEast, Tile::Ground, Some(Direction::Up)) => {
                Reachable::Reachable(other.with(None))
            }
            (Direction::Left, Tile::NorthEast, Tile::Ground, Some(Direction::Down)) => {
                Reachable::Reachable(other.with(None))
            }
            (Direction::Right, Tile::NorthWest, Tile::Ground, Some(Direction::Down)) => {
                Reachable::Reachable(other.with(None))
            }
            (Direction::Right, Tile::SouthWest, Tile::Ground, Some(Direction::Up)) => {
                Reachable::Reachable(other.with(None))
            }
            (Direction::Up, Tile::SouthWest, Tile::Ground, Some(Direction::Right)) => {
                Reachable::Reachable(other.with(None))
            }
            (Direction::Up, Tile::SouthEast, Tile::Ground, Some(Direction::Left)) => {
                Reachable::Reachable(other.with(None))
            }
            (Direction::Down, Tile::NorthEast, Tile::Ground, Some(Direction::Left)) => {
                Reachable::Reachable(other.with(None))
            }
            (Direction::Down, Tile::NorthWest, Tile::Ground, Some(Direction::Right)) => {
                Reachable::Reachable(other.with(None))
            }

            // Allowed to go from elbow to elbow sometimes.
            (
                Direction::Right,
                Tile::NorthWest,
                Tile::NorthEast,
                Some(Direction::Down | Direction::Right),
            ) => Reachable::Reachable(other.with(Some(Direction::Down))),
            (
                Direction::Right,
                Tile::NorthWest,
                Tile::SouthEast,
                Some(Direction::Down | Direction::Right),
            ) => Reachable::Reachable(other.with(Some(Direction::Left))),
            (
                Direction::Left,
                Tile::NorthWest,
                Tile::SouthEast,
                Some(Direction::Up | Direction::Left),
            ) => Reachable::Reachable(other.with(Some(Direction::Up))),
            (
                Direction::Left,
                Tile::NorthWest,
                Tile::SouthEast,
                Some(Direction::Down | Direction::Right),
            ) => Reachable::Reachable(other.with(Some(Direction::Right))),
            (
                Direction::Left,
                Tile::NorthWest,
                Tile::NorthEast,
                Some(Direction::Down | Direction::Right),
            ) => Reachable::Reachable(other.with(Some(Direction::Down))),
            (
                Direction::Left,
                Tile::NorthWest,
                Tile::NorthEast,
                Some(Direction::Up | Direction::Left),
            ) => Reachable::Reachable(other.with(Some(Direction::Up))),
            (
                Direction::Up,
                Tile::NorthWest,
                Tile::SouthWest,
                Some(Direction::Down | Direction::Right),
            ) => Reachable::Reachable(other.with(Some(Direction::Right))),
            (
                Direction::Up,
                Tile::NorthWest,
                Tile::SouthWest,
                Some(Direction::Up | Direction::Left),
            ) => Reachable::Reachable(other.with(Some(Direction::Left))),
            (
                Direction::Up,
                Tile::NorthWest,
                Tile::SouthEast,
                Some(Direction::Up | Direction::Left),
            ) => Reachable::Reachable(other.with(Some(Direction::Left))),
            (
                Direction::Up,
                Tile::NorthWest,
                Tile::SouthEast,
                Some(Direction::Down | Direction::Right),
            ) => Reachable::Reachable(other.with(Some(Direction::Right))),
            (
                Direction::Down,
                Tile::NorthWest,
                Tile::SouthWest,
                Some(Direction::Down | Direction::Right),
            ) => Reachable::Reachable(other.with(Some(Direction::Right))),
            (
                Direction::Down,
                Tile::NorthWest,
                Tile::SouthEast,
                Some(Direction::Down | Direction::Right),
            ) => Reachable::Reachable(other.with(Some(Direction::Up))),

            (
                Direction::Up,
                Tile::NorthEast,
                Tile::SouthWest,
                Some(Direction::Down | Direction::Left),
            ) => Reachable::Reachable(other.with(Some(Direction::Left))),
            (
                Direction::Up,
                Tile::NorthEast,
                Tile::SouthWest,
                Some(Direction::Up | Direction::Right),
            ) => Reachable::Reachable(other.with(Some(Direction::Right))),
            (
                Direction::Up,
                Tile::NorthEast,
                Tile::SouthEast,
                Some(Direction::Down | Direction::Left),
            ) => Reachable::Reachable(other.with(Some(Direction::Left))),
            (
                Direction::Up,
                Tile::NorthEast,
                Tile::SouthEast,
                Some(Direction::Up | Direction::Right),
            ) => Reachable::Reachable(other.with(Some(Direction::Right))),
            (
                Direction::Down,
                Tile::NorthEast,
                Tile::SouthWest,
                Some(Direction::Down | Direction::Left),
            ) => Reachable::Reachable(other.with(Some(Direction::Up))),
            (
                Direction::Down,
                Tile::NorthEast,
                Tile::SouthEast,
                Some(Direction::Down | Direction::Left),
            ) => Reachable::Reachable(other.with(Some(Direction::Up))),
            (
                Direction::Left,
                Tile::NorthEast,
                Tile::NorthWest,
                Some(Direction::Down | Direction::Left),
            ) => Reachable::Reachable(other.with(Some(Direction::Down))),
            (
                Direction::Left,
                Tile::NorthEast,
                Tile::SouthWest,
                Some(Direction::Down | Direction::Left),
            ) => Reachable::Reachable(other.with(Some(Direction::Right))),
            (
                Direction::Right,
                Tile::NorthEast,
                Tile::SouthWest,
                Some(Direction::Down | Direction::Left),
            ) => Reachable::Reachable(other.with(Some(Direction::Left))),
            (
                Direction::Right,
                Tile::NorthEast,
                Tile::SouthWest,
                Some(Direction::Up | Direction::Right),
            ) => Reachable::Reachable(other.with(Some(Direction::Right))),
            (
                Direction::Right,
                Tile::NorthEast,
                Tile::NorthWest,
                Some(Direction::Down | Direction::Left),
            ) => Reachable::Reachable(other.with(Some(Direction::Down))),
            (
                Direction::Right,
                Tile::NorthEast,
                Tile::NorthWest,
                Some(Direction::Up | Direction::Right),
            ) => Reachable::Reachable(other.with(Some(Direction::Up))),

            (
                Direction::Left,
                Tile::SouthEast,
                Tile::SouthWest,
                Some(Direction::Up | Direction::Left),
            ) => Reachable::Reachable(other.with(Some(Direction::Up))),
            (
                Direction::Left,
                Tile::SouthEast,
                Tile::NorthWest,
                Some(Direction::Up | Direction::Left),
            ) => Reachable::Reachable(other.with(Some(Direction::Right))),
            (
                Direction::Right,
                Tile::SouthEast,
                Tile::NorthWest,
                Some(Direction::Up | Direction::Left),
            ) => Reachable::Reachable(other.with(Some(Direction::Up))),
            (
                Direction::Right,
                Tile::SouthEast,
                Tile::NorthWest,
                Some(Direction::Down | Direction::Right),
            ) => Reachable::Reachable(other.with(Some(Direction::Down))),
            (
                Direction::Right,
                Tile::SouthEast,
                Tile::SouthWest,
                Some(Direction::Up | Direction::Left),
            ) => Reachable::Reachable(other.with(Some(Direction::Up))),
            (
                Direction::Right,
                Tile::SouthEast,
                Tile::SouthWest,
                Some(Direction::Down | Direction::Right),
            ) => Reachable::Reachable(other.with(Some(Direction::Down))),
            (
                Direction::Down,
                Tile::SouthEast,
                Tile::NorthEast,
                Some(Direction::Up | Direction::Left),
            ) => Reachable::Reachable(other.with(Some(Direction::Left))),
            (
                Direction::Down,
                Tile::SouthEast,
                Tile::NorthEast,
                Some(Direction::Down | Direction::Right),
            ) => Reachable::Reachable(other.with(Some(Direction::Right))),
            (
                Direction::Down,
                Tile::SouthEast,
                Tile::NorthWest,
                Some(Direction::Down | Direction::Right),
            ) => Reachable::Reachable(other.with(Some(Direction::Right))),
            (
                Direction::Down,
                Tile::SouthEast,
                Tile::NorthWest,
                Some(Direction::Up | Direction::Left),
            ) => Reachable::Reachable(other.with(Some(Direction::Left))),
            (
                Direction::Up,
                Tile::SouthEast,
                Tile::NorthEast,
                Some(Direction::Up | Direction::Left),
            ) => Reachable::Reachable(other.with(Some(Direction::Left))),
            (
                Direction::Up,
                Tile::SouthEast,
                Tile::NorthWest,
                Some(Direction::Up | Direction::Left),
            ) => Reachable::Reachable(other.with(Some(Direction::Down))),

            (
                Direction::Right,
                Tile::SouthWest,
                Tile::SouthEast,
                Some(Direction::Up | Direction::Right),
            ) => Reachable::Reachable(other.with(Some(Direction::Up))),
            (
                Direction::Right,
                Tile::SouthWest,
                Tile::NorthEast,
                Some(Direction::Up | Direction::Right),
            ) => Reachable::Reachable(other.with(Some(Direction::Left))),
            (
                Direction::Down,
                Tile::SouthWest,
                Tile::NorthWest,
                Some(Direction::Up | Direction::Right),
            ) => Reachable::Reachable(other.with(Some(Direction::Right))),
            (
                Direction::Down,
                Tile::SouthWest,
                Tile::NorthWest,
                Some(Direction::Down | Direction::Left),
            ) => Reachable::Reachable(other.with(Some(Direction::Left))),
            (
                Direction::Down,
                Tile::SouthWest,
                Tile::NorthEast,
                Some(Direction::Up | Direction::Right),
            ) => Reachable::Reachable(other.with(Some(Direction::Right))),
            (
                Direction::Down,
                Tile::SouthWest,
                Tile::NorthEast,
                Some(Direction::Down | Direction::Left),
            ) => Reachable::Reachable(other.with(Some(Direction::Left))),
            (
                Direction::Left,
                Tile::SouthWest,
                Tile::SouthEast,
                Some(Direction::Up | Direction::Right),
            ) => Reachable::Reachable(other.with(Some(Direction::Up))),
            (
                Direction::Left,
                Tile::SouthWest,
                Tile::SouthEast,
                Some(Direction::Down | Direction::Left),
            ) => Reachable::Reachable(other.with(Some(Direction::Down))),
            (
                Direction::Left,
                Tile::SouthWest,
                Tile::NorthEast,
                Some(Direction::Down | Direction::Left),
            ) => Reachable::Reachable(other.with(Some(Direction::Left))),
            (
                Direction::Left,
                Tile::SouthWest,
                Tile::NorthEast,
                Some(Direction::Up | Direction::Right),
            ) => Reachable::Reachable(other.with(Some(Direction::Right))),
            (
                Direction::Up,
                Tile::SouthWest,
                Tile::NorthWest,
                Some(Direction::Up | Direction::Right),
            ) => Reachable::Reachable(other.with(Some(Direction::Right))),
            (
                Direction::Up,
                Tile::SouthWest,
                Tile::NorthEast,
                Some(Direction::Up | Direction::Right),
            ) => Reachable::Reachable(other.with(Some(Direction::Down))),

            // (Direction::Up, Tile::SouthWest, Tile::Ground, ) => todo!(),
            otherwise => {
                println!();
                dbg!(self);
                dbg!(other);
                unreachable!("{otherwise:?}");
            }
        }
    }
}

enum Reachable {
    Unreachable,
    Reachable(QueuePoint),
}
