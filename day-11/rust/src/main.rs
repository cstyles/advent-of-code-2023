use std::collections::HashSet;
use std::iter::repeat;
use std::ops::Range;

fn main() {
    // let input = include_str!("../../test_input.txt");
    let input = include_str!("../../input.txt");

    let map: Vec<Vec<char>> = input.lines().map(|row| row.chars().collect()).collect();

    let galaxies: Vec<(usize, usize)> = map
        .iter()
        .enumerate()
        .flat_map(|(y, row)| repeat(y).zip(row.iter().enumerate()))
        .filter(|(_, (_, c))| **c == '#')
        .map(|(y, (x, _))| (y, x))
        .collect();

    let empty_rows: HashSet<usize> = map
        .iter()
        .enumerate()
        .filter(|(_, row)| is_empty(*row))
        .map(|(y, _)| y)
        .collect();

    let empty_columns: HashSet<usize> = columns(&map)
        .enumerate()
        .filter(|(_, column)| is_empty(column))
        .map(|(y, _)| y)
        .collect();

    let mut part1 = 0;
    let mut part2 = 0;
    for (i, &galaxy_a) in galaxies.iter().enumerate() {
        for &galaxy_b in galaxies.iter().skip(i + 1) {
            let naive_distance = manhattan_distance(galaxy_a, galaxy_b);

            let double_rows = empty_rows
                .iter()
                .filter(|r| range(galaxy_a.0, galaxy_b.0).contains(*r))
                .count();

            let double_columns = empty_columns
                .iter()
                .filter(|r| range(galaxy_a.1, galaxy_b.1).contains(*r))
                .count();

            part1 += naive_distance + double_rows + double_columns;
            part2 += naive_distance + 999_999 * (double_rows + double_columns);
        }
    }

    println!("part1 = {part1}");
    println!("part2 = {part2}");
}

fn columns(grid: &[Vec<char>]) -> impl Iterator<Item = Vec<char>> + '_ {
    let mut x = 0;
    let row_length = grid[0].len();

    std::iter::from_fn(move || match x >= row_length {
        true => None,
        false => {
            let column = grid.iter().map(|row| row[x]).collect();
            x += 1;
            Some(column)
        }
    })
}

fn is_empty<'a, I: IntoIterator<Item = &'a char>>(iter: I) -> bool {
    iter.into_iter().all(|c| *c == '.')
}

fn manhattan_distance((a_y, a_x): (usize, usize), (b_y, b_x): (usize, usize)) -> usize {
    b_y.abs_diff(a_y) + b_x.abs_diff(a_x)
}

fn range(a: usize, b: usize) -> Range<usize> {
    match a <= b {
        true => a..b,
        false => b..a,
    }
}

#[allow(dead_code)]
fn debug_point((y, x): (usize, usize)) {
    println!("({y}, {x})");
}
