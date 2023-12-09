use std::ops::RangeInclusive;

aoc::parts!(1, 2);

pub fn input_generator(input: &str) -> RangeInclusive<usize> {
    let nrs: Vec<usize> = input.split('-').map(|n| n.parse().unwrap()).collect();
    nrs[0]..=nrs[1]
}

#[allow(clippy::trivially_copy_pass_by_ref)]
fn is_password_part1(number: &usize) -> bool {
    let result = number
        .to_string()
        .chars()
        .fold((false, true, '0'), |(double, increasing, l), c| {
            (
                double || c == l,
                increasing && c.to_digit(10).unwrap() >= l.to_digit(10).unwrap(),
                c,
            )
        });
    result.0 && result.1
}

fn part_1(input: aoc::Input) -> impl ToString {
    let range = &input_generator(input.raw());

    range.clone().filter(is_password_part1).count()
}

#[allow(clippy::trivially_copy_pass_by_ref)]
fn is_password_part2(number: &usize) -> bool {
    let increasing = number
        .to_string()
        .chars()
        .fold((true, '0'), |(increasing, l), c| {
            (increasing && c.to_digit(10).unwrap() >= l.to_digit(10).unwrap(), c)
        })
        .0;

    let doubles = (0..=9)
        .map(|i| {
            number
                .to_string()
                .chars()
                .map(|c| c.to_digit(10).unwrap())
                .filter(|x| *x == i)
                .count()
        })
        .any(|c| c == 2);

    increasing && doubles
}

fn part_2(input: aoc::Input) -> impl ToString {
    let range = &input_generator(input.raw());

    range.clone().filter(is_password_part2).count()
}
