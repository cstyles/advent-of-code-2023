fn main() {
    // let input = include_str!("../../test_input.txt");
    let input = include_str!("../../input.txt");
    let rows: Vec<_> = input.lines().map(parse_row).collect();

    let part1: i64 = rows.iter().map(AsRef::as_ref).map(solve).sum();
    let part2: i64 = rows.into_iter().map(reverse).map(|nums| solve(&nums)).sum();

    println!("part1 = {part1}");
    println!("part2 = {part2}");
}

fn solve(row: &[i64]) -> i64 {
    let next_row: Vec<_> = row.windows(2).map(|nums| nums[1] - nums[0]).collect();

    match next_row.iter().all(|num| *num == 0) {
        false => row.last().unwrap() + solve(&next_row),
        true => *row.last().unwrap(),
    }
}

fn parse_row(line: &str) -> Vec<i64> {
    line.split(' ').map(|num| num.parse().unwrap()).collect()
}

fn reverse<T>(mut v: Vec<T>) -> Vec<T> {
    v.reverse();
    v
}
