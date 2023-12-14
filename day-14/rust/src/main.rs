use std::collections::HashMap;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
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

                if let Some(tile_above) = y
                    .checked_sub(1)
                    .map(|y| &mut grid[y][x])
                    .filter(|tile| **tile == Tile::Empty)
                {
                    *tile_above = Tile::Round;
                    grid[y][x] = Tile::Empty;
                    rocks_moved = true;
                };
            }
        }

        if !rocks_moved {
            break;
        }
    }
}

fn tilt_south(grid: &mut Grid) {
    loop {
        let mut rocks_moved = false;

        for y in (0..grid.len()).rev() {
            for x in (0..grid[0].len()).rev() {
                let tile = grid[y][x];

                if tile != Tile::Round {
                    continue;
                };

                if let Some(tile_below) = grid
                    .get_mut(y + 1)
                    .map(|row| &mut row[x])
                    .filter(|tile| **tile == Tile::Empty)
                {
                    *tile_below = Tile::Round;
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

fn tilt_west(grid: &mut Grid) {
    loop {
        let mut rocks_moved = false;

        for x in 0..grid[0].len() {
            for row in grid.iter_mut() {
                let tile = row[x];

                if tile != Tile::Round {
                    continue;
                };

                if let Some(left_tile) = x
                    .checked_sub(1)
                    .map(|x| &mut row[x])
                    .filter(|tile| **tile == Tile::Empty)
                {
                    *left_tile = Tile::Round;
                    row[x] = Tile::Empty;
                    rocks_moved = true;
                }
            }
        }

        if !rocks_moved {
            break;
        }
    }
}

fn tilt_east(grid: &mut Grid) {
    loop {
        let mut rocks_moved = false;

        for x in 0..grid[0].len() {
            for row in grid.iter_mut() {
                let tile = row[x];

                if tile != Tile::Round {
                    continue;
                };

                if let Some(right_tile) = row.get_mut(x + 1).filter(|tile| **tile == Tile::Empty) {
                    *right_tile = Tile::Round;
                    row[x] = Tile::Empty;
                    rocks_moved = true;
                }
            }
        }

        if !rocks_moved {
            break;
        }
    }
}

fn load(grid: Grid) -> usize {
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

    let mut part1_grid = grid.clone();
    tilt_north(&mut part1_grid);
    println!("part1 = {}", load(part1_grid));

    let mut seen: HashMap<Grid, usize> = [(grid.clone(), 0)].into();
    let mut current_cycle = 0;

    let first_cycle_of_loop = loop {
        current_cycle += 1;

        tilt_north(&mut grid);
        tilt_west(&mut grid);
        tilt_south(&mut grid);
        tilt_east(&mut grid);

        if let Some(prev_round) = seen.insert(grid.clone(), current_cycle) {
            break prev_round;
        }
    };

    let loop_length = current_cycle - first_cycle_of_loop;
    let how_many_loops = (1_000_000_000 - current_cycle) / loop_length;
    let leftover_cycles = 1_000_000_000 - current_cycle - loop_length * how_many_loops;

    for _ in 0..leftover_cycles {
        tilt_north(&mut grid);
        tilt_west(&mut grid);
        tilt_south(&mut grid);
        tilt_east(&mut grid);
    }

    println!("part1 = {}", load(grid));
}

#[allow(dead_code)]
fn debug(grid: &Grid) {
    for row in grid {
        let row = row.iter().map(Tile::as_char).collect::<String>();
        println!("{row}");
    }
}
