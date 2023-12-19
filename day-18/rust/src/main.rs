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
struct Instruction<const PART_TWO: bool> {
    direction: Direction,
    magnitude: u32,
}

impl Instruction<false> {
    fn parse(line: &str) -> Self {
        let direction = line.chars().next().unwrap();
        let direction = Direction::parse(direction);

        let (magnitude, _) = &line[2..].split_once(' ').unwrap();
        let magnitude = magnitude.parse().unwrap();

        Self {
            direction,
            magnitude,
        }
    }
}

impl Instruction<true> {
    fn parse(line: &str) -> Self {
        let hex = &line[line.len() - 7..line.len() - 1];

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
    fn move_by<const PART_TWO: bool>(&self, instruction: Instruction<PART_TWO>) -> Self {
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

fn main() {
    // let input = include_str!("../../test_input.txt");
    let input = include_str!("../../input.txt");

    let part1_instructions: Vec<_> = input.lines().map(Instruction::<false>::parse).collect();
    solve::<false>(part1_instructions);

    let part2_instructions: Vec<_> = input.lines().map(Instruction::<true>::parse).collect();
    solve::<true>(part2_instructions);
}

fn solve<const PART_TWO: bool>(instructions: Vec<Instruction<PART_TWO>>) {
    let mut point = Point { y: 0, x: 0 };
    let mut vertices = vec![point];
    let mut perimeter = 0;

    for instruction in instructions {
        point = point.move_by(instruction);
        vertices.push(point);
        perimeter += instruction.magnitude;
    }

    let mut area = 0;
    for points in vertices.windows(2) {
        let [a, b] = points else { panic!() };

        // Shoelace?
        area += (a.y + b.y) * (a.x - b.x);
        // area += (a.x * b.y) - (b.x * a.y);
    }

    // Pick's Theorem?
    let part2 = area as u64 / 2 + perimeter as u64 / 2 + 1;
    println!("part2 = {part2}");
}
