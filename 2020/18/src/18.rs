use itertools::Itertools;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::digit1,
    multi::many1,
    sequence::{delimited, pair},
    IResult,
};

aoc::parts!(1, 2);

#[derive(Debug)]
pub enum Math {
    Number(usize),
    Sum(Box<Math>, bool, Box<Math>),
}

impl Math {
    fn calculate(&self) -> usize {
        match self {
            Math::Number(n) => *n,
            Math::Sum(head, true, tail) => head.calculate() * tail.calculate(),
            Math::Sum(head, false, tail) => head.calculate() + tail.calculate(),
        }
    }
}

fn plus(input: &str) -> IResult<&str, bool> {
    let (rest, _) = delimited(tag(" "), tag("+"), tag(" "))(input)?;

    Ok((rest, false))
}

fn times(input: &str) -> IResult<&str, bool> {
    let (rest, _) = delimited(tag(" "), tag("*"), tag(" "))(input)?;

    Ok((rest, true))
}

fn operator(input: &str) -> IResult<&str, bool> {
    alt((plus, times))(input)
}

fn number(input: &str) -> IResult<&str, Math> {
    let (rest, n) = digit1(input)?;

    Ok((rest, Math::Number(n.parse().unwrap())))
}

fn sum_part1(input: &str) -> IResult<&str, Math> {
    let (rest, head) = alt((parenthesis_part1, number))(input)?;
    let (rest, mut tail) = many1(pair(operator, alt((parenthesis_part1, number))))(rest)?;

    Ok((
        rest,
        tail.drain(..)
            .fold(head, |acc, (op, tail)| Math::Sum(Box::from(acc), op, Box::from(tail))),
    ))
}

fn parenthesis_part1(input: &str) -> IResult<&str, Math> {
    delimited(tag("("), sum_part1, tag(")"))(input)
}

fn part_1(input: aoc::Input) -> impl ToString {
    input
        .lines()
        .map(|l| sum_part1(l).unwrap().1.calculate())
        .sum::<usize>()
}

fn sum_part2(input: &str) -> IResult<&str, Math> {
    let (rest, head) = alt((parenthesis_part2, number))(input)?;
    let (rest, mut tail) = many1(pair(operator, alt((parenthesis_part2, number))))(rest)?;

    let mut part = vec![head];
    let mut parts = Vec::new();
    for (op, n) in tail.drain(..) {
        if op {
            parts.push(part);
            part = vec![n];
        } else {
            part.push(n)
        }
    }
    parts.push(part);

    Ok((
        rest,
        parts
            .drain(..)
            .map(|mut part| {
                part.drain(..)
                    .fold1(|acc, tail| Math::Sum(Box::from(acc), false, Box::from(tail)))
                    .unwrap()
            })
            .fold1(|acc, tail| Math::Sum(Box::from(acc), true, Box::from(tail)))
            .unwrap(),
    ))
}

fn parenthesis_part2(input: &str) -> IResult<&str, Math> {
    delimited(tag("("), sum_part2, tag(")"))(input)
}

fn part_2(input: aoc::Input) -> impl ToString {
    input
        .lines()
        .map(|l| sum_part2(l).unwrap().1.calculate())
        .sum::<usize>()
}

#[cfg(test)]
mod tests {
    // use super::*;

    // #[test]
    // fn check_part1() {
    //     assert_eq!(part1("1 + 2 * 3 + 4 * 5 + 6"), 71);
    //     assert_eq!(part1("1 + (2 * 3) + (4 * (5 + 6))"), 51);
    //     assert_eq!(part1("2 * 3 + (4 * 5)"), 26);
    //     assert_eq!(part1("5 + (8 * 3 + 9 + 3 * 4 * 3)"), 437);
    //     assert_eq!(part1("5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))"), 12240);
    //     assert_eq!(
    //         part1("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2"),
    //         13632
    //     );
    // }
    // #[test]
    // fn check_part2() {
    //     assert_eq!(part2("1 + 2 * 3 + 4 * 5 + 6"), 231);
    //     assert_eq!(part2("1 + (2 * 3) + (4 * (5 + 6))"), 51);
    //     assert_eq!(part2("2 * 3 + (4 * 5)"), 46);
    //     assert_eq!(part2("5 + (8 * 3 + 9 + 3 * 4 * 3)"), 1445);
    //     assert_eq!(part2("5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))"), 669060);
    //     assert_eq!(
    //         part2("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2"),
    //         23340
    //     );
    // }
}
