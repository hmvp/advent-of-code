aoc::parts!(1, 2);

pub fn parse_input(input: aoc::Input) -> Vec<usize> {
    input.raw().chars().map(|c| c.to_digit(10).unwrap() as usize).collect()
}

fn moves(mut current_cup: usize, mut cups: Vec<usize>, nr_of_moves: usize) -> Vec<usize> {
    for _ in 0..nr_of_moves {
        // Take out cups
        let taken_out = [
            cups[current_cup],
            cups[cups[current_cup]],
            cups[cups[cups[current_cup]]],
        ];
        cups[current_cup] = cups[cups[cups[cups[current_cup]]]];

        // Select destination
        let mut destination_cup = current_cup.checked_sub(1).unwrap_or(cups.len() - 1);
        while taken_out.contains(&destination_cup) {
            destination_cup = destination_cup.checked_sub(1).unwrap_or(cups.len() - 1);
        }

        // Return cups
        cups[taken_out[2]] = cups[destination_cup];
        cups[destination_cup] = taken_out[0];

        // Select new current cup
        current_cup = cups[current_cup];
    }

    cups
}

fn part_1(input: aoc::Input) -> impl ToString {
    let mut input = parse_input(input);

    let mut cups = vec![999_999_999; 9];
    input.push(*input.first().unwrap());

    for n in input.windows(2) {
        cups[n[0] - 1] = n[1] - 1;
    }

    let cups: Vec<usize> = moves(*input.first().unwrap() - 1, cups, 100);
    [
        (cups[0] + 1),
        (cups[cups[0]] + 1),
        (cups[cups[cups[0]]] + 1),
        (cups[cups[cups[cups[0]]]] + 1),
        (cups[cups[cups[cups[cups[0]]]]] + 1),
        (cups[cups[cups[cups[cups[cups[0]]]]]] + 1),
        (cups[cups[cups[cups[cups[cups[cups[0]]]]]]] + 1),
        (cups[cups[cups[cups[cups[cups[cups[cups[0]]]]]]]] + 1),
        (cups[cups[cups[cups[cups[cups[cups[cups[cups[0]]]]]]]]] + 1),
    ]
    .iter()
    .take(8)
    .fold(0, |acc, i| acc * 10 + i)
}

const NR_CUPS: usize = 1_000_000;

fn part_2(input: aoc::Input) -> impl ToString {
    let mut input = parse_input(input);

    let max = *input.iter().max().unwrap();
    input.extend((max + 1)..=NR_CUPS);

    let mut cups = vec![999_999_999; NR_CUPS];
    input.push(*input.first().unwrap());

    for n in input.windows(2) {
        cups[n[0] - 1] = n[1] - 1;
    }

    let cups: Vec<usize> = moves(*input.first().unwrap() - 1, cups, 10_000_000);

    (cups[0] + 1) * (cups[cups[0]] + 1)
}
