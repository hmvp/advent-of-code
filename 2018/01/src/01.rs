aoc::parts!(1, 2);

fn parse_input(input: aoc::Input) -> Vec<i32> {
    input.lines().map(|l| l.parse().unwrap()).collect::<Vec<_>>()
}

fn part_1(input: aoc::Input) -> impl ToString {
    let freqs = parse_input(input);

    freqs.iter().sum::<i32>()
}

fn part_2(input: aoc::Input) -> impl ToString {
    let freqs = parse_input(input);

    let mut freq = 0;
    let mut seen = vec![];
    for delta in freqs.iter().cycle() {
        freq += delta;

        if seen.contains(&freq) {
            return freq;
        }

        seen.push(freq);
    }
    0
}
