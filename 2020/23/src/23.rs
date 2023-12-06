use std::iter::repeat_with;

aoc::parts!(1, 2);

fn digits(n: usize, base: usize) -> Vec<usize> {
    let mut n = n;
    let mut digits = repeat_with(|| {
        let m = n;
        n /= base;
        m
    })
    .take_while(|&n| n > 0)
    .map(|n| n % base)
    .collect::<Vec<_>>();
    digits.reverse();
    digits
}

pub fn input_generator(input: &str) -> Vec<usize> {
    digits(input.parse().unwrap(), 10)
}

fn find_lower_index(input: &[usize], current: usize) -> usize {
    let mut n = current - 1;
    for _ in 0..input.len() {
        if n == 0 {
            n = *input.iter().max().unwrap();
        }

        let pos = input.iter().position(|i| *i == n);

        if let Some(pos) = pos {
            n = pos;
            break;
        }

        n -= 1;
    }
    n
}

fn part_1(input: aoc::Input) -> impl ToString {
    let input = input_generator(input.raw());

    let mut cups = input;
    let mut park = cups.clone();

    for _ in 0..100 {
        let mut removed = [0, 0, 0];
        removed.swap_with_slice(&mut cups[1..4]);
        let one_lower_index = find_lower_index(&cups, cups[0]);
        cups[4..=one_lower_index].swap_with_slice(&mut park[0..one_lower_index - 3]);
        removed.swap_with_slice(&mut cups[one_lower_index - 2..=one_lower_index]);
        park[0..one_lower_index - 3].swap_with_slice(&mut cups[1..one_lower_index - 2]);

        cups.rotate_left(1);
    }
    let pos_1 = cups.iter().position(|i| *i == 1).unwrap();
    cups.rotate_left(pos_1);
    cups.iter().skip(1).fold(1, |acc, i| acc * 10 + i)
}

fn part_2(input: aoc::Input) -> impl ToString {
    let input = input_generator(input.raw());

    let max = *input.iter().max().unwrap();
    let mut cups = input;
    cups.extend(max..1_000_000);
    let mut park = cups.clone();

    for _ in 0..10_000_000 {
        let mut removed = [0, 0, 0];
        removed.swap_with_slice(&mut cups[1..4]);
        let one_lower_index = find_lower_index(&cups, cups[0]);
        cups[4..=one_lower_index].swap_with_slice(&mut park[0..one_lower_index - 3]);
        removed.swap_with_slice(&mut cups[one_lower_index - 2..=one_lower_index]);
        park[0..one_lower_index - 3].swap_with_slice(&mut cups[1..one_lower_index - 2]);

        cups.rotate_left(1);
    }
    let pos_1 = cups.iter().position(|i| *i == 1).unwrap();
    cups[pos_1 + 1] * cups[pos_1 + 2]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]

    fn check_find_lower_index() {
        assert_eq!(find_lower_index(&[1, 0, 0, 0, 9, 2, 5, 8, 4,], 1), 4);
    }
}
