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

    let (part1, part2) = std::thread::scope(|scope| {
        let part1 = scope.spawn(|| solve::<false>(&grid));
        let part2 = scope.spawn(|| solve::<true>(&grid));

        (part1.join().unwrap(), part2.join().unwrap())
    });

    println!("part1 = {}", part1);
    println!("part2 = {}", part2);
}

fn solve<const PART_TWO: bool>(grid: &Grid) -> u32 {
    let moves_remaining = if PART_TWO { 10 } else { 3 };

    let start = HeapItem {
        point: Point { y: 0, x: 0 },
        direction: Direction::Right,
        moves_remaining,
        total_distance: 0,
    };

    let destination = Point {
        y: grid.len() as u8 - 1,
        x: grid[0].len() as u8 - 1,
    };

    let mut shortest_distances: HashMap<(Point, Direction, u8), u32> = [].into();
    let mut heap: BinaryHeap<HeapItem> = [start].into();

    // Allow us to start off part2 by moving downward.
    if PART_TWO {
        heap.push(HeapItem {
            moves_remaining: 0,
            ..start
        });
    }

    while let Some(heap_item) = heap.pop() {
        if heap_item.point == destination && (!PART_TWO || heap_item.moves_remaining <= 6) {
            return heap_item.total_distance;
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

    unreachable!("never reached the destination :(");
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
    y: u8,
    x: u8,
}

impl Point {
    fn up(&self) -> Option<Self> {
        self.y.checked_sub(1).map(|y| Self { y, ..*self })
    }

    fn down(&self, height: u8) -> Option<Self> {
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

    fn right(&self, width: u8) -> Option<Self> {
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
            Direction::Down => self.down(grid.len() as u8),
            Direction::Left => self.left(),
            Direction::Right => self.right(grid[0].len() as u8),
        }
    }

    fn lookup(&self, grid: &Grid) -> u32 {
        grid[self.y as usize][self.x as usize]
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
struct HeapItem {
    point: Point,
    direction: Direction,
    moves_remaining: u8,
    total_distance: u32,
}

impl Ord for HeapItem {
    fn cmp(&self, other: &Self) -> Ordering {
        self.total_distance
            .cmp(&other.total_distance)
            .reverse()
            .then_with(|| self.moves_remaining.cmp(&other.moves_remaining).reverse())
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
    fn for_map(&self) -> (Point, Direction, u8) {
        (self.point, self.direction, self.moves_remaining)
    }

    fn possible_moves<const PART_TWO: bool>(&self, grid: &Grid) -> Vec<Self> {
        if PART_TWO && self.moves_remaining > 6 {
            return self
                .point
                .in_direction(self.direction, grid)
                .map(|point| Self {
                    point,
                    direction: self.direction,
                    moves_remaining: self.moves_remaining - 1,
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
                        total_distance: self.total_distance + point.lookup(grid),
                    }));
                }
            } else {
                let moves_remaining = if PART_TWO { 9 } else { 2 };

                moves.extend(self.point.in_direction(direction, grid).map(|point| Self {
                    point,
                    direction,
                    moves_remaining,
                    total_distance: self.total_distance + point.lookup(grid),
                }));
            }
        }

        moves
    }
}
