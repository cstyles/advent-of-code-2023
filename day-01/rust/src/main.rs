fn extract_digits(string: &str) -> (u32, u32) {
    let mut chars = string.chars().filter_map(|c| c.to_digit(10));
    let a = chars.next().unwrap();
    let b = chars.last().unwrap_or(a);

    (a, b)
}

fn main() {
    let input = include_str!("../../input.txt");

    let part1: u32 = input
        .lines()
        .map(extract_digits)
        .map(|(a, b)| a * 10 + b)
        .sum();

    println!("part1 = {part1}");
}
