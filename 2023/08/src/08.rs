use std::collections::{HashMap, HashSet};

use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{alphanumeric1, multispace1, newline};
use nom::combinator::{map, value, opt};
use nom::multi::many1;
use nom::sequence::{delimited, pair, separated_pair, terminated};

aoc::parts!(1, 2);

#[derive(Clone, Debug)]
enum Instruction {
    R,
    L,
}

fn parse_input(input: aoc::Input) -> (Vec<Instruction>, HashMap<&str, (&str, &str)>) {
    let parse_path = terminated(
        many1(alt((
            value(Instruction::R, tag::<&str, &str, ()>("R")),
            value(Instruction::L, tag("L")),
        ))),
        multispace1,
    );

    let parse_map = map(
        many1(terminated(
            separated_pair(
                alphanumeric1,
                tag(" = "),
                delimited(
                    tag("("),
                    separated_pair(alphanumeric1, tag(", "), alphanumeric1),
                    tag(")"),
                ),
            ),
            opt(newline),
        )),
        |mut list| list.drain(..).collect::<HashMap<&str, (&str, &str)>>(),
    );

    let mut parse_all = pair(parse_path, parse_map);

    parse_all(input.raw()).unwrap().1
}

fn part_1(input: aoc::Input) -> impl ToString {
    let (instructions, map) = parse_input(input);

    let mut current_position = "AAA";
    let mut count = 0;
    while current_position != "ZZZ" {
        for instruction in &instructions {
            count += 1;
            match instruction {
                Instruction::R => current_position = map[current_position].1,
                Instruction::L => current_position = map[current_position].0,
            }
        }
    }
    count
}

fn part_2(input: aoc::Input) -> impl ToString {
    let (instructions, map) = dbg!(parse_input(input));

    let starting_positions = map
        .keys()
        .filter(|name| name.ends_with('A'))
        .copied()
        .collect::<HashSet<_>>();
    let end_positions = map
        .keys()
        .filter(|name| name.ends_with('Z'))
        .copied()
        .collect::<HashSet<_>>();

    let mut current_positions = starting_positions;
    let mut count = 0;
    while current_positions != end_positions {
        for instruction in &instructions {
            count += 1;

            current_positions = current_positions
                .drain()
                .map(|current_position| match instruction {
                    Instruction::R => map[current_position].1,
                    Instruction::L => map[current_position].0,
                })
                .collect();

            if current_positions == end_positions {
                break;
            }
        }
    }
    count
}
