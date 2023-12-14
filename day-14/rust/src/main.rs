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

fn tilt_south(grid: &mut Grid) {
    loop {
        let mut rocks_moved = false;

        for y in (0..grid.len()).rev() {
            for x in (0..grid[0].len()).rev() {
                let tile = grid[y][x];

                if tile != Tile::Round {
                    continue;
                };

                let down_y = y + 1;
                if down_y >= grid.len() {
                    continue;
                };

                let tile_below = grid[down_y][x];
                if tile_below == Tile::Empty {
                    grid[down_y][x] = Tile::Round;
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

                let Some(left_x) = x.checked_sub(1) else {
                    continue;
                };

                let left_tile = row[left_x];
                if left_tile == Tile::Empty {
                    row[left_x] = Tile::Round;
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
            for y in 0..grid.len() {
                let tile = grid[y][x];

                if tile != Tile::Round {
                    continue;
                };

                let right_x = x + 1;
                if right_x >= grid[0].len() {
                    continue;
                };

                let right_tile = grid[y][right_x];
                if right_tile == Tile::Empty {
                    grid[y][right_x] = Tile::Round;
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
    let mut cycle = 0;

    let first_cycle_of_loop = loop {
        cycle += 1;

        tilt_north(&mut grid);
        tilt_west(&mut grid);
        tilt_south(&mut grid);
        tilt_east(&mut grid);

        if let Some(prev_round) = seen.insert(grid.clone(), cycle) {
            break prev_round;
        }
    };

    let cycle_length = cycle - first_cycle_of_loop;
    let skip_factor = (1_000_000_000 - cycle) / cycle_length;
    let leftover_cycles = 1_000_000_000 - cycle - cycle_length * skip_factor;

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
