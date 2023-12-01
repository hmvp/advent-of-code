use aoc::Input;

aoc::parts!(1, 2);

fn part_1(input: Input) -> impl ToString {
    let input: Vec<_> = input.lines().map(|l| l.parse().unwrap()).collect();

    let mut depth = 0;
    let mut distance = 0;
    for x in input {
        match x {
            Action::Up(i) => depth -= i,
            Action::Down(i) => depth += i,
            Action::Forward(i) => distance += i,
        }
    }
    depth * distance
}

fn part_2(input: Input) -> impl ToString {
    let input: Vec<_> = input.lines().map(|l| l.parse().unwrap()).collect();

    let mut depth = 0;
    let mut distance = 0;
    let mut aim = 0;
    for x in input {
        match x {
            Action::Up(i) => aim -= i,
            Action::Down(i) => aim += i,
            Action::Forward(i) => {distance += i; depth += i * aim},
        }
    }
    depth * distance
}


use std::num::ParseIntError;
use std::str::FromStr;

pub enum Action {
    Up(i32),
    Down(i32),
    Forward(i32),
}

impl FromStr for Action {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split_ascii_whitespace().rev();
        let distance = parts.next().unwrap().parse()?;
        Ok(match parts.next() {
            Some(s) if s == "up" => Action::Up(distance),
            Some(s) if s == "down" => Action::Down(distance),
            Some(s) if s == "forward" => Action::Forward(distance),
            _ => panic!(),
        })
    }
}

