#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Tile {
    Ash,
    Rock,
}

impl Tile {
    fn parse(c: char) -> Self {
        match c {
            '.' => Self::Ash,
            '#' => Self::Rock,
            _ => unreachable!("bad tile: {c}"),
        }
    }

    // fn as_char(self) -> char
}

#[derive(Debug, Clone)]
struct Pattern {
    grid: Vec<Vec<Tile>>,
}

impl Pattern {
    fn parse(string: &str) -> Self {
        let grid = string
            .lines()
            .map(|line| line.chars().map(Tile::parse).collect())
            .collect();

        Self { grid }
    }

    fn width(&self) -> usize {
        self.grid.first().unwrap().len()
    }

    fn columns(&self) -> Vec<Vec<Tile>> {
        let mut columns = vec![];
        let height = self.grid.len();

        for x in 0..self.width() {
            columns.push((0..height).map(|y| self.grid[y][x]).collect());
        }

        columns
    }

    fn vertical_reflection(&self) -> Option<usize> {
        let columns = self.columns();

        for column in 1..self.width() {
            let mut left = column - 1;
            let mut right = column;

            while columns[right] == columns[left] {
                if left == 0 || right == self.width() - 1 {
                    return Some(column);
                } else {
                    left -= 1;
                    right += 1;
                }
            }
        }

        None
    }

    fn horizontal_reflection(&self) -> Option<usize> {
        for row in 1..self.grid.len() {
            let mut upper = row - 1;
            let mut lower = row;

            while self.grid[lower] == self.grid[upper] {
                if upper == 0 || lower == self.grid.len() - 1 {
                    return Some(row);
                } else {
                    upper -= 1;
                    lower += 1;
                }
            }
        }

        None
    }

    fn summarize(&self) -> usize {
        self.vertical_reflection()
            .unwrap_or_else(|| self.horizontal_reflection().unwrap() * 100)
    }
}

fn main() {
    // let input = include_str!("../../test_input.txt");
    let input = include_str!("../../input.txt");

    let patterns: Vec<_> = input.split("\n\n").map(Pattern::parse).collect();

    let part1: usize = patterns.iter().map(|pattern| pattern.summarize()).sum();
    println!("part1 = {part1}");
}
