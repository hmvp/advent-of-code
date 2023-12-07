use aoc::IterUnwrap;
use itertools::Itertools;
use nom::character::complete::{alphanumeric1, digit1, space1};
use nom::combinator::map;
use nom::sequence::separated_pair;

aoc::parts!(1, 2);

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug)]
enum Card {
    A,
    K,
    Q,
    J,
    T,
    C9,
    C8,
    C7,
    C6,
    C5,
    C4,
    C3,
    C2,
    Joker,
}

impl Card {
    fn from(c: char) -> Self {
        match c {
            'A' => Self::A,
            'K' => Self::K,
            'Q' => Self::Q,
            'J' => Self::J,
            'T' => Self::T,
            '9' => Self::C9,
            '8' => Self::C8,
            '7' => Self::C7,
            '6' => Self::C6,
            '5' => Self::C5,
            '4' => Self::C4,
            '3' => Self::C3,
            '2' => Self::C2,
            'Z' => Self::Joker,
            _ => unreachable!(),
        }
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug)]
enum HandType {
    FiveOfAKind,
    FourOfAKind,
    FullHouse,
    ThreeOfAKind,
    TwoPair,
    OnePair,
    HighCard,
}

impl HandType {
    fn from_cards(cards: &str) -> Self {
        let counts = cards.chars().counts();
        match (
            counts.get(&'Z').unwrap_or(&0),
            &counts.values().sorted().rev().collect_vec()[..],
        ) {
            (_, [5]) => Self::FiveOfAKind,
            (1, [4, 1]) => Self::FiveOfAKind,
            (4, [4, 1]) => Self::FiveOfAKind,
            (0, [4, 1]) => Self::FourOfAKind,
            (2, [3, 2]) => Self::FiveOfAKind,
            (3, [3, 2]) => Self::FiveOfAKind,
            (0, [3, 2]) => Self::FullHouse,
            (1, [3, 1, 1]) => Self::FourOfAKind,
            (3, [3, 1, 1]) => Self::FourOfAKind,
            (0, [3, 1, 1]) => Self::ThreeOfAKind,
            (2, [2, 2, 1]) => Self::FourOfAKind,
            (1, [2, 2, 1]) => Self::FullHouse,
            (0, [2, 2, 1]) => Self::TwoPair,
            (1, [2, 1, 1, 1]) => Self::ThreeOfAKind,
            (2, [2, 1, 1, 1]) => Self::ThreeOfAKind,
            (0, [2, 1, 1, 1]) => Self::OnePair,
            (1, [1, 1, 1, 1, 1]) => Self::OnePair,
            (0, [1, 1, 1, 1, 1]) => Self::HighCard,
            _ => unreachable!(),
        }
    }
}

#[derive(PartialEq, Eq, Ord, PartialOrd, Debug)]
struct Hand {
    hand_type: HandType,
    cards: [Card; 5],
    bid: usize,
}

impl Hand {
    fn from(cards: &str, bid: usize) -> Self {
        Hand {
            cards: cards.chars().map(Card::from).collect_n(),
            hand_type: HandType::from_cards(cards),
            bid,
        }
    }
}

fn parse_input(input: &str) -> Vec<Hand> {
    let mut parse_hand = separated_pair(
        alphanumeric1::<&str, ()>,
        space1,
        map(digit1, |s: &str| s.parse().unwrap()),
    );

    input
        .lines()
        .map(|line| {
            let (cards, bid) = parse_hand(line).unwrap().1;
            Hand::from(cards, bid)
        })
        .collect_vec()
}

fn calculate_hands(mut hands: Vec<Hand>) -> usize {
    hands.sort();

    hands
        .iter()
        .rev()
        .enumerate()
        .map(|(index, hand)| (index + 1) * hand.bid)
        .sum::<usize>()
}

fn part_1(input: aoc::Input) -> impl ToString {
    let hands = parse_input(input.raw());

    calculate_hands(hands)
}

fn part_2(input: aoc::Input) -> impl ToString {
    let input = input.raw().replace('J', "Z");

    let hands = parse_input(&input);

    calculate_hands(hands)
}
