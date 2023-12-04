#[derive(Debug)]
struct Card {
    id: u32,
    winning: Vec<u32>,
    have: Vec<u32>,
}

impl Card {
    fn parse(line: &str) -> Self {
        let line = &line[5..]; // Trim "Card "
        let line = line.trim_start();
        let (id, line) = line.split_once(": ").unwrap();
        let id = id.parse().unwrap();
        let (winning, have) = line.split_once(" | ").unwrap();

        let winning = winning
            .split_whitespace()
            .map(|num| num.parse().unwrap())
            .collect();

        let have = have
            .split_whitespace()
            .map(|num| num.parse().unwrap())
            .collect();

        Self { id, winning, have }
    }

    fn value(&self) -> u32 {
        let winning_numbers = self
            .have
            .iter()
            .filter(|have| self.winning.contains(have))
            .count();

        match winning_numbers as u32 {
            0 => 0,
            winning_numbers => 2u32.pow(winning_numbers - 1),
        }
    }
}

fn main() {
    // let input = include_str!("../../test_input.txt");
    let input = include_str!("../../input.txt");

    let cards: Vec<Card> = input.lines().map(Card::parse).collect();
    let part1: u32 = cards.iter().map(Card::value).sum();
    println!("part1 = {part1}");
}
