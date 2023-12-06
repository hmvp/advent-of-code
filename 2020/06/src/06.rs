#![allow(clippy::inconsistent_digit_grouping)]

aoc::parts!(1, 2);

fn part_1(input: aoc::Input) -> impl ToString {
    input.raw()
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

fn part_2(input: aoc::Input) -> impl ToString {
    input.raw()
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
