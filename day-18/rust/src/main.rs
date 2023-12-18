use std::collections::{HashMap, HashSet};

#[derive(Debug, Copy, Clone)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn parse(c: char) -> Self {
        match c {
            'U' => Self::Up,
            'D' => Self::Down,
            'L' => Self::Left,
            'R' => Self::Right,
            _ => unreachable!("bad input: {c}"),
        }
    }
}

#[derive(Debug, Copy, Clone)]
struct Instruction {
    direction: Direction,
    magnitude: u32,
    color: u32,
}

impl Instruction {
    fn parse(line: &str) -> Self {
        let direction = line.chars().next().unwrap();
        let direction = Direction::parse(direction);

        let (magnitude, rest) = &line[2..].split_once(' ').unwrap();
        let magnitude = magnitude.parse().unwrap();

        let color = &rest[2..8];
        let color = u32::from_str_radix(color, 16).unwrap();

        Self {
            direction,
            magnitude,
            color,
        }
    }

    fn trace(mut self, start: Point) -> impl Iterator<Item = Point> {
        let mut point = start;
        std::iter::from_fn(move || {
            if self.magnitude == 0 {
                None
            } else {
                self.magnitude -= 1;
                point = point.move_(self.direction);
                Some(point)
            }
        })
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct Point {
    y: isize,
    x: isize,
}

impl Point {
    fn move_(&self, direction: Direction) -> Self {
        match direction {
            Direction::Up => self.up(),
            Direction::Down => self.down(),
            Direction::Left => self.left(),
            Direction::Right => self.right(),
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

    fn neighbors(&self) -> [Self; 4] {
        [
            self.up(),
            self.down(),
            self.left(),
            self.right(),
        ]
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
enum Tile {
    Trench,
    Ground,
}

type Map = HashMap<Point, Tile>;

fn main() {
    // let input = include_str!("../../test_input.txt");
    let input = include_str!("../../input.txt");

    let instructions: Vec<Instruction> = input.lines().map(Instruction::parse).collect();

    let mut point = Point { y: 0, x: 0 };
    let mut map = Map::from([(point, Tile::Trench)]);

    for instruction in instructions {
        for new_point in instruction.trace(point) {
            map.insert(new_point, Tile::Trench);
            point = new_point;
        }
    }

    // cheating ;)
    // let inside_point = Point { y: 1, x: 1 }; // test
    let inside_point = Point { y: -1, x: -1 }; // real
    let mut seen: HashSet<Point> = map.keys().copied().collect();
    let mut stack = vec![inside_point];

    while let Some(p) = stack.pop() {
        if !seen.insert(p) {
            continue;
        }

        for neighbor in p.neighbors() {
            stack.push(neighbor);
        }
    }

    let part1 = seen.len();
    println!("part1 = {part1}");
}

// fn debug(map: &Map) {
//     let min_y = map.keys().map(|p| p.y).min().unwrap();
//     let min_x = map.keys().map(|p| p.x).min().unwrap();

//     let shifted: Map = map
//         .iter()
//         .map(|(point, tile)| {
//             (
//                 Point {
//                     y: point.y - min_y,
//                     x: point.x - min_x,
//                 },
//                 *tile,
//             )
//         })
//         .collect();
// }
