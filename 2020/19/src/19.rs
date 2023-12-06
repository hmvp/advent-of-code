use itertools::Itertools;
use nom::{
    branch::alt, bytes::complete::tag, combinator::map as nom_map, sequence::delimited, sequence::tuple, Finish,
    IResult,
};
use std::collections::HashMap;

aoc::parts!(1, 2);

#[derive(Debug, Clone)]
pub enum Rule {
    Char(char),
    Single(usize),
    Double((usize, usize)),
    SingleOr(usize, usize),
    DoubleOr((usize, usize), (usize, usize)),
    Part2R1(usize, (usize, usize)),
    Part2R2((usize, usize), (usize, usize, usize)),
}

//fn(&str) -> IResult<&str, ()>>

pub fn input_generator(input: &str) -> (HashMap<usize, Rule>, Vec<String>) {
    let mut lines = input.trim().lines();

    let parser = lines
        .by_ref()
        .take_while(|l| !l.is_empty())
        .map(|l| {
            let mut rule_split = l.split(": ");
            let rule = rule_split.next().unwrap().parse().unwrap();

            let rule_str = rule_split.next().unwrap();
            (
                rule,
                if rule_str.contains('"') {
                    Rule::Char(rule_str.trim_matches('"').chars().next().unwrap())
                } else if rule_str.contains('|') {
                    let mut options = rule_str
                        .split('|')
                        .map(|r| r.split(' ').flat_map(str::parse).collect_vec())
                        .collect_vec();
                    let mut options = options.drain(..);
                    let first = options.next().unwrap();

                    if first.len() == 1 {
                        Rule::SingleOr(first[0], options.next().unwrap()[0])
                    } else {
                        let second = options.next().unwrap();
                        Rule::DoubleOr((first[0], first[1]), (second[0], second[1]))
                    }
                } else {
                    let x = rule_str.split(' ').flat_map(str::parse).collect_vec();

                    if x.len() == 1 {
                        Rule::Single(x[0])
                    } else {
                        Rule::Double((x[0], x[1]))
                    }
                },
            )
        })
        .collect();

    let strings = lines.map(ToString::to_string).collect();

    dbg!((parser, strings))
}

fn create_parser<'a, 'c>(
    map: &'c HashMap<usize, Rule>,
    rule: usize,
) -> Box<dyn FnMut(&'a str) -> IResult<&'a str, ()> + '_> {
    let x: Box<dyn FnMut(&'a str) -> IResult<&'a str, ()> + '_> = match map.get(&rule).unwrap() {
        Rule::Char(c) => Box::from(move |s: &'a str| {
            let result = tag(&*c.to_string())(s);
            result.map(|(rest, _)| (rest, ()))
        }),
        Rule::Single(rule) => Box::from(move |s: &'a str| {
            let result = create_parser(map, *rule)(s);
            result.map(|(rest, ())| (rest, ()))
        }),
        Rule::Double(rules) => Box::from(move |s: &'a str| {
            let result = tuple((create_parser(map, rules.0), create_parser(map, rules.1)))(s);
            result.map(|(rest, _)| (rest, ()))
        }),
        Rule::SingleOr(rules1, rules2) => Box::from(move |s: &'a str| {
            let result = alt((create_parser(map, *rules1), create_parser(map, *rules2)))(s);
            result.map(|(rest, ())| (rest, ()))
        }),
        Rule::DoubleOr(rules1, rules2) => Box::from(move |s: &'a str| {
            let result = alt((
                tuple((create_parser(map, rules1.0), create_parser(map, rules1.1))),
                tuple((create_parser(map, rules2.0), create_parser(map, rules2.1))),
            ))(s);
            result.map(|(rest, _)| (rest, ()))
        }),
        Rule::Part2R1(rules1, rules2) => Box::from(move |s: &'a str| {
            let result = alt((
                create_parser(map, *rules1),
                nom_map(
                    tuple((create_parser(map, rules2.0), create_parser(map, rules2.1))),
                    |_| (),
                ),
            ))(s);
            result.map(|(rest, ())| (rest, ()))
        }),
        Rule::Part2R2(rules1, rules2) => Box::from(move |s: &'a str| {
            let result = alt((
                tuple((create_parser(map, rules1.0), create_parser(map, rules1.1))),
                nom_map(
                    delimited(
                        create_parser(map, rules2.0),
                        create_parser(map, rules2.1),
                        create_parser(map, rules2.2),
                    ),
                    |()| ((), ()),
                ),
            ))(s);
            result.map(|(rest, _)| (rest, ()))
        }),
    };
    x
}

fn part_1(input: aoc::Input) -> impl ToString {
    let input = &input_generator(input.raw());

    input
        .1
        .iter()
        .map(|i| {
            dbg!(create_parser(&input.0, 0)(i).finish())
                .map(|(rest, ())| rest.is_empty())
                .unwrap_or(false)
        })
        .filter(|result| *result)
        .count()
}

fn part_2(input: aoc::Input) -> impl ToString {
    let input = &input_generator(input.raw());

    let mut input = (*input).clone();

    input.0.insert(8, Rule::Part2R1(42, (42, 8)));
    input.0.insert(11, Rule::Part2R2((42, 31), (42, 11, 31)));
    input
        .1
        .iter()
        .map(|i| {
            dbg!(create_parser(&input.0, 0)(i).finish())
                .map(|(rest, ())| rest.is_empty())
                .unwrap_or(false)
        })
        .filter(|result| *result)
        .count()
}
