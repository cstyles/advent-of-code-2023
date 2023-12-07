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

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct Card(u32);

impl Card {
    fn parse(c: char) -> Self {
        Card(match c {
            '2' => 2,
            '3' => 3,
            '4' => 4,
            '5' => 5,
            '6' => 6,
            '7' => 7,
            '8' => 8,
            '9' => 9,
            'T' => 10,
            'J' => 11,
            'Q' => 12,
            'K' => 13,
            'A' => 14,
            _ => unreachable!("bad input: {c}"),
        })
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
struct Hand {
    hand: [Card; 5],
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

        Self { hand, bid }
    }

    fn type_(&self) -> Type {
        let map =
            self.hand
                .into_iter()
                .fold(HashMap::<Card, u32>::with_capacity(13), |mut map, card| {
                    map.entry(card).and_modify(|count| *count += 1).or_insert(1);
                    map
                });

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
            .hand
            .into_iter()
            .zip(other.hand)
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
        self.hand[0].0.cmp(&other.hand[0].0).then(
            self.hand[1].0.cmp(&other.hand[1].0).then(
                self.hand[2].0.cmp(&other.hand[2].0).then(
                    self.hand[3]
                        .0
                        .cmp(&other.hand[3].0)
                        .then(self.hand[4].0.cmp(&other.hand[4].0)),
                ),
            ),
        )
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
