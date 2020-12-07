#![allow(clippy::inconsistent_digit_grouping)]

use aoc_runner_derive::{aoc};

#[aoc(day6, part1)]
pub fn part1(input: &str) -> u32 {
    input
    .split("\n\n")
    .map(|group| {
        group.split('\n').fold(0, |acc, answers| {
            acc | answers
                .bytes()
                .fold(0, |acc, answerx| acc | (1 << (answerx - 0x61)))
        })
    })
   .fold(0, |acc, i: u32| acc + i.count_ones())
}

#[aoc(day6, part2)]
pub fn part2(input: &str) -> u32 {
    input
    .split("\n\n")
    .map(|group| {
        group.split('\n').fold(u32::MAX, |acc, answers| {
            acc & answers
                .bytes()
                .fold(0, |acc, answerx| acc | (1 << (answerx - 0x61)))
        })
    })
   .fold(0, |acc, i: u32| acc + i.count_ones())
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "abc

a
b
c

ab
ac

a
a
a
a

b";



    #[test]
    fn check_part1() {
        assert_eq!(part1(&INPUT), 11);
    }

    #[test]
    fn check_part2() {
        assert_eq!(part2(&INPUT), 6);
    }
}
