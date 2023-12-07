use itertools::Itertools;

aoc::parts!(1, 2);


trait Prio {
    fn priority(self) -> usize;
}

impl Prio for char {
    fn priority(self) -> usize {
        (match self {
            c@'a'..='z' => c.to_ascii_lowercase() as u8 - 'a'.to_ascii_lowercase() as u8 + 1,
            c@'A'..='Z' => c.to_ascii_lowercase() as u8 - 'A'.to_ascii_lowercase() as u8 + 27,
            _ => unreachable!(),
        }) as usize
    }
}

fn part_1(input: aoc::Input) -> impl ToString {
    input
        .lines()
        .map(|l| {
            let (first, second) = l.split_at(l.len() / 2);
            first.chars().find(|&c| second.contains(c)).unwrap()
        })
        .map(Prio::priority).collect::<Vec<_>>().iter()
        .sum::<usize>()
}

fn part_2(input: aoc::Input) -> impl ToString {
    input
    .lines().chunks(3).into_iter()
    .map(|chunk| {
        let (first, second, third) = chunk.collect_tuple().unwrap();
        first.chars().find(|&c| second.contains(c) && third.contains(c)).unwrap()
    })
    .map(Prio::priority).collect::<Vec<_>>().iter()
    .sum::<usize>()
}
