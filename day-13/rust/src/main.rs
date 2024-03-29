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

    fn as_char(&self) -> char {
        match self {
            Tile::Ash => '.',
            Tile::Rock => '#',
        }
    }
}

#[allow(dead_code)]
fn debug_tiles(tiles: &[Tile]) {
    println!("{}", tiles.iter().map(Tile::as_char).collect::<String>());
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

    fn vertical_reflection_smudge(&self) -> Option<usize> {
        let columns = self.columns();
        let old_vertical_reflection = self.vertical_reflection();

        for column in 1..self.width() {
            if old_vertical_reflection == Some(column) {
                continue;
            }

            let mut left = column - 1;
            let mut right = column;
            let mut smudge_fixed = false;

            loop {
                if columns[right] != columns[left] {
                    if !smudge_fixed && one_off(&columns[right], &columns[left]) {
                        smudge_fixed = true;
                    } else {
                        break;
                    }
                }

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

    fn horizontal_reflection_smudge(&self) -> Option<usize> {
        let old_horizontal_reflection = self.horizontal_reflection();

        for row in 1..self.grid.len() {
            if old_horizontal_reflection == Some(row) {
                continue;
            }

            let mut upper = row - 1;
            let mut lower = row;
            let mut smudge_fixed = false;

            loop {
                if self.grid[lower] != self.grid[upper] {
                    if !smudge_fixed && one_off(&self.grid[lower], &self.grid[upper]) {
                        smudge_fixed = true;
                    } else {
                        break;
                    }
                }

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

    fn summarize_smudge(&self) -> usize {
        self.vertical_reflection_smudge()
            .unwrap_or_else(|| self.horizontal_reflection_smudge().unwrap() * 100)
    }
}

fn main() {
    // let input = include_str!("../../test_input.txt");
    let input = include_str!("../../input.txt");

    let patterns: Vec<_> = input.split("\n\n").map(Pattern::parse).collect();

    let part1: usize = patterns.iter().map(|pattern| pattern.summarize()).sum();
    println!("part1 = {part1}");

    let part2: usize = patterns.iter().map(Pattern::summarize_smudge).sum();
    println!("part2 = {part2}");
}

fn one_off(a: &[Tile], b: &[Tile]) -> bool {
    a.iter().zip(b.iter()).filter(|(a, b)| a != b).count() == 1
}
