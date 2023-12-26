use std::collections::hash_map::Entry;
use std::collections::{HashMap, VecDeque};

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

    fn reachable_neighbors<'a>(&'a self, grid: &'a Grid) -> impl Iterator<Item = Self> + 'a {
        [self.up(), self.down(grid), self.left(), self.right(grid)]
            .into_iter()
            .flatten()
            .filter(|point| point.lookup(grid) == Tile::Garden)
    }

    fn lookup(&self, grid: &Grid) -> Tile {
        grid[self.y][self.x]
    }
}

type Grid = Vec<Vec<Tile>>;
type Distances = HashMap<Point, u32>;

fn main() {
    // let input = include_str!("../../test_input.txt");
    // let input = include_str!("../../test_input2.txt");
    // let input = include_str!("../../test_input3.txt");
    // let input = include_str!("../../test_input4.txt");
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

    part1(start, &grid);
    part2(grid);
}

fn part1(start: Point, grid: &Grid) {
    let mut reachable_tiles: Distances = [].into();
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
                queue.extend(point.reachable_neighbors(grid).map(|point| QueueItem {
                    point,
                    distance: distance + 1,
                }))
            }
        };
    }

    let part1 = reachable_tiles
        .into_iter()
        .filter(|(_, distance)| distance % 2 == 0 && *distance <= 64)
        .count();

    println!("part1 = {part1:?}");
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct QueueItem {
    point: Point,
    distance: u32,
}

const ACTUAL_STEPS: usize = 26_501_365;

fn part2(grid: Grid) {
    let height = grid.len();
    let width = grid[0].len();

    let x = ((ACTUAL_STEPS - width / 2) / width) as isize;
    let y = ((ACTUAL_STEPS - height / 2) / height) as isize;

    let up_cap = num_reachable_in_subgrid(&grid, -y, 0);
    let down_cap = num_reachable_in_subgrid(&grid, y, 0);
    let left_cap = num_reachable_in_subgrid(&grid, 0, -x);
    let right_cap = num_reachable_in_subgrid(&grid, 0, x);

    let up_left_small_diag = num_reachable_in_subgrid(&grid, -y, -1);
    let up_left_big_diag = num_reachable_in_subgrid(&grid, -y + 1, -1);
    let up_right_small_diag = num_reachable_in_subgrid(&grid, -y, 1);
    let up_right_big_diag = num_reachable_in_subgrid(&grid, -y + 1, 1);
    let down_left_small_diag = num_reachable_in_subgrid(&grid, 1, -x);
    let down_left_big_diag = num_reachable_in_subgrid(&grid, 1, -x + 1);
    let down_right_small_diag = num_reachable_in_subgrid(&grid, 1, x);
    let down_right_big_diag = num_reachable_in_subgrid(&grid, 1, x - 1);

    let small_diag_segments = y as usize;
    let big_diag_segments = small_diag_segments - 1;

    let caps = up_cap + down_cap + left_cap + right_cap;
    let big_diagonals = big_diag_segments
        * (up_left_big_diag + up_right_big_diag + down_left_big_diag + down_right_big_diag);
    let small_diagonals = small_diag_segments
        * (up_left_small_diag + up_right_small_diag + down_left_small_diag + down_right_small_diag);
    let diagonals = big_diagonals + small_diagonals;

    let inner_odd = num_reachable_in_subgrid(&grid, 0, 0);
    let inner_even = num_reachable_in_subgrid(&grid, 0, 1);

    let mut inner_total = inner_odd;
    let mut ring = 1;

    while ring < 202_300 {
        let per_subgrid = match ring % 2 == 0 {
            true => inner_odd,
            false => inner_even,
        };

        inner_total += ring * 4 * per_subgrid;

        ring += 1;
    }

    let part2 = caps + diagonals + inner_total;
    println!("part2 = {part2}");
}

/// `grid_y` & `grid_x` are the coordinates of the subgrid (e.g., (0, 0) is
/// the original subgrid), not a point inside a subgrid.
fn distance_to_subgrid(height: usize, width: usize, grid_y: isize, grid_x: isize) -> usize {
    match (grid_y == 0, grid_x == 0) {
        (true, true) => 0, // starting subgrid
        (true, false) => width / 2 + width * (grid_x.unsigned_abs() - 1),
        (false, true) => height / 2 + height * (grid_y.unsigned_abs() - 1),
        (false, false) => {
            width / 2
                + height / 2
                + width * (grid_x.unsigned_abs() - 1)
                + height * (grid_y.unsigned_abs() - 1)
        }
    }
}

// TODO: just re-use this for part1
fn distances_for_subgrid(grid: &Grid, y: isize, x: isize) -> Distances {
    let height = grid.len();
    let width = grid[0].len();

    //    -y
    // -x  0  +x
    //    +y
    let start = match (y.signum(), x.signum()) {
        (-1, -1) => QueueItem {
            point: Point {
                y: height - 1,
                x: width - 1,
            },
            distance: 2,
        },
        (-1, 0) => QueueItem {
            point: Point {
                y: height - 1,
                x: width / 2,
            },
            distance: 1,
        },
        (-1, 1) => QueueItem {
            point: Point {
                y: height - 1,
                x: 0,
            },
            distance: 2,
        },
        (0, -1) => QueueItem {
            point: Point {
                y: height / 2,
                x: width - 1,
            },
            distance: 1,
        },
        (0, 0) => QueueItem {
            point: Point {
                y: height / 2,
                x: width / 2,
            },
            distance: 0,
        },
        (0, 1) => QueueItem {
            point: Point {
                y: height / 2,
                x: 0,
            },
            distance: 1,
        },
        (1, -1) => QueueItem {
            point: Point { y: 0, x: width - 1 },
            distance: 2,
        },
        (1, 0) => QueueItem {
            point: Point { y: 0, x: width / 2 },
            distance: 1,
        },
        (1, 1) => QueueItem {
            point: Point { y: 0, x: 0 },
            distance: 2,
        },
        _ => unreachable!(),
    };

    let mut reachable_tiles: Distances = [].into();
    let mut queue: VecDeque<QueueItem> = [start].into();

    while let Some(QueueItem { point, distance }) = queue.pop_front() {
        match reachable_tiles.entry(point) {
            Entry::Occupied(_) => continue,
            Entry::Vacant(entry) => {
                entry.insert(distance);
                queue.extend(point.reachable_neighbors(grid).map(|point| QueueItem {
                    point,
                    distance: distance + 1,
                }))
            }
        };
    }

    reachable_tiles
}

fn num_reachable_in_subgrid(grid: &Grid, gy: isize, gx: isize) -> usize {
    let height = grid.len();
    let width = grid[0].len();
    let distance_to_here = distance_to_subgrid(height, width, gy, gx);

    distances_for_subgrid(grid, gy, gx)
        .into_iter()
        .filter(|&(_, dist)| {
            let final_dist = dist as usize + distance_to_here;
            final_dist <= ACTUAL_STEPS && final_dist % 2 == 1
        })
        .count()
}
