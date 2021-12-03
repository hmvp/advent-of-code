use aoc_runner_derive::{aoc, aoc_generator};
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

#[aoc_generator(day2)]
pub fn input_generator(input: &str) -> Vec<Action> {
    input.lines().map(|l| l.parse().unwrap()).collect()
}

#[aoc(day2, part1)]
pub fn part1(input: &[Action]) -> i32 {
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

#[aoc(day2, part2)]
pub fn part2(input: &[Action]) -> i32 {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_part1() {
        // assert_eq!(
        //     part1(&input_generator("""forward 5
        //     down 5
        //     forward 8
        //     up 3
        //     down 8
        //     forward 2""")),
        //     150
        // );
    }
    #[test]
    fn check_part2() {
        assert_eq!(
            part2(&[199, 200, 208, 210, 200, 207, 240, 269, 260, 263]),
            5
        );
    }
}
