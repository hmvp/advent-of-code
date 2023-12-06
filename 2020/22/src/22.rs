use itertools::Itertools;
use std::collections::HashSet;

aoc::parts!(1, 2);

pub fn input_generator(input: &str) -> Vec<Vec<usize>> {
    input
        .trim()
        .split("\n\n")
        .map(|deck| deck.lines().dropping(1).flat_map(str::parse).collect_vec())
        .collect()
}

fn part_1(input: aoc::Input) -> impl ToString {
    let input = &input_generator(input.raw());

    let mut deck1 = input[0].clone();
    let mut deck2 = input[1].clone();
    let winning = loop {
        if deck1.is_empty() {
            break deck2;
        }

        if deck2.is_empty() {
            break deck1;
        }

        let card1 = deck1.remove(0);
        let card2 = deck2.remove(0);

        if card1 > card2 {
            deck1.push(card1);
            deck1.push(card2);
        } else {
            deck2.push(card2);
            deck2.push(card1);
        }
    };

    winning
        .iter()
        .rev()
        .enumerate()
        .fold(0, |acc, (i, c)| acc + (i + 1) * c)
}

fn play(input: &[Vec<usize>]) -> (bool, Vec<usize>) {
    let mut hands: HashSet<(Vec<usize>, Vec<usize>)> = HashSet::new();

    let mut deck1 = input[0].clone();
    let mut deck2 = input[1].clone();
    loop {
        if !hands.insert((deck1.clone(), deck2.clone())) {
            break (true, deck1);
        }

        if deck1.is_empty() {
            break (false, deck2);
        }

        if deck2.is_empty() {
            break (true, deck1);
        }

        let card1 = deck1.remove(0);
        let card2 = deck2.remove(0);

        let mut deck1_won = card1 > card2;
        if card1 <= deck1.len() && card2 <= deck2.len() {
            deck1_won = play(&[Vec::from(&deck1[0..card1]), Vec::from(&deck2[0..card2])]).0;
        }

        if deck1_won {
            deck1.push(card1);
            deck1.push(card2);
        } else {
            deck2.push(card2);
            deck2.push(card1);
        }
    }
}

fn part_2(input: aoc::Input) -> impl ToString {
    let input = &input_generator(input.raw());

    let (_, winning) = play(input);

    winning
        .iter()
        .rev()
        .enumerate()
        .fold(0, |acc, (i, c)| acc + (i + 1) * c)
}

#[cfg(test)]
mod tests {
    // use super::*;

    //     #[test]
    //     fn check_part2() {
    //         assert_eq!(
    //             part2(&input_generator(
    //                 "
    // Player 1:
    // 43
    // 19

    // Player 2:
    // 2
    // 29
    // 14
    // "
    //             )),
    //             105
    //         );
    //     }
}
