use std::collections::HashMap;

use gcd::Gcd;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{alphanumeric1, multispace1, newline};
use nom::combinator::{map, opt, value};
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

fn resolve_instructions(
    starting_position: &str,
    map: &HashMap<&str, (&str, &str)>,
    instructions: &[Instruction],
) -> usize {
    let mut current_position = starting_position;
    let mut count = 0;
    while !current_position.ends_with('Z') {
        for instruction in instructions {
            count += 1;
            match instruction {
                Instruction::R => current_position = map[current_position].1,
                Instruction::L => current_position = map[current_position].0,
            }
        }
    }
    count
}

fn part_1(input: aoc::Input) -> impl ToString {
    let (instructions, map) = parse_input(input);

    resolve_instructions("AAA", &map, &instructions)
}

fn lcm(a: usize, b: usize) -> usize {
    a * (b / a.gcd(b))
}

fn part_2(input: aoc::Input) -> impl ToString {
    let (instructions, map) = dbg!(parse_input(input));

    map.keys()
        .filter(|name| name.ends_with('A'))
        .map(|start| resolve_instructions(start, &map, &instructions))
        .reduce(lcm)
        .unwrap()
}
