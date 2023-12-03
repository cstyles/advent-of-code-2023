mod point;
use point::Point;

use std::collections::HashSet;

#[derive(Debug, Copy, Clone)]
enum SymbolType {
    Slash,
    Star,
    At,
    Dollar,
    Pound,
    Equal,
    Plus,
    Percent,
    And,
    Dash,
}

#[derive(Debug, Copy, Clone)]
struct Symbol {
    stype: SymbolType,
    point: Point,
}

impl Symbol {
    fn new(c: char, point: Point) -> Option<Self> {
        let stype = match c {
            '/' => SymbolType::Slash,
            '*' => SymbolType::Star,
            '@' => SymbolType::At,
            '$' => SymbolType::Dollar,
            '#' => SymbolType::Pound,
            '=' => SymbolType::Equal,
            '+' => SymbolType::Plus,
            '%' => SymbolType::Percent,
            '&' => SymbolType::And,
            '-' => SymbolType::Dash,
            '.' | '0'..='9' => return None,
            _ => unreachable!("bad input: {c}"),
        };

        Some(Self { stype, point })
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct Number {
    point: Point, // left-most point
    value: u32,
}

// const SIZE: usize = 10;
// const INPUT: &str = include_str!("../../test_input.txt");
const SIZE: usize = 140;
const INPUT: &str = include_str!("../../input.txt");

fn main() {
    // Addressable map
    let grid: Vec<Vec<char>> = INPUT
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect();

    let mut symbols = vec![];
    for (y, line) in INPUT.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            let point = Point { y, x };
            symbols.extend(Symbol::new(c, point));
        }
    }

    let numbers: HashSet<Number> = symbols
        .iter()
        .flat_map(|symbol| symbol.point.neighbors::<SIZE>())
        .filter_map(|neighbor| find_number(&grid, neighbor))
        .collect();

    let part1: u32 = numbers.into_iter().map(|n| n.value).sum();
    println!("part1 = {part1}");
}

/// Panics if out of bounds
fn lookup_unchecked(grid: &[Vec<char>], y: usize, x: usize) -> char {
    grid[y][x]
}

fn lookup(grid: &[Vec<char>], y: usize, x: usize) -> Option<char> {
    grid.get(y).and_then(|row| row.get(x)).copied()
}

fn lookup_point(grid: &[Vec<char>], Point { y, x }: Point) -> Option<char> {
    lookup(grid, y, x)
}

/// Tries to parse a number at (or near) a coordinate
fn find_number(grid: &[Vec<char>], point: Point) -> Option<Number> {
    let digit = lookup_point(grid, point)?;
    let digit = digit.to_digit(10)?;

    let mut digits = vec![digit];

    // Try parsing to the right
    // TODO: iterator-ify!
    let mut going_right = point;
    while let Some(digit) = going_right
        .right::<SIZE>()
        .and_then(|p| lookup_point(grid, p))
        .and_then(|c| c.to_digit(10))
    {
        digits.push(digit);
        going_right = going_right.right::<SIZE>().unwrap();
    }

    // Try parsing to the left
    let mut going_left = point;
    while let Some(digit) = going_left
        .left::<SIZE>()
        .and_then(|p| lookup_point(grid, p))
        .and_then(|c| c.to_digit(10))
    {
        digits.insert(0, digit);
        going_left = going_left.left::<SIZE>().unwrap(); // TODO
    }

    Some(Number {
        point: going_left,
        value: digits_to_number(digits),
    })
}

fn digits_to_number(digits: Vec<u32>) -> u32 {
    digits.into_iter().fold(0, |acc, elm| acc * 10 + elm)
}
