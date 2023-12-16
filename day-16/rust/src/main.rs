use std::collections::HashSet;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Tile {
    Empty,
    Vertical,
    Horizontal,
    Slash,
    Backslash,
}

impl Tile {
    fn parse(c: char) -> Self {
        match c {
            '.' => Self::Empty,
            '|' => Self::Vertical,
            '-' => Self::Horizontal,
            '/' => Self::Slash,
            '\\' => Self::Backslash,
            _ => unreachable!("bad input: {c}"),
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct Beam {
    y: usize,
    x: usize,
    direction: Direction,
}

impl Beam {
    fn up(self) -> Option<Self> {
        self.y.checked_sub(1).map(|y| Self {
            y,
            direction: Direction::Up,
            ..self
        })
    }

    fn down(self, height: usize) -> Option<Self> {
        match self.y >= height - 1 {
            true => None,
            false => Some(Self {
                y: self.y + 1,
                direction: Direction::Down,
                ..self
            }),
        }
    }

    fn left(self) -> Option<Self> {
        self.x.checked_sub(1).map(|x| Self {
            x,
            direction: Direction::Left,
            ..self
        })
    }

    fn right(self, width: usize) -> Option<Self> {
        match self.x >= width - 1 {
            true => None,
            false => Some(Self {
                x: self.x + 1,
                direction: Direction::Right,
                ..self
            }),
        }
    }

    fn next(self, height: usize, width: usize) -> Option<Self> {
        match self.direction {
            Direction::Up => self.up(),
            Direction::Down => self.down(height),
            Direction::Left => self.left(),
            Direction::Right => self.right(width),
        }
    }
}

fn main() {
    // let input = include_str!("../../test_input.txt");
    let input = include_str!("../../input.txt");

    let map: Vec<Vec<Tile>> = input
        .lines()
        .map(|line| line.chars().map(Tile::parse).collect())
        .collect();

    let part1 = solve(
        &map,
        Beam {
            y: 0,
            x: 0,
            direction: Direction::Right,
        },
    );
    println!("part1 = {part1}");

    let height = map.len();
    let width = map[0].len();

    let mut part2 = 0;
    for x in 0..width {
        part2 = part2.max(solve(
            &map,
            Beam {
                y: 0,
                x,
                direction: Direction::Down,
            },
        ));
        part2 = part2.max(solve(
            &map,
            Beam {
                y: height - 1,
                x,
                direction: Direction::Up,
            },
        ));
    }
    for y in 0..height {
        part2 = part2.max(solve(
            &map,
            Beam {
                y,
                x: 0,
                direction: Direction::Right,
            },
        ));
        part2 = part2.max(solve(
            &map,
            Beam {
                y,
                x: width - 1,
                direction: Direction::Left,
            },
        ));
    }

    println!("part2 = {part2}");
}

fn solve(map: &Map, beam: Beam) -> usize {
    let mut beams = vec![beam];

    let height = map.len();
    let width = map[0].len();

    let mut seen: HashSet<Beam> = [].into();

    while let Some(beam) = beams.pop() {
        if !seen.insert(beam) {
            continue;
        }

        match (beam.direction, lookup(map, (beam.y, beam.x))) {
            (_, Tile::Empty)
            | (Direction::Up, Tile::Vertical)
            | (Direction::Down, Tile::Vertical)
            | (Direction::Left, Tile::Horizontal)
            | (Direction::Right, Tile::Horizontal) => beams.extend(beam.next(height, width)),

            (Direction::Up, Tile::Horizontal) | (Direction::Down, Tile::Horizontal) => {
                beams.extend([beam.right(width), beam.left()].into_iter().flatten())
            }

            (Direction::Left, Tile::Vertical) | (Direction::Right, Tile::Vertical) => {
                beams.extend([beam.up(), beam.down(height)].into_iter().flatten())
            }

            (Direction::Up, Tile::Slash) => beams.extend(beam.right(width)),
            (Direction::Up, Tile::Backslash) => beams.extend(beam.left()),
            (Direction::Down, Tile::Slash) => beams.extend(beam.left()),
            (Direction::Down, Tile::Backslash) => beams.extend(beam.right(width)),
            (Direction::Left, Tile::Slash) => beams.extend(beam.down(height)),
            (Direction::Left, Tile::Backslash) => beams.extend(beam.up()),
            (Direction::Right, Tile::Slash) => beams.extend(beam.up()),
            (Direction::Right, Tile::Backslash) => beams.extend(beam.down(height)),
        };
    }

    seen.into_iter()
        .map(|beam| (beam.y, beam.x))
        .collect::<HashSet<_>>()
        .len()
}

type Map = Vec<Vec<Tile>>;

fn lookup(map: &Map, (y, x): (usize, usize)) -> Tile {
    map[y][x]
}

#[allow(dead_code)]
fn debug_part1(map: &Map, part1: &HashSet<(usize, usize)>) {
    for (y, row) in map.iter().enumerate() {
        for (x, _tile) in row.iter().enumerate() {
            if part1.contains(&(y, x)) {
                print!("#");
            } else {
                print!(" ");
            }
        }
        println!();
    }
}
