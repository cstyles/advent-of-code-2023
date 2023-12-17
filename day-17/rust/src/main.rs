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

    // 1173 = too high
    // let part1 = part1(&grid);
    // println!("part1 = {part1}");

    // 1295 = too low
    let part2 = part2(&grid);
    println!("part2 = {part2}");
}

fn part1(grid: &Grid) -> u32 {
    let start = HeapItem {
        point: Point { y: 0, x: 0 },
        direction: Direction::Right,
        moves_remaining: 3,
        total_distance: 0,
    };

    let destination = Point {
        y: grid.len() - 1,
        x: grid[0].len() - 1,
    };

    // let mut best_so_far = 1_173; // naive upper bound
    let mut best_so_far = u32::MAX; // naive upper bound
    let mut shortest_distances: HashMap<(Point, Direction, u8), u32> = [].into();
    // let mut shortest_distances: HashMap<Point, u32> =
    // [(start.for_map(), 0)].into();
    let mut heap: BinaryHeap<HeapItem> = [start].into();

    while let Some(heap_item) = heap.pop() {
        // if queue_item.point == destination {
        //     println!("at destination: {}", queue_item.total_distance);
        // }

        // std::thread::sleep(std::time::Duration::from_millis(10));

        if heap_item.total_distance >= best_so_far {
            continue;
        }

        // println!(
        //     "(({}, {}), {:>9?}, {:?}) => {}",
        //     heap_item.point.y,
        //     heap_item.point.x,
        //     heap_item.direction,
        //     heap_item.moves_remaining,
        //     heap_item.total_distance
        // );
        // std::thread::sleep(std::time::Duration::from_millis(10));

        if heap_item.point == destination {
            // println!("reached destination");
            best_so_far = best_so_far.min(heap_item.total_distance);
        }

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

        // let shortest_distance_seen = shortest_distances.get_mut(&heap_item.for_map()).unwrap();
        // .entry(heap_item.for_map())
        // .or_insert(heap_item.total_distance);

        // match heap_item.total_distance.cmp(shortest_distance_seen) {
        //     Ordering::Less => *shortest_distance_seen = heap_item.total_distance,
        //     Ordering::Equal => continue,
        //     Ordering::Greater => continue
        // };

        // if heap_item.total_distance >= *shortest_distance_seen {
        //     continue;
        // } else {
        //     println!(
        //         "overwriting {} with {}",
        //         *shortest_distance_seen, heap_item.total_distance
        //     );
        //     std::thread::sleep(std::time::Duration::from_millis(1));
        //     *shortest_distance_seen = heap_item.total_distance;
        // }

        heap.extend(heap_item.possible_moves(grid));
    }

    best_so_far
}

fn part2(grid: &Grid) -> u32 {
    let start = HeapItem2 {
        point: Point { y: 0, x: 0 },
        direction: Direction::Right,
        moves_remaining: 10,
        min_moves_remaining: 0,
        total_distance: 0,
    };

    let destination = Point {
        y: grid.len() - 1,
        x: grid[0].len() - 1,
    };

    let mut best_so_far = u32::MAX;
    let mut shortest_distances: HashMap<(Point, Direction, u8, u8), u32> = [].into();
    let mut heap: BinaryHeap<HeapItem2> = [start].into();

    while let Some(heap_item) = heap.pop() {
        // println!(
        //     "(({}, {}), {:?}, {}, {}) => {}",
        //     heap_item.point.y,
        //     heap_item.point.x,
        //     heap_item.direction,
        //     heap_item.moves_remaining,
        //     heap_item.min_moves_remaining,
        //     heap_item.total_distance
        // );
        // std::thread::sleep(std::time::Duration::from_millis(10));

        if heap_item.total_distance >= best_so_far {
            continue;
        }

        if heap_item.point == destination && heap_item.min_moves_remaining == 0 {
            println!("reached destination");
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

        heap.extend(heap_item.possible_moves(grid));
    }

    best_so_far
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
        // fn for_map(&self) -> Point {
        // self.point
    }

    fn possible_moves(&self, grid: &Grid) -> Vec<Self> {
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
                    // println!("at ({}, {}), can't go {:?} anymore", self.point.y, self.point.x, direction);
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
                moves.extend(self.point.in_direction(direction, grid).map(|point| Self {
                    point,
                    direction,
                    moves_remaining: 2,
                    total_distance: self.total_distance + point.lookup(grid),
                }));
            }
        }

        moves
    }
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
struct HeapItem2 {
    point: Point,
    direction: Direction,
    moves_remaining: u8,
    min_moves_remaining: u8,
    total_distance: u32,
}

impl Ord for HeapItem2 {
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

impl PartialOrd for HeapItem2 {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl HeapItem2 {
    fn for_map(&self) -> (Point, Direction, u8, u8) {
        (
            self.point,
            self.direction,
            self.moves_remaining,
            self.min_moves_remaining,
        )
    }

    fn possible_moves(&self, grid: &Grid) -> Vec<Self> {
        if self.min_moves_remaining > 0 {
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
                moves.extend(self.point.in_direction(direction, grid).map(|point| Self {
                    point,
                    direction,
                    moves_remaining: 9,
                    min_moves_remaining: 3,
                    total_distance: self.total_distance + point.lookup(grid),
                }));
            }
        }

        moves
    }
}
