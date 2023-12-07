use std::{cmp::Ordering, collections::HashMap, ops::ControlFlow};

#[derive(Debug, Copy, Clone)]
enum Type {
    FiveOfAKind,
    FourOfAKind,
    FullHouse,
    ThreeOfAKind,
    TwoPair,
    OnePair,
    HighCard,
}

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
struct Card(u32);

const JOKER: Card = Card(1);

impl Card {
    fn parse(c: char) -> Self {
        Card(match c {
            'J' => 1,
            '2' => 2,
            '3' => 3,
            '4' => 4,
            '5' => 5,
            '6' => 6,
            '7' => 7,
            '8' => 8,
            '9' => 9,
            'T' => 10,
            'Q' => 12,
            'K' => 13,
            'A' => 14,
            _ => unreachable!("bad input: {c}"),
        })
    }
}

impl std::fmt::Display for Card {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self.0 {
                1 => 'J',
                2 => '2',
                3 => '3',
                4 => '4',
                5 => '5',
                6 => '6',
                7 => '7',
                8 => '8',
                9 => '9',
                10 => 'T',
                12 => 'Q',
                13 => 'K',
                14 => 'A',
                _ => unreachable!(),
            }
        )
    }
}

impl std::fmt::Debug for Card {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(&self, f)
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
struct Hand {
    cards: [Card; 5],
    bid: u32,
}

impl Hand {
    fn parse(string: &str) -> Self {
        let (hand, bid) = string.split_once(' ').unwrap();
        let bid = bid.parse().unwrap();
        let mut chars = hand.chars();

        let hand = [
            Card::parse(chars.next().unwrap()),
            Card::parse(chars.next().unwrap()),
            Card::parse(chars.next().unwrap()),
            Card::parse(chars.next().unwrap()),
            Card::parse(chars.next().unwrap()),
        ];

        Self { cards: hand, bid }
    }

    fn type_(&self) -> Type {
        let map = self.best().cards.into_iter().fold(
            HashMap::<Card, u32>::with_capacity(13),
            |mut map, card| {
                map.entry(card).and_modify(|count| *count += 1).or_insert(1);
                map
            },
        );

        if map.values().any(|count| *count == 5) {
            Type::FiveOfAKind
        } else if map.values().any(|count| *count == 4) {
            Type::FourOfAKind
        } else if map.values().any(|count| *count == 3) {
            if map.values().any(|count| *count == 2) {
                Type::FullHouse
            } else {
                Type::ThreeOfAKind
            }
        } else {
            match map.values().filter(|count| **count == 2).count() {
                2 => Type::TwoPair,
                1 => Type::OnePair,
                _ => Type::HighCard,
            }
        }
    }

    fn break_tie(&self, other: &Self) -> Ordering {
        match self
            .cards
            .into_iter()
            .zip(other.cards)
            .try_fold((), |_, (left, right)| match left.0.cmp(&right.0) {
                Ordering::Equal => ControlFlow::Continue(()),
                Ordering::Greater => ControlFlow::Break(TieBreak::Left),
                Ordering::Less => ControlFlow::Break(TieBreak::Right),
            }) {
            ControlFlow::Continue(()) => Ordering::Equal,
            ControlFlow::Break(TieBreak::Left) => Ordering::Greater,
            ControlFlow::Break(TieBreak::Right) => Ordering::Less,
        }
    }

    #[allow(dead_code)]
    fn break_tie2(&self, other: &Self) -> Ordering {
        self.cards[0].0.cmp(&other.cards[0].0).then(
            self.cards[1].0.cmp(&other.cards[1].0).then(
                self.cards[2].0.cmp(&other.cards[2].0).then(
                    self.cards[3]
                        .0
                        .cmp(&other.cards[3].0)
                        .then(self.cards[4].0.cmp(&other.cards[4].0)),
                ),
            ),
        )
    }

    /// Returns the best possible version of a hand, made by substituting
    /// Jokers for other cards.
    fn best(self) -> Self {
        match self.cards.into_iter().position(|card| card == JOKER) {
            None => self,
            Some(position) => [
                Card(2),
                Card(3),
                Card(4),
                Card(5),
                Card(6),
                Card(7),
                Card(8),
                Card(9),
                Card(10),
                Card(12),
                Card(13),
                Card(14),
            ]
            .into_iter()
            .map(|card| self.replace(position, card))
            .map(Hand::best)
            .max()
            .unwrap(),
        }
    }

    fn replace(mut self, position: usize, with: Card) -> Self {
        self.cards[position] = with;
        self
    }
}

enum TieBreak {
    Left,
    Right,
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self.type_(), other.type_()) {
            (Type::FiveOfAKind, Type::FiveOfAKind)
            | (Type::FourOfAKind, Type::FourOfAKind)
            | (Type::FullHouse, Type::FullHouse)
            | (Type::ThreeOfAKind, Type::ThreeOfAKind)
            | (Type::TwoPair, Type::TwoPair)
            | (Type::OnePair, Type::OnePair)
            | (Type::HighCard, Type::HighCard) => self.break_tie(other),
            (Type::FiveOfAKind, _) => Ordering::Greater,
            (Type::FourOfAKind, Type::FiveOfAKind) => Ordering::Less,
            (Type::FourOfAKind, _) => Ordering::Greater,
            (Type::FullHouse, Type::FiveOfAKind | Type::FourOfAKind) => Ordering::Less,
            (Type::FullHouse, _) => Ordering::Greater,
            (Type::ThreeOfAKind, Type::FiveOfAKind | Type::FourOfAKind | Type::FullHouse) => {
                Ordering::Less
            }
            (Type::ThreeOfAKind, _) => Ordering::Greater,
            (Type::TwoPair, Type::HighCard | Type::OnePair) => Ordering::Greater,
            (Type::TwoPair, _) => Ordering::Less,
            (Type::OnePair, Type::HighCard) => Ordering::Greater,
            (Type::OnePair, _) => Ordering::Less,
            (Type::HighCard, _) => Ordering::Less,
        }
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn main() {
    // let input = include_str!("../../test_input.txt");
    let input = include_str!("../../input.txt");

    let mut hands: Vec<_> = input.lines().map(Hand::parse).collect();
    hands.sort();

    let part1: u32 = hands
        .into_iter()
        .enumerate()
        .map(|(rank, hand)| (rank as u32 + 1) * hand.bid)
        .sum();

    println!("part1 = {part1}");
}
