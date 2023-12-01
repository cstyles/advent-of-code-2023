trait IteratorExt: Iterator {
    /// Returns the first and last items from an iterator.
    ///
    /// Returns the first item twice if there's only one item.
    fn first_and_last(self) -> Option<(Self::Item, Self::Item)>;
}

impl<T: Copy, I: Iterator<Item = T>> IteratorExt for I {
    fn first_and_last(mut self) -> Option<(Self::Item, Self::Item)> {
        self.next().map(|a| (a, self.last().unwrap_or(a)))
    }
}

fn extract_digits(string: &str) -> (u32, u32) {
    string
        .chars()
        .filter_map(|c| c.to_digit(10))
        .first_and_last()
        .unwrap()
}

/// An iterator that yields numbers from a string.
struct Numbers<'a> {
    chars: &'a [char],
}

impl<'a> Numbers<'a> {
    fn new(chars: &'a [char]) -> Self {
        Self { chars }
    }
}

impl Iterator for Numbers<'_> {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        while self.chars.len() >= 5 {
            let window = &self.chars[..5];
            self.chars = &self.chars[1..];

            match window {
                ['1', ..] => return Some(1),
                ['2', ..] => return Some(2),
                ['3', ..] => return Some(3),
                ['4', ..] => return Some(4),
                ['5', ..] => return Some(5),
                ['6', ..] => return Some(6),
                ['7', ..] => return Some(7),
                ['8', ..] => return Some(8),
                ['9', ..] => return Some(9),
                ['o', 'n', 'e', ..] => return Some(1),
                ['t', 'w', 'o', ..] => return Some(2),
                ['t', 'h', 'r', 'e', 'e'] => return Some(3),
                ['f', 'o', 'u', 'r', ..] => return Some(4),
                ['f', 'i', 'v', 'e', ..] => return Some(5),
                ['s', 'i', 'x', ..] => return Some(6),
                ['s', 'e', 'v', 'e', 'n'] => return Some(7),
                ['e', 'i', 'g', 'h', 't'] => return Some(8),
                ['n', 'i', 'n', 'e', ..] => return Some(9),
                _ => continue,
            }
        }

        None
    }
}

fn main() {
    let input = include_str!("../../input.txt");

    let part1: u32 = input
        .lines()
        .map(extract_digits)
        .map(|(a, b)| a * 10 + b)
        .sum();

    println!("part1 = {part1}");

    let part2: u32 = input
        .lines()
        .map(|line| line.chars().chain("xxxx".chars()).collect::<Vec<_>>())
        .map(|chars| Numbers::new(&chars).first_and_last().unwrap())
        .map(|(a, b)| a * 10 + b)
        .sum();

    println!("part2 = {part2}");
}
