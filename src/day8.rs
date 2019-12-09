use aoc_runner_derive::{aoc, aoc_generator};
use bytecount::count;

#[aoc_generator(day8)]
pub fn input_generator(input: &str) -> Vec<u8> {
    input
        .chars()
        .map(|c| c.to_digit(10).unwrap() as u8)
        .collect()
}

#[aoc(day8, part1)]
pub fn part1(input: &[u8]) -> usize {
    let layer = input
        .chunks(25 * 6)
        .map(|chunk| (count(chunk, 0), chunk))
        .fold((25 * 6, None), |(acc_nr, s), (nr, chunk)| {
            if nr < acc_nr {
                (nr, Some(chunk))
            } else {
                (acc_nr, s)
            }
        })
        .1
        .unwrap();
    count(layer, 1) * count(layer, 2)
}

#[aoc(day8, part2)]
pub fn part2(input: &[u8]) -> usize {
    const SIZE: usize = 25 * 6;

    let picture = input.chunks(SIZE).fold([2u8; SIZE], |mut picture, layer| {
        for i in 0..SIZE {
            if picture[i] == 2 {
                picture[i] = layer[i]
            }
        }
        picture
    });
    picture.chunks(25).for_each(|row| {
        row.iter()
            .for_each(|&c| if c == 1 { print!("X") } else { print!(" ") });
        println!()
    });
    0
}
