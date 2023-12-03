use aoc::Input;
use nom::branch::alt;
use nom::bytes::complete::{tag, take};
use nom::character::complete::{alpha1, digit1};
use nom::combinator::{map, map_parser, value};
use nom::multi::many1;
use nom::IResult;

aoc::parts!(1, 2);

fn part_1(input: Input) -> impl ToString {
    input.lines().fold(0, |acc, item| {
        let digits = item
            .chars()
            .filter(|&c| c.is_numeric())
            .map(|c| c.to_digit(10).unwrap())
            .collect::<Vec<_>>();
        acc + digits.first().unwrap() * 10 + digits.last().unwrap()
    })
}

fn first_number_parser(i: &str) -> IResult<&str, Vec<Option<usize>>> {
    let named_number = alt((
        value(Some(1), tag("one")),
        value(Some(2), tag("two")),
        value(Some(3), tag("three")),
        value(Some(4), tag("four")),
        value(Some(5), tag("five")),
        value(Some(6), tag("six")),
        value(Some(7), tag("seven")),
        value(Some(8), tag("eight")),
        value(Some(9), tag("nine")),
        map(map_parser(take(1u8), digit1), |c: &str| c.parse::<usize>().ok()),
        value(None, map_parser(take(1u8), alpha1)),
    ));

    many1(named_number)(i)
}

fn last_number_parser(i: &str) -> IResult<&str, Vec<Option<usize>>> {
    let named_number = alt((
        value(Some(1), tag("eno")),
        value(Some(2), tag("owt")),
        value(Some(3), tag("eerht")),
        value(Some(4), tag("ruof")),
        value(Some(5), tag("evif")),
        value(Some(6), tag("xis")),
        value(Some(7), tag("neves")),
        value(Some(8), tag("thgie")),
        value(Some(9), tag("enin")),
        map(map_parser(take(1u8), digit1), |c: &str| c.parse::<usize>().ok()),
        value(None, map_parser(take(1u8), alpha1)),
    ));

    many1(named_number)(i)
}

fn part_2(input: Input) -> impl ToString {
    input.lines().fold(0, |acc, item| {
        let first_number: Vec<usize> = dbg!(first_number_parser(item).unwrap().1.drain(..).flatten().collect());
        let last_number: Vec<usize> = dbg!(last_number_parser(&item.chars().rev().collect::<String>())
            .unwrap()
            .1
            .drain(..)
            .flatten()
            .collect());
        acc + dbg!(first_number.first().unwrap()) * 10 + dbg!(last_number.first().unwrap())
    })
}
