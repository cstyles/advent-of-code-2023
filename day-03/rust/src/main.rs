mod point;
use point::Point;

use std::collections::{HashSet, VecDeque};
use std::iter::{repeat, successors};

#[derive(Debug, Copy, Clone)]
struct Symbol {
    character: char,
    point: Point,
}

impl Symbol {
    fn new(character: char, point: Point) -> Option<Self> {
        match character {
            '.' | '0'..='9' => None,
            _ => Some(Self { character, point }),
        }
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

    let symbols: Vec<Symbol> = INPUT
        .lines()
        .enumerate()
        .flat_map(|(y, line)| repeat(y).zip(line.chars().enumerate()))
        .filter_map(|(y, (x, c))| Symbol::new(c, Point { y, x }))
        .collect();

    let numbers: HashSet<Number> = symbols
        .iter()
        .flat_map(|symbol| symbol.point.neighbors::<SIZE>())
        .filter_map(|neighbor| find_number(&grid, neighbor))
        .collect();

    let part1: u32 = numbers.into_iter().map(|n| n.value).sum();
    println!("part1 = {part1}");

    let part2: u32 = symbols
        .into_iter()
        .filter(|s| s.character == '*')
        .filter_map(|s| gear_ratio(&grid, s.point))
        .sum();

    println!("part2 = {part2}");
}

/// Panics if out of bounds
fn lookup(grid: &[Vec<char>], Point { y, x }: Point) -> char {
    grid[y][x]
}

/// Tries to parse a number at (or near) a coordinate
fn find_number(grid: &[Vec<char>], mut point: Point) -> Option<Number> {
    let digit = lookup(grid, point).to_digit(10)?;
    let mut digits = VecDeque::from([digit]);

    // Try parsing to the right
    let digits_to_the_right = successors(point.right::<SIZE>(), Point::right::<SIZE>)
        .map_while(|point| lookup(grid, point).to_digit(10));
    digits.extend(digits_to_the_right);

    // Try parsing to the left
    for digit_to_the_left in successors(point.left::<SIZE>(), Point::left::<SIZE>)
        .map_while(|point| lookup(grid, point).to_digit(10))
    {
        digits.push_front(digit_to_the_left);
        point = point.left::<SIZE>().unwrap();
    }

    let value = digits_to_number(digits);
    Some(Number { point, value })
}

fn digits_to_number(digits: impl IntoIterator<Item = u32>) -> u32 {
    digits.into_iter().fold(0, |acc, elm| acc * 10 + elm)
}

fn gear_ratio(grid: &[Vec<char>], point: Point) -> Option<u32> {
    let neighbors: HashSet<Number> = point
        .neighbors::<SIZE>()
        .filter_map(|neighbor| find_number(grid, neighbor))
        .collect();

    if neighbors.len() == 2 {
        Some(neighbors.into_iter().map(|num| num.value).product())
    } else {
        None
    }
}
