#![allow(clippy::inconsistent_digit_grouping)]

use aoc_runner_derive::{aoc, aoc_generator};
#[aoc_generator(day5)]
pub fn input_generator(input: &str) -> Vec<usize> {
    input
        .lines()
        .map(|l| {
            l.chars().fold(0, |mut acc, c| {
                acc <<= 1;
                if c == 'B' || c == 'R' {
                    acc | 1
                } else {
                    acc
                }
            })
        })
        .collect()
}

#[aoc(day5, part1)]
pub fn part1(input: &[usize]) -> usize {
    *input.iter().max().unwrap()
}

#[aoc(day5, part2)]
pub fn part2(input: & [usize]) -> usize {
    let mut input = input.to_vec();
    input.sort_unstable();
    let gap = input.windows(2).find(|window| window[0]+1 != window[1] ).unwrap();

    gap[0]+1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_input_generator() {
        assert_eq!(input_generator("BFFFBBFRRR
FFFBBBFRRR
BBFFBBFRLL"), vec![0b100_0110_111, 0b000_1110_111, 0b110_0110_100]);
    }


    #[test]
    fn check_part1() {
        assert_eq!(part1(&[0b100_0110_111, 0b000_1110_111, 0b110_0110_100]), 820);
    }
}
