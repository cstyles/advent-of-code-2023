#[derive(Debug, Copy, Clone, PartialEq, Eq)]
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
        write!(f, "{row}")
    }
}

impl Row {
    fn parse(line: &str) -> Self {
        let (springs, groups) = line.split_once(' ').unwrap();
        let springs = springs.chars().map(Status::parse).collect();
        let groups = groups.split(',').map(|n| n.parse().unwrap()).collect();

        Self { springs, groups }
    }

    fn possible_arrangements(self) -> usize {
        let unknown = self
            .springs
            .iter()
            .enumerate()
            .find(|(_, status)| **status == Status::Unknown)
            .map(|(i, _)| i);

        match unknown {
            Some(position) => [Status::Operational, Status::Damaged]
                .into_iter()
                .map(|status| self.replace(position, status))
                .map(Row::possible_arrangements)
                .sum(),
            None => {
                if self.is_valid() {
                    1
                } else {
                    0
                }
            }
        }
    }

    fn is_valid(&self) -> bool {
        let mut group_sizes = vec![];
        let mut springs = self.springs.iter();
        let spring = springs.next().unwrap();
        let mut in_group = *spring == Status::Damaged;
        let mut group_size = usize::from(in_group);

        for spring in springs {
            match spring {
                Status::Operational => {
                    if in_group {
                        group_sizes.push(group_size);
                    }

                    in_group = false;
                    group_size = 0;
                }
                Status::Damaged => {
                    in_group = true;
                    group_size += 1;
                }
                Status::Unknown => todo!(),
            }
        }

        if in_group {
            group_sizes.push(group_size);
        }

        if group_sizes.len() != self.groups.len() {
            return false;
        }

        group_sizes
            .into_iter()
            .zip(self.groups.iter())
            .all(|(a, b)| a == *b)
    }

    fn replace(&self, position: usize, status: Status) -> Self {
        let mut this = self.clone();
        this.springs[position] = status;
        this
    }
}

fn main() {
    // let input = include_str!("../../test_input.txt");
    let input = include_str!("../../input.txt");

    let rows: Vec<_> = input.lines().map(Row::parse).collect();
    let part1: usize = rows.into_iter().map(Row::possible_arrangements).sum();
    println!("part1 = {part1}");
}
