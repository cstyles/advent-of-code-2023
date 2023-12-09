fn main() {
    // let input = include_str!("../../test_input.txt");
    let input = include_str!("../../input.txt");

    let part1: i64 = input.lines().map(part1).sum();
    println!("part1 = {part1}");

    let part2: i64 = input.lines().map(part2).sum();
    println!("part2 = {part2}");
}

fn part1(line: &str) -> i64 {
    let numbers: Vec<i64> = line.split(' ').map(|n| n.parse().unwrap()).collect();

    solve(numbers)
}

fn solve(row: Vec<i64>) -> i64 {
    let next_row: Vec<_> = row.windows(2).map(|nums| nums[1] - nums[0]).collect();

    if next_row.iter().all(|num| *num == 0) {
        *row.last().unwrap()
    } else {
        row.last().unwrap() + solve(next_row)
    }
}

fn part2(line: &str) -> i64 {
    let numbers: Vec<i64> = line.split(' ').map(|n| n.parse().unwrap()).collect();

    solve2(numbers)
}

fn solve2(row: Vec<i64>) -> i64 {
    let next_row: Vec<_> = row.windows(2).map(|nums| nums[1] - nums[0]).collect();

    if next_row.iter().all(|num| *num == 0) {
        *row.first().unwrap()
    } else {
        row.first().unwrap() - solve2(next_row)
    }
}
