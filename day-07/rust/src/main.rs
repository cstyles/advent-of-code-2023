use std::cmp::Ordering;
use std::collections::HashMap;
use std::ops::ControlFlow;

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
struct Card<const PART_TWO: bool>(u32);

impl<const PART_TWO: bool> Card<PART_TWO> {
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
            'J' => {
                if PART_TWO {
                    1
                } else {
                    11
                }
            }
            'Q' => 12,
            'K' => 13,
            'A' => 14,
            _ => unreachable!("bad input: {c}"),
        })
    }
}

#[derive(Copy, Clone, PartialEq, Eq)]
struct Hand<const PART_TWO: bool> {
    cards: Cards<PART_TWO>,
    bid: u32,
}

#[derive(Copy, Clone, PartialEq, Eq)]
struct Cards<const PART_TWO: bool>([Card<PART_TWO>; 5]);

impl<const PART_TWO: bool> Cards<PART_TWO> {
    fn type_(&self) -> Type {
        let map = self.0.into_iter().fold(
            HashMap::<Card<PART_TWO>, u32>::with_capacity(13),
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
                0 => Type::HighCard,
                _ => unreachable!("invalid type"),
            }
        }
    }

    fn best_type(&self) -> Type {
        let type_ = self.type_();
        let j_count = self
            .0
            .iter()
            .filter(|card| [Card(1), Card(11)].contains(*card))
            .count();

        if j_count == 0 {
            return type_;
        }

        match type_ {
            Type::FiveOfAKind => Type::FiveOfAKind,
            Type::FourOfAKind => Type::FiveOfAKind,
            Type::FullHouse => Type::FiveOfAKind,
            Type::ThreeOfAKind => Type::FourOfAKind,
            Type::TwoPair => match j_count {
                1 => Type::FullHouse,
                2 => Type::FourOfAKind,
                _ => unreachable!(),
            },
            Type::OnePair => Type::ThreeOfAKind,
            Type::HighCard => Type::OnePair,
        }
    }

    fn break_tie(&self, other: &Self) -> Ordering {
        match self
            .0
            .into_iter()
            .zip(other.0)
            .try_fold((), |_, (left, right)| match left.0.cmp(&right.0) {
                Ordering::Equal => ControlFlow::Continue(()),
                order => ControlFlow::Break(order),
            }) {
            ControlFlow::Continue(()) => Ordering::Equal,
            ControlFlow::Break(order) => order,
        }
    }
}

impl<const PART_TWO: bool> Ord for Cards<PART_TWO> {
    fn cmp(&self, other: &Self) -> Ordering {
        let (this, other) = match PART_TWO {
            false => (self.type_(), other.type_()),
            true => (self.best_type(), other.best_type()),
        };

        match (this, other) {
            (Type::FiveOfAKind, Type::FiveOfAKind)
            | (Type::FourOfAKind, Type::FourOfAKind)
            | (Type::FullHouse, Type::FullHouse)
            | (Type::ThreeOfAKind, Type::ThreeOfAKind)
            | (Type::TwoPair, Type::TwoPair)
            | (Type::OnePair, Type::OnePair)
            | (Type::HighCard, Type::HighCard) => Ordering::Equal,
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

impl<const PART_TWO: bool> PartialOrd for Cards<PART_TWO> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<const PART_TWO: bool> Hand<PART_TWO> {
    fn parse(string: &str) -> Self {
        let (hand, bid) = string.split_once(' ').unwrap();
        let bid = bid.parse().unwrap();
        let mut chars = hand.chars();

        let cards = Cards::<PART_TWO>([
            Card::parse(chars.next().unwrap()),
            Card::parse(chars.next().unwrap()),
            Card::parse(chars.next().unwrap()),
            Card::parse(chars.next().unwrap()),
            Card::parse(chars.next().unwrap()),
        ]);

        Self { cards, bid }
    }
}

impl<const PART_TWO: bool> Ord for Hand<PART_TWO> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.cards
            .cmp(&other.cards)
            .then_with(|| self.cards.break_tie(&other.cards))
    }
}

impl<const PART_TWO: bool> PartialOrd for Hand<PART_TWO> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn main() {
    part::<false>();
    part::<true>();
}

fn part<const PART_TWO: bool>() {
    // let input = include_str!("../../test_input.txt");
    let input = include_str!("../../input.txt");

    let mut hands: Vec<_> = input.lines().map(Hand::<PART_TWO>::parse).collect();
    hands.sort();

    let answer: u32 = hands
        .into_iter()
        .enumerate()
        .map(|(rank, hand)| (rank as u32 + 1) * hand.bid)
        .sum();

    let number = match PART_TWO {
        false => 1,
        true => 2,
    };

    println!("part{number} = {answer}");
}
