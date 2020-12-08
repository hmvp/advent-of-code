#![allow(clippy::inconsistent_digit_grouping)]

use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::HashSet;
use std::num::ParseIntError;
use std::str::FromStr;

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum Op {
    Acc(i32),
    Nop(i32),
    Jmp(i32),
}

impl FromStr for Op {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (op, nr) = s.trim().split_at(3);
        let (sign, nr) = nr.split_at(1);
        let mut nr: i32 = nr.parse()?;

        if sign == "-" {
            nr = -nr;
        }

        Ok(match op {
            "jmp" => Op::Jmp(nr),
            "nop" => Op::Nop(nr),
            "acc" => Op::Acc(nr),
            _ => unreachable!(),
        })
    }
}

impl Op {
    fn execute(self, pc: &mut usize, acc: &mut isize) {
        match self {
            Op::Acc(nr) => {
                *pc += 1;
                *acc += nr as isize;
            }
            Op::Nop(_) => *pc += 1,
            Op::Jmp(nr) => {
                if nr >= 0 {
                    *pc += nr as usize
                } else {
                    *pc -= -nr as usize
                }
            }
        }
    }

    fn swap(&mut self) {
        match self {
            Op::Acc(_) => {}
            Op::Nop(nr) => *self = Op::Jmp(*nr),
            Op::Jmp(nr) => *self = Op::Nop(*nr),
        }
    }
}

#[aoc_generator(day8)]
pub fn input_generator(input: &str) -> Vec<Op> {
    input.lines().flat_map(str::parse).collect()
}

fn run(input: &[Op]) -> Result<isize, isize> {
    let mut pc = 0;
    let mut acc = 0;
    let mut seen: HashSet<usize> = HashSet::new();

    loop {
        if !seen.insert(pc) {
            return Err(acc);
        }

        if pc >= input.len() {
            return Ok(acc);
        }

        input[pc].execute(&mut pc, &mut acc);
    }
}

#[aoc(day8, part1)]
pub fn part1(input: &[Op]) -> isize {
    run(input).unwrap_err()
}

#[aoc(day8, part2)]
pub fn part2(input: &[Op]) -> isize {
    let good_input = input
        .iter()
        .scan((input, 0), |(input, index), op| {
            *index += 1;
            if let Op::Acc(_) = op {
                Some(vec![])
            } else {
                let mut result = vec![input.to_vec(), input.to_vec()];
                result[0][*index - 1].swap();
                Some(result)
            }
        })
        .flatten()
        .find(|input| run(input).is_ok())
        .unwrap();

    run(&good_input).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_input_generator() {
        assert_eq!(
            input_generator(
                "nop +0
                acc +1
                jmp +4
                acc +3
                jmp -3
                acc -99
                acc +1
                jmp -4
                acc +6"
            ),
            vec![
                Op::Nop(0),
                Op::Acc(1),
                Op::Jmp(4),
                Op::Acc(3),
                Op::Jmp(-3),
                Op::Acc(-99),
                Op::Acc(1),
                Op::Jmp(-4),
                Op::Acc(6)
            ]
        );
    }

    #[test]
    fn check_part1() {
        assert_eq!(
            part1(&vec![
                Op::Nop(0),
                Op::Acc(1),
                Op::Jmp(4),
                Op::Acc(3),
                Op::Jmp(-3),
                Op::Acc(-99),
                Op::Acc(1),
                Op::Jmp(-4),
                Op::Acc(6)
            ]),
            5
        );
    }

    #[test]
    fn check_part2() {
        assert_eq!(
            part2(&input_generator(
                "nop +0
                acc +1
                jmp +4
                acc +3
                jmp -3
                acc -99
                acc +1
                jmp -4
                acc +6"
            )),
            8
        );
    }
}
