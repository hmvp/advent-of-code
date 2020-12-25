use aoc_runner_derive::{aoc, aoc_generator};

const MAGIC_NR: usize = 20201227;

#[aoc_generator(day25)]
pub fn input_generator(input: &str) -> Vec<usize> {
    input.lines().flat_map(str::parse).collect()
}

fn transform(subject_number: usize, loop_size: usize) -> usize {
    let mut n = 1;

    for _ in 0..loop_size {
        n *= subject_number;
        n %= MAGIC_NR;
    }
    n
}

fn find_loop_size(public_key: usize) -> usize {
    let mut n = 1;

    for i in 1.. {
        n *= 7;
        n %= MAGIC_NR;

        if public_key == n {
            return i
        }
    }
    0
}

#[aoc(day25, part1)]
pub fn part1(input: &[usize]) -> usize {
    let card_loop_size = find_loop_size(input[0]);
    let door_loop_size = find_loop_size(input[1]);
    dbg!(input, card_loop_size,door_loop_size);

    transform(input[0], door_loop_size)
}

#[aoc(day25, part2)]
pub fn part2(input: &[usize]) -> usize {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_transform() {
        assert_eq!(transform(7, 8), 5764801);
    }

    #[test]
    fn check_part1() {
        assert_eq!(part1(&[5764801, 17807724]), 14897079);
    }
   
}
