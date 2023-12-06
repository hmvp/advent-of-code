use itertools::Itertools;

aoc::parts!(1, 2);

fn parse_input(input: aoc::Input) -> Vec<Option<usize>> {
    input
        .lines()
        .map(|line| {
            if line.is_empty() {
                None
            } else {
                Some(line.parse().unwrap())
            }
        })
        .collect()
}

fn part_1(input: aoc::Input) -> impl ToString {
    let input = parse_input(input);

    input
        .iter()
        .batching(|it| {
            if let Some(Some(n)) = it.next() {
                Some(it.take_while(|l| l.is_some()).fold(*n, |acc, i| acc + i.unwrap()))
            } else {
                None
            }
        })
        .max()
        .unwrap()
}

fn part_2(input: aoc::Input) -> impl ToString {
    let input = parse_input(input);

    input
        .iter()
        .batching(|it| {
            if let Some(Some(n)) = it.next() {
                Some(it.take_while(|l| l.is_some()).fold(*n, |acc, i| acc + i.unwrap()))
            } else {
                None
            }
        })
        .sorted()
        .rev()
        .take(3)
        .sum::<usize>()
}
