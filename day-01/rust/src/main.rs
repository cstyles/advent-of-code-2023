fn extract_digits(string: &str) -> (u32, u32) {
    let mut chars = string.chars().filter_map(|c| c.to_digit(10));
    let a = chars.next().unwrap();
    let b = chars.last().unwrap_or(a);

    (a, b)
}

fn take2(string: &str) -> (u32, u32) {
    let chars: Vec<char> = string.chars().chain("xxxx".chars()).collect();

    let mut numbers = vec![];
    for word in chars.windows(5) {
        let word: [char; 5] = word.try_into().unwrap();
        let num = match word {
            ['1', ..] => 1,
            ['2', ..] => 2,
            ['3', ..] => 3,
            ['4', ..] => 4,
            ['5', ..] => 5,
            ['6', ..] => 6,
            ['7', ..] => 7,
            ['8', ..] => 8,
            ['9', ..] => 9,
            ['o', 'n', 'e', ..] => 1,
            ['t', 'w', 'o', ..] => 2,
            ['t', 'h', 'r', 'e', 'e', ..] => 3,
            ['f', 'o', 'u', 'r', ..] => 4,
            ['f', 'i', 'v', 'e', ..] => 5,
            ['s', 'i', 'x', ..] => 6,
            ['s', 'e', 'v', 'e', 'n'] => 7,
            ['e', 'i', 'g', 'h', 't'] => 8,
            ['n', 'i', 'n', 'e', ..] => 9,
            _ => continue,
        };

        numbers.push(num);
    }

    (
        numbers.first().copied().unwrap(),
        numbers.last().copied().unwrap(),
    )
}

fn main() {
    let input = include_str!("../../input.txt");

    let part1: u32 = input
        .lines()
        .map(extract_digits)
        .map(|(a, b)| a * 10 + b)
        .sum();

    println!("part1 = {part1}");

    let part2: u32 = input.lines().map(take2).map(|(a, b)| a * 10 + b).sum();
    println!("part2 = {part2}");
}
