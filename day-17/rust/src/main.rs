use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};

type Grid = Vec<Vec<u32>>;

fn main() {
    // let input = include_str!("../../test_input.txt");
    // let input = include_str!("../../test_input2.txt");
    let input = include_str!("../../input.txt");

    let grid: Grid = input
        .lines()
        .map(|line| line.chars().map(|n| n.to_digit(10).unwrap()).collect())
        .collect();

    let part1 = solve::<false>(&grid);
    println!("part1 = {part1}");

    let part2 = solve::<true>(&grid);
    println!("part2 = {part2}");
}

fn solve<const PART_TWO: bool>(grid: &Grid) -> u32 {
    let (moves_remaining, min_moves_remaining) = if PART_TWO { (10, 0) } else { (3, 0) };

    let start = HeapItem {
        point: Point { y: 0, x: 0 },
        direction: Direction::Right,
        moves_remaining,
        min_moves_remaining,
        total_distance: 0,
    };

    let destination = Point {
        y: grid.len() - 1,
        x: grid[0].len() - 1,
    };

    let mut best_so_far = u32::MAX;
    let mut shortest_distances: HashMap<(Point, Direction, u8, u8), u32> = [].into();
    let mut heap: BinaryHeap<HeapItem> = [start].into();

    while let Some(heap_item) = heap.pop() {
        if heap_item.total_distance >= best_so_far {
            continue;
        }

        if heap_item.point == destination && heap_item.min_moves_remaining == 0 {
            best_so_far = best_so_far.min(heap_item.total_distance);
        }

        // TODO: entry API?
        match shortest_distances.get_mut(&heap_item.for_map()) {
            Some(shortest_distance) => {
                if heap_item.total_distance >= *shortest_distance {
                    continue;
                } else {
                    *shortest_distance = heap_item.total_distance;
                }
            }
            None => {
                shortest_distances.insert(heap_item.for_map(), heap_item.total_distance);
            }
        };

        heap.extend(heap_item.possible_moves::<PART_TWO>(grid));
    }

    best_so_far
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
struct Point {
    y: usize,
    x: usize,
}

impl Point {
    fn up(&self) -> Option<Self> {
        self.y.checked_sub(1).map(|y| Self { y, ..*self })
    }

    fn down(&self, height: usize) -> Option<Self> {
        match self.y >= height - 1 {
            true => None,
            false => Some(Self {
                y: self.y + 1,
                ..*self
            }),
        }
    }

    fn left(&self) -> Option<Self> {
        self.x.checked_sub(1).map(|x| Self { x, ..*self })
    }

    fn right(&self, width: usize) -> Option<Self> {
        match self.x >= width - 1 {
            true => None,
            false => Some(Self {
                x: self.x + 1,
                ..*self
            }),
        }
    }

    fn in_direction(&self, direction: Direction, grid: &Grid) -> Option<Self> {
        match direction {
            Direction::Up => self.up(),
            Direction::Down => self.down(grid.len()),
            Direction::Left => self.left(),
            Direction::Right => self.right(grid[0].len()),
        }
    }

    fn lookup(&self, grid: &Grid) -> u32 {
        grid[self.y][self.x]
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
struct HeapItem {
    point: Point,
    direction: Direction,
    moves_remaining: u8,
    min_moves_remaining: u8,
    total_distance: u32,
}

impl Ord for HeapItem {
    fn cmp(&self, other: &Self) -> Ordering {
        self.total_distance
            .cmp(&other.total_distance)
            .reverse()
            .then_with(|| self.moves_remaining.cmp(&other.moves_remaining).reverse())
            .then_with(|| {
                self.min_moves_remaining
                    .cmp(&other.min_moves_remaining)
                    .reverse()
            })
            .then_with(|| self.point.cmp(&other.point))
            .then_with(|| self.direction.cmp(&other.direction))
    }
}

impl PartialOrd for HeapItem {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl HeapItem {
    fn for_map(&self) -> (Point, Direction, u8, u8) {
        (
            self.point,
            self.direction,
            self.moves_remaining,
            self.min_moves_remaining,
        )
    }

    fn possible_moves<const PART_TWO: bool>(&self, grid: &Grid) -> Vec<Self> {
        if PART_TWO && self.min_moves_remaining > 0 {
            return self
                .point
                .in_direction(self.direction, grid)
                .map(|point| Self {
                    point,
                    direction: self.direction,
                    moves_remaining: self.moves_remaining - 1,
                    min_moves_remaining: self.min_moves_remaining - 1,
                    total_distance: self.total_distance + point.lookup(grid),
                })
                .into_iter()
                .collect();
        }

        let mut moves = vec![];

        let directions = match self.direction {
            Direction::Up => [Direction::Up, Direction::Left, Direction::Right],
            Direction::Down => [Direction::Down, Direction::Left, Direction::Right],
            Direction::Left => [Direction::Down, Direction::Up, Direction::Left],
            Direction::Right => [Direction::Down, Direction::Right, Direction::Up],
        };

        for direction in directions {
            if direction == self.direction {
                if self.moves_remaining == 0 {
                    continue;
                } else {
                    moves.extend(self.point.in_direction(direction, grid).map(|point| Self {
                        point,
                        direction,
                        moves_remaining: self.moves_remaining - 1,
                        min_moves_remaining: 0,
                        total_distance: self.total_distance + point.lookup(grid),
                    }));
                }
            } else {
                let (moves_remaining, min_moves_remaining) = if PART_TWO { (9, 3) } else { (2, 0) };

                moves.extend(self.point.in_direction(direction, grid).map(|point| Self {
                    point,
                    direction,
                    moves_remaining,
                    min_moves_remaining,
                    total_distance: self.total_distance + point.lookup(grid),
                }));
            }
        }

        moves
    }
}
