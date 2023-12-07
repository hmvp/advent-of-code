use std::ops::RangeInclusive;

aoc::parts!(1, 2);

fn parse_input(input: aoc::Input) -> Vec<(RangeInclusive<usize>, RangeInclusive<usize>)> {
    input
        .lines()
        .map(|l| {
            let (first, second) = l.split_once(',').unwrap();
            let (first_start, first_end) = first.split_once('-').unwrap();
            let (second_start, second_end) = second.split_once('-').unwrap();
            (
                first_start.parse().unwrap()..=first_end.parse().unwrap(),
                second_start.parse().unwrap()..=second_end.parse().unwrap(),
            )
        })
        .collect::<Vec<_>>()
}

fn part_1(input: aoc::Input) -> impl ToString {
    let input = parse_input(input);

    input
        .iter()
        .filter_map(|(first, second)| {
            match (
                first.contains(second.start()),
                first.contains(second.end()),
                second.contains(first.start()),
                second.contains(first.end()),
            ) {
                (_, _, true, true) | (true, true, _, _) => Some(()),
                _ => None,
            }
        })
        .count()
}

fn part_2(input: aoc::Input) -> impl ToString {
    let input = parse_input(input);

    input
        .iter()
        .filter_map(|(first, second)| {
            if [
                first.contains(second.start()),
                first.contains(second.end()),
                second.contains(first.start()),
                second.contains(first.end()),
            ]
            .iter()
            .any(|&x| x)
            {
                Some(())
            } else {
                None
            }
        })
        .count()
}
