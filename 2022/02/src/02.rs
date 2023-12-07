aoc::parts!(1, 2);

enum Choice {
    Rock,
    Paper,
    Scissors,
}

impl Choice {
    fn new(from: char) -> Self {
        match from {
            'A' | 'X' => Choice::Rock,
            'B' | 'Y' => Choice::Paper,
            'C' | 'Z' => Choice::Scissors,
            _ => unreachable!(),
        }
    }

    fn outcome(them: &Choice, outcome: char) -> Self {
        match (them, outcome) {
            (Choice::Rock, 'X') => Choice::Scissors,
            (Choice::Rock, 'Y') => Choice::Rock,
            (Choice::Rock, 'Z') => Choice::Paper,
            (Choice::Paper, 'X') => Choice::Rock,
            (Choice::Paper, 'Y') => Choice::Paper,
            (Choice::Paper, 'Z') => Choice::Scissors,
            (Choice::Scissors, 'X') => Choice::Paper,
            (Choice::Scissors, 'Y') => Choice::Scissors,
            (Choice::Scissors, 'Z') => Choice::Rock,
            _ => unreachable!(),
        }
    }
}

struct Game {
    them: Choice,
    us: Choice,
}

impl Game {
    fn points(&self) -> usize {
        use Choice::*;
        match (&self.them, &self.us) {
            (Rock, Rock) => 3 + 1,
            (Rock, Paper) => 6 + 2,
            (Rock, Scissors) => 0 + 3,
            (Paper, Rock) => 0 + 1,
            (Paper, Paper) => 3 + 2,
            (Paper, Scissors) => 6 + 3,
            (Scissors, Rock) => 6 + 1,
            (Scissors, Paper) => 0 + 2,
            (Scissors, Scissors) => 3 + 3,
        }
    }
}

fn part_1(input: aoc::Input) -> impl ToString {
    input
        .lines()
        .map(|line| {
            let mut chars = line.chars();
            Game {
                them: Choice::new(chars.next().unwrap()),
                us: Choice::new(chars.nth(1).unwrap()),
            }
        })
        .fold(0, |acc, game| acc + game.points())
}

fn part_2(input: aoc::Input) -> impl ToString {
    input
        .lines()
        .map(|line| {
            let mut chars = line.chars();
            let them = Choice::new(chars.next().unwrap());
            let us = Choice::outcome(&them, chars.nth(1).unwrap());
            Game { them, us }
        })
        .fold(0, |acc, game| acc + game.points())
}
