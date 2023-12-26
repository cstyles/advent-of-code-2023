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

    // fn neighbors(&self, grid: &Grid) -> impl Iterator<Item = Self> {
    //     [self.up(), self.down(grid), self.left(), self.right(grid)]
    //         .into_iter()
    //         .flatten()
    // }

    fn reachable_neighbors<'a>(&'a self, grid: &'a Grid) -> impl Iterator<Item = Self> + 'a {
        [self.up(), self.down(grid), self.left(), self.right(grid)]
            .into_iter()
            .flatten()
            .filter(|point| point.lookup(grid) == Tile::Garden)
    }

    fn lookup(&self, grid: &Grid) -> Tile {
        grid[self.y][self.x]
    }

    // fn manhattan_distance(&self, other: &Self) -> usize {
    //     self.y.abs_diff(other.y) + self.x.abs_diff(other.x)
    // }
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

const ACTUAL_STEPS: usize = 26_501_365;
// const ACTUAL_STEPS: usize = 15;

fn part2(grid: Grid) {
    let height = grid.len();
    let width = grid[0].len();

    // let start_distances = distances_for_subgrid(&grid, 0, 0);
    // let up_distances = distances_for_subgrid(&grid, -1, 0);
    // let down_distances = distances_for_subgrid(&grid, 1, 0);
    // let left_distances = distances_for_subgrid(&grid, 0, -1);
    // let right_distances = distances_for_subgrid(&grid, 0, 1);
    // let up_left_distances = distances_for_subgrid(&grid, -1, -1);
    // let up_right_distances = distances_for_subgrid(&grid, -1, 1);
    // let down_left_distances = distances_for_subgrid(&grid, 1, -1);
    // let down_right_distances = distances_for_subgrid(&grid, 1, 1);

    // let start_distances_max = start_distances.values().max().copied().unwrap();
    // let up_distances_max = up_distances.values().max().copied().unwrap();
    // let down_distances_max = down_distances.values().max().copied().unwrap();
    // let left_distances_max = left_distances.values().max().copied().unwrap();
    // let right_distances_max = right_distances.values().max().copied().unwrap();
    // let up_left_distances_max = up_left_distances.values().max().copied().unwrap();
    // let up_right_distances_max = up_right_distances.values().max().copied().unwrap();
    // let down_left_distances_max = down_left_distances.values().max().copied().unwrap();
    // let down_right_distances_max = down_right_distances.values().max().copied().unwrap();

    // println!();
    // dbg!(start_distances_max);
    // dbg!(up_distances_max);
    // dbg!(down_distances_max);
    // dbg!(left_distances_max);
    // dbg!(right_distances_max);
    // dbg!(up_left_distances_max);
    // dbg!(up_right_distances_max);
    // dbg!(down_left_distances_max);
    // dbg!(down_right_distances_max);
    // println!();

    let x = ((ACTUAL_STEPS - width / 2) / width) as isize;
    let y = ((ACTUAL_STEPS - height / 2) / height) as isize;

    dbg!(y);
    dbg!(x);

    // dbg!(x);
    // println!("dist to {y}: {}", distance_to_subgrid(height, width, y, 0));
    // println!("left over: {}", ACTUAL_STEPS - distance_to_subgrid(height, width, y, 0));

    let (gy, gx) = (0, x);
    // dbg!((gy, gx));
    // let distance_to_gy_gx = distance_to_subgrid(height, width, gy, gx);
    // dbg!(distance_to_gy_gx);
    // let left = 26501365usize.saturating_sub(distance_to_gy_gx);
    // dbg!(left);

    let hmmm = distance_to_subgrid(height, width, 2, x - 1);
    dbg!(hmmm);
    let hmmm = distance_to_subgrid(height, width, y, -1);

    // for (gy, gx) in [(0, x), (1, x), (1, x - 1), (2, x - 1), (2, x - 2)] {
    // for (gy, gx) in [(0, x), (-1, x), (-1, x - 1), (-2, x - 1), (-2, x - 2)] {
    // for (gy, gx) in [(0, -x), (-1, -x), (-1, -x + 1), (-2, -x + 1), (-2, -x + 2)] {
    // for (gy, gx) in [(0, x), (1, x), (1, x - 1), (2, x - 1), (2, x - 2)] {
    // for (gy, gx) in [(-y, 0), (-y, 1), (-y + 1, 1), (-y + 1, 2), (-y + 2, 2)] {
    // for (gy, gx) in [(-y, 0), (-y, -1), (-y + 1, -1), (-y + 1, -2), (-y + 2, -2)] {
    // for (gy, gx) in [(0, x), (-1, x), (-1, x - 1), (-2, x - 1), (-2, x - 2)] {
    //     println!("({gy}, {gx})");
    //     let distance_to_here = distance_to_subgrid(height, width, gy, gx);
    //     let dists: Distances = distances_for_subgrid(&grid, gy, gx)
    //         .into_iter()
    //         .filter(|&(_, dist)| {
    //             let final_dist = dist as usize + distance_to_here;
    //             let parity = Parity::for_subgrid(gy, gx);
    //             final_dist <= ACTUAL_STEPS && parity.matches(final_dist)
    //         })
    //         .collect();
    //     dbg!(dists.len());
    //     // debug(&grid, dists);
    //     println!();
    // }

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

    let diagonal_length = y as usize
        + 1 // include both caps
        - 2; // exclude both caps
    let small_diag_segments = y as usize;
    let big_diag_segments = small_diag_segments - 1;

    dbg!(diagonal_length, big_diag_segments, small_diag_segments);

    let caps = up_cap + down_cap + left_cap + right_cap;
    dbg!(caps);
    let big_diagonals = big_diag_segments
        * (up_left_big_diag + up_right_big_diag + down_left_big_diag + down_right_big_diag); // TODO: This is not quite right and off by x2.
    // THEre might be an even/odd problem
    let small_diagonals = small_diag_segments
        * (up_left_small_diag + up_right_small_diag + down_left_small_diag + down_right_small_diag); // this is correct but off by x2.
    let diagonals = big_diagonals + small_diagonals;
    dbg!(small_diagonals, big_diagonals, diagonals);

    let inner_odd = num_reachable_in_subgrid(&grid, 0, 0);
    dbg!(inner_odd);
    let inner_even = num_reachable_in_subgrid(&grid, 0, 1);
    dbg!(inner_even);

    println!();
    let mut inner_total = inner_odd;
    let mut ring = 1;

    // while ring <= 202_300 {
    while ring < 202_300 {
        let per_subgrid = match ring % 2 == 0 {
            true => inner_odd,
            false => inner_even,
        };

        inner_total += ring * 4 * per_subgrid;

        ring += 1;
    }

    // Account for caps
    // inner_total -= 4 * inner_odd;

    // dbg!(num_reachable_in_subgrid(&grid, 2, 0));
    dbg!(inner_total);

    // 630661863252816 = correct (ape)
    // 630661863455116 = correct (jonathan + 2 other corroboraters)
    // 630658739740792 = too low  ( < 202,300)
    // 630664964885620 = too high (<= 202,300)
    dbg!(caps + diagonals + inner_total);
    let part2 = caps + diagonals + inner_total;
    println!("part2 = {part2}");

    // debug(&grid, reachable_in_subgrid(&grid, 1, 1));

    // debug(&grid, reachable_in_subgrid(&grid, 0, -x));
    // debug(&grid, reachable_in_subgrid(&grid, 1, -x + 1));
    dbg!(distance_to_subgrid(height, width, 0, -x));
    dbg!(distance_to_subgrid(height, width, 1, -x));
    dbg!(distance_to_subgrid(height, width, 1, -x + 1));
    dbg!(26501365 - distance_to_subgrid(height, width, 0, -x));
    dbg!(26501365 - distance_to_subgrid(height, width, 1, -x));
    dbg!(26501365 - distance_to_subgrid(height, width, 1, -x + 1));

    // These are all right
    // - [x] up cap = 5817
    // - [x] bottom cap = 5816
    // - [x] right cap = 5844
    // - [x] left cap = 5789

    // These are not :(
    // - [x] up-left small diag = 939
    // - [x] up-left big diag = 6749
    // - [x] up-right small diag = 959
    // - [x] up-right big diag = 6785
    // - [x] down-right small diag = 968
    // - [x] down-right big diag = 6776
    // - [x] down-left small diag = 931
    // - [x] down-left big diag = 6757

    // debug(&grid, dists);

    // let part2: u64 = seen.into_values().sum();
    // println!("part2 = {part2}");
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

// #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
// struct SubGrid {
//     y: isize,
//     x: isize,
// }

// impl SubGrid {
//     fn neighbors(&self) -> [Self; 4] {
//         [
//             // Up
//             SubGrid {
//                 y: self.y - 1,
//                 ..*self
//             },
//             // Down
//             SubGrid {
//                 y: self.y + 1,
//                 ..*self
//             },
//             // Left
//             SubGrid {
//                 x: self.x - 1,
//                 ..*self
//             },
//             // Right
//             SubGrid {
//                 x: self.x + 1,
//                 ..*self
//             },
//         ]
//     }

//     fn left(&self) -> Self {
//         SubGrid {
//             x: self.x - 1,
//             ..*self
//         }
//     }

//     fn full_neighbors(&self) -> [Self; 8] {
//         [
//             // Up
//             SubGrid {
//                 y: self.y - 1,
//                 ..*self
//             },
//             // Up left
//             SubGrid {
//                 y: self.y - 1,
//                 x: self.x - 1,
//             },
//             // Up right
//             SubGrid {
//                 y: self.y - 1,
//                 x: self.x + 1,
//             },
//             // Left
//             SubGrid {
//                 x: self.x - 1,
//                 ..*self
//             },
//             // Right
//             SubGrid {
//                 x: self.x + 1,
//                 ..*self
//             },
//             // Down left
//             SubGrid {
//                 y: self.y + 1,
//                 x: self.x - 1,
//             },
//             // Down
//             SubGrid {
//                 y: self.y + 1,
//                 ..*self
//             },
//             // Down right
//             SubGrid {
//                 y: self.y + 1,
//                 x: self.x + 1,
//             },
//         ]
//     }
// }

// #[derive(Debug, Clone, Copy, PartialEq, Eq)]
// enum Parity {
//     Even,
//     Odd,
// }

// impl Parity {
//     /// Returns whether an even or odd number of steps into this subgrid
//     /// will yield a tile that we can end up on.
//     fn for_subgrid(y: isize, x: isize) -> Self {
//         match (y + x) % 2 == 0 {
//             false => Self::Even,
//             true => Self::Odd,
//         }
//     }

//     // fn matches(&self, num: u32) -> bool {
//     fn matches(&self, num: usize) -> bool {
//         match (*self, num % 2 == 0) {
//             (Parity::Even, true) => true,
//             (Parity::Even, false) => false,
//             (Parity::Odd, true) => false,
//             (Parity::Odd, false) => true,
//         }
//     }

//     fn flip(&mut self) {
//         match self {
//             Self::Even => *self = Self::Odd,
//             Self::Odd => *self = Self::Even,
//         }
//     }
// }

fn debug(grid: &Grid, reachable_tiles: Distances) {
    for (y, row) in grid.iter().enumerate() {
        for (x, tile) in row.iter().enumerate() {
            let point = Point { y, x };
            if reachable_tiles.contains_key(&point) {
                print!(".");
            } else {
                match tile {
                    Tile::Garden => print!(" "),
                    Tile::Rock => print!("#"),
                }
            }
        }
        println!();
    }
}

fn reachable_in_subgrid(grid: &Grid, gy: isize, gx: isize) -> Distances {
    let height = grid.len();
    let width = grid[0].len();
    let distance_to_here = distance_to_subgrid(height, width, gy, gx);

    distances_for_subgrid(grid, gy, gx)
        .into_iter()
        .filter(|&(_, dist)| {
            let final_dist = dist as usize + distance_to_here;
            final_dist <= ACTUAL_STEPS && final_dist % 2 == 1
            // let parity = Parity::for_subgrid(gy, gx);
            // final_dist <= ACTUAL_STEPS && parity.matches(final_dist)
        })
        .collect()
}

// TODO: remove `reachable_in_subgrid` and just `count` without `collect`ing
fn num_reachable_in_subgrid(grid: &Grid, gy: isize, gx: isize) -> usize {
    reachable_in_subgrid(grid, gy, gx).len()
}
