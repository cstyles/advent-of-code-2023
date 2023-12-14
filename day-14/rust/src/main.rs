#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Tile {
    Empty,
    Round,
    Cubed,
}

impl Tile {
    fn parse(c: char) -> Self {
        match c {
            '.' => Self::Empty,
            'O' => Self::Round,
            '#' => Self::Cubed,
            _ => unreachable!("bad input: {c}"),
        }
    }

    fn as_char(&self) -> char {
        match self {
            Self::Empty => '.',
            Self::Round => 'O',
            Self::Cubed => '#',
        }
    }
}

type Grid = Vec<Vec<Tile>>;

fn tilt_north(grid: &mut Grid) {
    loop {
        let mut rocks_moved = false;

        for y in 0..grid.len() {
            for x in 0..grid[0].len() {
                let tile = grid[y][x];

                if tile != Tile::Round {
                    continue;
                };

                let Some(up_y) = y.checked_sub(1) else {
                    continue;
                };

                let tile_above = grid[up_y][x];
                if tile_above == Tile::Empty {
                    grid[up_y][x] = Tile::Round;
                    grid[y][x] = Tile::Empty;
                    rocks_moved = true;
                }
            }
        }

        if !rocks_moved {
            break;
        }
    }
}

fn load(grid: &Grid) -> usize {
    grid.iter()
        .rev()
        .enumerate()
        .map(|(i, row)| (i, row.iter().filter(|tile| **tile == Tile::Round).count()))
        .map(|(i, count)| (i + 1) * count)
        .sum()
}

fn main() {
    // let input = include_str!("../../test_input.txt");
    let input = include_str!("../../input.txt");

    let mut grid: Vec<Vec<Tile>> = input
        .lines()
        .map(|line| line.chars().map(Tile::parse).collect())
        .collect();

    debug(&grid);
    println!();
    tilt_north(&mut grid);
    debug(&grid);

    println!("part1 = {}", load(&grid));
}

#[allow(dead_code)]
fn debug(grid: &Grid) {
    for row in grid {
        let row = row.iter().map(Tile::as_char).collect::<String>();
        println!("{row}");
    }
}
