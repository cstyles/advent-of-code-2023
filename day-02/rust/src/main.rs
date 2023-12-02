#[derive(Debug, Copy, Clone)]
struct Round {
    red: u32,
    green: u32,
    blue: u32,
}

impl Round {
    fn parse(string: &str) -> Self {
        let mut red = 0;
        let mut green = 0;
        let mut blue = 0;

        for pull in string.split(", ") {
            let (count, color) = pull.split_once(' ').unwrap();
            let count = count.parse().unwrap();

            match color {
                "red" => red = count,
                "green" => green = count,
                "blue" => blue = count,
                _ => unreachable!("bad input"),
            }
        }

        Self { red, green, blue }
    }

    fn possible(&self) -> bool {
        self.red <= 12 && self.green <= 13 && self.blue <= 14
    }

    fn power(self) -> u32 {
        self.red * self.green * self.blue
    }
}

#[derive(Debug, Clone)]
struct Game {
    id: u32,
    rounds: Vec<Round>,
}

impl Game {
    fn parse(line: &str) -> Self {
        let line = &line[5..]; // skip "Game "
        let (id, line) = line.split_once(": ").unwrap();

        let id = id.parse().unwrap();
        let rounds = line.split("; ").map(Round::parse).collect();

        Self { id, rounds }
    }

    fn possible(&self) -> bool {
        self.rounds.iter().all(|round| round.possible())
    }

    fn fewest_cubes_possible(self) -> Round {
        Round {
            red: self.rounds.iter().map(|r| r.red).max().unwrap(),
            green: self.rounds.iter().map(|r| r.green).max().unwrap(),
            blue: self.rounds.iter().map(|r| r.blue).max().unwrap(),
        }
    }
}

fn main() {
    // let input = include_str!("../../test_input.txt");
    let input = include_str!("../../input.txt");

    let part1: u32 = input
        .lines()
        .map(Game::parse)
        .filter(Game::possible)
        .map(|game| game.id)
        .sum();

    println!("part1 = {part1}");

    let part2: u32 = input
        .lines()
        .map(Game::parse)
        .map(Game::fewest_cubes_possible)
        .map(Round::power)
        .sum();

    println!("part2 = {part2}");
}
