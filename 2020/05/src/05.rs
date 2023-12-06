#![allow(clippy::inconsistent_digit_grouping)]


aoc::parts!(1, 2);

pub fn input_generator(input: aoc::Input) -> Vec<usize> {
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


fn part_1(input: aoc::Input) -> impl ToString {
    let input = &input_generator(input);

    *input.iter().max().unwrap()
}

fn part_2(input: aoc::Input) -> impl ToString {
    let mut input = input_generator(input);

    input.sort_unstable();
    let gap = input.windows(2).find(|window| window[0]+1 != window[1] ).unwrap();

    gap[0]+1
}
