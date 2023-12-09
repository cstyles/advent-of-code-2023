fn main() {
    // let input = include_str!("../../test_input.txt");
    let input = include_str!("../../input.txt");

    let (part1, part2) = input
        .lines()
        .map(parse_row)
        .map(|numbers| (solve(&numbers), solve2(&numbers)))
        .fold((0, 0), sum2);

    println!("part1 = {part1}");
    println!("part2 = {part2}");
}

fn solve(row: &[i64]) -> i64 {
    let next_row: Vec<_> = row.windows(2).map(|nums| nums[1] - nums[0]).collect();

    if next_row.iter().all(|num| *num == 0) {
        *row.last().unwrap()
    } else {
        row.last().unwrap() + solve(&next_row)
    }
}

fn solve2(row: &[i64]) -> i64 {
    let next_row: Vec<_> = row.windows(2).map(|nums| nums[1] - nums[0]).collect();

    if next_row.iter().all(|num| *num == 0) {
        *row.first().unwrap()
    } else {
        row.first().unwrap() - solve2(&next_row)
    }
}

fn parse_row(line: &str) -> Vec<i64> {
    line.split(' ').map(|num| num.parse().unwrap()).collect()
}

fn sum2((a, b): (i64, i64), (c, d): (i64, i64)) -> (i64, i64) {
    (a + c, b + d)
}
