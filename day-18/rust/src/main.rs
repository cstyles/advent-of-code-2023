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

    fn from_hex(c: char) -> Self {
        match c {
            '0' => Self::Right,
            '1' => Self::Down,
            '2' => Self::Left,
            '3' => Self::Up,
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

#[derive(Debug, Copy, Clone)]
struct RealInstruction {
    direction: Direction,
    magnitude: u32,
}

impl RealInstruction {
    fn parse(line: &str) -> Self {
        let (_, hex) = line.split_once('#').unwrap();
        let hex = &hex[..hex.len() - 1];

        let direction = hex.chars().last().unwrap();
        let direction = Direction::from_hex(direction);

        let magnitude = &hex[..5];
        let magnitude = u32::from_str_radix(magnitude, 16).unwrap();

        Self {
            direction,
            magnitude,
        }
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
        [self.up(), self.down(), self.left(), self.right()]
    }

    // TODO: remove me
    fn move_by1(&self, instruction: Instruction) -> Self {
        match instruction.direction {
            Direction::Up => Self {
                y: self.y - instruction.magnitude as isize,
                ..*self
            },
            Direction::Down => Self {
                y: self.y + instruction.magnitude as isize,
                ..*self
            },
            Direction::Left => Self {
                x: self.x - instruction.magnitude as isize,
                ..*self
            },
            Direction::Right => Self {
                x: self.x + instruction.magnitude as isize,
                ..*self
            },
        }
    }

    fn move_by(&self, instruction: RealInstruction) -> Self {
        match instruction.direction {
            Direction::Up => Self {
                y: self.y - instruction.magnitude as isize,
                ..*self
            },
            Direction::Down => Self {
                y: self.y + instruction.magnitude as isize,
                ..*self
            },
            Direction::Left => Self {
                x: self.x - instruction.magnitude as isize,
                ..*self
            },
            Direction::Right => Self {
                x: self.x + instruction.magnitude as isize,
                ..*self
            },
        }
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

    part1(input);
    part2(input);
}

fn part1(input: &str) {
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

fn part2(input: &str) {
    // let instructions: Vec<Instruction> = input.lines().map(Instruction::parse).collect();
    let instructions: Vec<RealInstruction> = input.lines().map(RealInstruction::parse).collect();

    let mut point = Point { y: 0, x: 0};
    let mut vertices = vec![point];
    let mut perimeter = 0;

    for instruction in instructions {
        // point = point.move_by1(instruction);
        point = point.move_by(instruction);
        vertices.push(point);
        perimeter += instruction.magnitude;
    }

    // dbg!(&vertices);

    let mut area = 0;
    for points in vertices.windows(2) {
        let [a, b] = points else { panic!() };

        area += (a.y + b.y) * (a.x - b.x);
        // area += (a.x * b.y) - (b.x * a.y);
    }

    // 48020794727661 = too low
    let part2 = area as u64 / 2 + perimeter as u64 / 2 + 1;
    println!("part2 = {part2}");
}
