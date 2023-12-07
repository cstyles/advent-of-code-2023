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
        match self.winning_count() {
            0 => 0,
            winning_numbers => 2u32.pow(winning_numbers - 1),
        }
    }

    fn winning_count(&self) -> u32 {
        self.have
            .iter()
            .filter(|have| self.winning.contains(have))
            .count() as u32
    }
}

fn main() {
    // let input = include_str!("../../test_input.txt");
    let input = include_str!("../../input.txt");

    let cards: Vec<Card> = input.lines().map(Card::parse).collect();
    let part1: u32 = cards.iter().map(Card::value).sum();
    println!("part1 = {part1}");

    let counts = vec![1; cards.len()];
    let counts = cards.into_iter().fold(counts, |counts, card| {
        let winning_count = card.winning_count();
        let count = counts[card.id as usize - 1];

        (1..=winning_count).fold(counts, |counts, id| {
            let id = (id + card.id - 1) as usize;
            let count = count + counts[id];
            replace(counts, id, count)
        })
    });

    let part2: u32 = counts.into_iter().sum();
    println!("part2 = {part2}");
}

fn replace<T>(mut v: Vec<T>, position: usize, item: T) -> Vec<T> {
    v[position] = item;
    v
}
