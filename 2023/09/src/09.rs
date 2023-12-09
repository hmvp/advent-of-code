aoc::parts!(1, 2);

fn parse_input(input: aoc::Input) -> Vec<Vec<isize>> {
    input
        .lines()
        .map(|line| line.split(' ').map(|n| n.parse().unwrap()).collect::<Vec<_>>())
        .collect()
}

fn find_next(input: &[isize]) -> (isize, isize) {
    let next = input.windows(2).map(|window| window[1] - window[0]).collect::<Vec<_>>();

    if next.iter().all(|n| *n == 0) {
        return (*input.first().unwrap(), *input.last().unwrap());
    }

    let next = find_next(&next);
    (input.first().unwrap() - next.0, input.last().unwrap() + next.1)
}

fn part_1(input: aoc::Input) -> impl ToString {
    parse_input(input)
        .iter()
        .map(|sequence| find_next(sequence).1)
        .sum::<isize>()
}

fn part_2(input: aoc::Input) -> impl ToString {
    parse_input(input)
    .iter()
    .map(|sequence| find_next(sequence).0)
    .sum::<isize>()
}
