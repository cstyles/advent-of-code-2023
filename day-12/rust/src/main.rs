use std::collections::HashMap;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
enum Status {
    Operational,
    Damaged,
    Unknown,
}

impl Status {
    fn parse(c: char) -> Self {
        match c {
            '.' => Self::Operational,
            '#' => Self::Damaged,
            '?' => Self::Unknown,
            _ => unreachable!("bad input: {c}"),
        }
    }

    fn as_char(&self) -> char {
        match self {
            Status::Operational => '.',
            Status::Damaged => '#',
            Status::Unknown => '?',
        }
    }
}

#[derive(Clone)]
struct Row {
    springs: Vec<Status>,
    groups: Vec<usize>,
}

impl std::fmt::Debug for Row {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let row: String = self.springs.iter().map(Status::as_char).collect();
        write!(f, "{row} {:?}", self.groups)
    }
}

impl Row {
    fn parse(line: &str) -> Self {
        let (springs, groups) = line.split_once(' ').unwrap();
        let springs = springs.chars().map(Status::parse).collect();
        let groups = groups.split(',').map(|n| n.parse().unwrap()).collect();

        Self { springs, groups }
    }

    fn replace(&self, position: usize, status: Status) -> Self {
        self.clone().replace_owned(position, status)
    }

    fn replace_owned(mut self, position: usize, status: Status) -> Self {
        self.springs[position] = status;
        self
    }

    fn unfold(self) -> Self {
        let springs_length = self.springs.len();
        let mut springs = self.springs;
        springs.push(Status::Unknown);

        let mut springs: Vec<Status> = springs
            .into_iter()
            .cycle()
            .take(5 * (springs_length + 1))
            .collect();
        let _ = springs.pop();

        let groups_length = self.groups.len();
        let groups = self
            .groups
            .into_iter()
            .cycle()
            .take(5 * groups_length)
            .collect();

        Self { springs, groups }
    }

    fn possible_arrangements(mut self, cache: &mut Cache) -> u64 {
        if let Some(num) = cache.get(&(self.springs.clone(), self.groups.clone())) {
            return *num;
        }

        match (self.springs.first(), self.groups.first().copied()) {
            (None, None) => 1,
            (None, Some(_)) => 0,               // invalid
            (Some(Status::Damaged), None) => 0, // invalid
            (Some(Status::Unknown), None) => self.trim_and_continue(cache),
            (Some(Status::Operational), _) => self.trim_and_continue(cache),
            (Some(Status::Damaged), Some(mut group_size)) => {
                let mut slice = self.springs.as_slice();

                loop {
                    match (slice, group_size) {
                        ([], 0) => return u64::from(self.groups.len() == 1), // done
                        ([], _) => return 0,                                 // invalid
                        ([Status::Damaged, _rest @ ..], 0) => return 0,
                        ([Status::Damaged, rest @ ..], _) => {
                            slice = rest;
                            group_size -= 1;
                            continue;
                        }
                        ([Status::Operational, rest @ ..], 0) => {
                            let new_springs = Vec::from(rest);
                            let _ = std::mem::replace(&mut self.springs, new_springs);
                            self.groups.remove(0);
                            return self.possible_arrangements(cache);
                        }
                        ([Status::Operational, _rest @ ..], _) => return 0, // invalid
                        ([Status::Unknown, rest @ ..], 0) => {
                            let new_springs = Vec::from(rest);
                            let _ = std::mem::replace(&mut self.springs, new_springs);
                            self.groups.remove(0);
                            return self.possible_arrangements(cache);
                        }
                        ([Status::Unknown, rest @ ..], _) => {
                            slice = rest;
                            group_size -= 1;
                            continue;
                        }
                    };
                }
            }
            (Some(Status::Unknown), Some(_)) => {
                let new_row = self.replace(0, Status::Operational);
                let new_springs = new_row.springs.clone();
                let try_operational = new_row.possible_arrangements(cache);
                cache.insert((new_springs, self.groups.clone()), try_operational);

                let new_row = self.replace_owned(0, Status::Damaged);
                let groups = new_row.groups.clone();
                let new_springs = new_row.springs.clone();
                let try_damaged = new_row.possible_arrangements(cache);
                cache.insert((new_springs, groups), try_damaged);

                try_operational + try_damaged
            }
        }
    }

    fn trim_and_continue(mut self, cache: &mut Cache) -> u64 {
        self.springs.remove(0);
        self.possible_arrangements(cache)
    }
}

type Cache = HashMap<(Vec<Status>, Vec<usize>), u64>;

fn main() {
    // let input = include_str!("../../test_input.txt");
    let input = include_str!("../../input.txt");

    let rows: Vec<_> = input.lines().map(Row::parse).collect();

    let mut cache = HashMap::<(Vec<Status>, Vec<usize>), u64>::new();
    let part1: u64 = rows
        .clone()
        .into_iter()
        .map(|row| row.possible_arrangements(&mut cache))
        .sum();
    println!("part1 = {part1}");

    let part2: u64 = rows
        .into_iter()
        .map(|row| row.unfold().possible_arrangements(&mut cache))
        .sum();
    println!("part2 = {part2}");
}
