fn main() {
    // let input = include_str!("../../test_input.txt");
    let input = include_str!("../../input.txt");

    let (part1, part2) = input
        .lines()
        .map(parse_row)
        .map(|numbers| (solve::<false>(&numbers), solve::<true>(&numbers)))
        .fold((0, 0), sum2);

    println!("part1 = {part1}");
    println!("part2 = {part2}");
}

fn solve<const PART_TWO: bool>(row: &[i64]) -> i64 {
    let next_row: Vec<_> = row.windows(2).map(|nums| nums[1] - nums[0]).collect();

    match (PART_TWO, next_row.iter().all(|num| *num == 0)) {
        (false, false) => row.last().unwrap() + solve::<PART_TWO>(&next_row),
        (false, true) => *row.last().unwrap(),
        (true, false) => row.first().unwrap() - solve::<PART_TWO>(&next_row),
        (true, true) => *row.first().unwrap(),
    }
}

fn parse_row(line: &str) -> Vec<i64> {
    line.split(' ').map(|num| num.parse().unwrap()).collect()
}

fn sum2((a, b): (i64, i64), (c, d): (i64, i64)) -> (i64, i64) {
    (a + c, b + d)
}
