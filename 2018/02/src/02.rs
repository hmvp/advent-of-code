use std::collections::HashMap;

aoc::parts!(1, 2);

fn parse_input1(input: &str) -> Vec<(bool, bool)> {
    input
        .lines()
        .map(|line| {
            let mut count = HashMap::new();
            for c in line.chars() {
                *count.entry(c).or_insert(0) += 1;
            }

            (
                count.values().any(|&x| x == 2),
                count.values().any(|&x| x == 3),
            )
        })
        .collect()
}

fn part_1(input: aoc::Input) -> impl ToString {
    let id_counts = &parse_input1(input.raw());

    let mut twos = 0;
    let mut threes = 0;

    for (two, three) in id_counts {
        if *two {
            twos += 1;
        }
        if *three {
            threes += 1;
        }
    }
    twos * threes
}

fn parse_input2(input: &str) -> Vec<String> {
    input
        .lines()
        .map(std::string::ToString::to_string)
        .collect::<Vec<String>>()
}

fn part_2(input: aoc::Input) -> impl ToString {
    let ids = &parse_input2(input.raw());

    for a in ids {
        for b in ids {
            let same_letters = a
                .chars()
                .zip(b.chars())
                .filter(|(a, b)| a == b)
                .map(|(a, _)| a)
                .collect::<String>();

            if same_letters.len() == a.len() - 1 {
                return same_letters;
            }
        }
    }
    String::from("not found")
}
