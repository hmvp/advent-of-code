use aoc::Input;
use nom::bytes::complete::tag;
use nom::character::complete::{alphanumeric1, digit1, space1};
use nom::combinator::map;
use nom::multi::many1;
use nom::sequence::{delimited, preceded, separated_pair, terminated};

aoc::parts!(1, 2);

struct Card {
    winning_numbers: Vec<u32>,
    card_numbers: Vec<u32>,
}

impl Card {
    fn cards(&self) -> u32 {
        u32::try_from(
            self.card_numbers
                .iter()
                .filter(|nr| self.winning_numbers.contains(nr))
                .count(),
        )
        .unwrap()
    }

    fn points(&self) -> u32 {
        2_u32.pow(self.cards() - 1)
    }
}

fn parse_input(input: Input) -> Vec<Card> {
    let parse_number1 = map(preceded(space1, digit1::<&str, ()>), |string: &str| {
        string.parse().unwrap()
    });
    let parse_number2 = map(preceded(space1, digit1::<&str, ()>), |string: &str| {
        string.parse().unwrap()
    });
    let mut parse_card = preceded(
        terminated(delimited(alphanumeric1, space1, digit1), tag(":")),
        map(
            separated_pair(many1(parse_number1), tag(" |"), many1(parse_number2)),
            |(winning_numbers, card_numbers)| Card {
                winning_numbers,
                card_numbers,
            },
        ),
    );

    input.lines().fold(Vec::new(), |mut acc, line| {
        acc.push(parse_card(line).expect(line).1);
        acc
    })
}

fn part_1(input: aoc::Input) -> impl ToString {
    parse_input(input).iter().map(Card::points).sum::<u32>()
}



fn process_list(list: &[u32]) -> u32 {
    let mut cards: Vec<u32> = std::iter::repeat(1u32).take(list.len()).collect();

    for (index, card) in list.iter().enumerate() {
        let multiplier = cards[index];

        for i in 0..*card {
            cards[index + 1 + i as usize] += multiplier;
        }
    }

    cards.iter().sum()
}

fn part_2(input: aoc::Input) -> impl ToString {
    process_list(&parse_input(input).iter().map(Card::cards).collect::<Vec<_>>())
}
