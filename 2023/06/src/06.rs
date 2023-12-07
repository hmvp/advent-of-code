use nom::bytes::complete::tag;
use nom::character::complete::{alphanumeric1, digit1, newline, space1};
use nom::combinator::map;
use nom::multi::{many1, separated_list1};
use nom::sequence::preceded;
use std::iter::zip;

aoc::parts!(1, 2);

struct Race {
    time: usize,
    distance: usize,
}

impl Race {
    fn winning_moves(&self) -> Vec<usize> {
        (0..=self.time)
            .map(|push_time| (self.time - push_time) * push_time)
            .filter(|&distance| distance > self.distance)
            .collect()
    }
}

fn parse_input1(input: aoc::Input) -> Vec<Race> {
    let parse_line = preceded(
        preceded(alphanumeric1::<&str, ()>, tag(":")),
        many1(preceded(space1, map(digit1, |n: &str| n.parse().unwrap()))),
    );

    map(separated_list1(newline, parse_line), |lines| {
        zip(&lines[0], &lines[1])
            .map(|(&time, &distance)| Race { time, distance })
            .collect()
    })(input.raw())
    .unwrap()
    .1
}

fn part_1(input: aoc::Input) -> impl ToString {
    let mut races = parse_input1(input);

    races.drain(..).map(|r| r.winning_moves().len()).product::<usize>()
}

fn parse_input2(input: aoc::Input) -> Race {
    let parse_line = map(
        preceded(
            preceded(alphanumeric1::<&str, ()>, tag(":")),
            many1(preceded(space1, digit1)),
        ),
        |numbers| {
            numbers
                .iter()
                .fold(String::new(), |mut acc, i| {
                    acc.push_str(i);
                    acc
                })
                .parse()
                .unwrap()
        },
    );

    map(separated_list1(newline, parse_line), |lines| Race {
        time: lines[0],
        distance: lines[1],
    })(input.raw())
    .unwrap()
    .1
}

fn part_2(input: aoc::Input) -> impl ToString {
    let race = parse_input2(input);

    race.winning_moves().len()
}
