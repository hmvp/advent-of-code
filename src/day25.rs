use super::compute::compute;
use aoc_runner_derive::{aoc, aoc_generator};
use crossbeam_channel::unbounded;
use crossbeam_utils::thread;
use std::io::Read;
use termion;

#[aoc_generator(day25)]
pub fn input_generator(input: &str) -> Vec<isize> {
    input.split(',').map(|l| l.parse().unwrap()).collect()
}

#[aoc(day25, part1)]
pub fn part1(program: &[isize]) -> usize {
    let mut stdin = termion::async_stdin();
    let mut input_string = String::new();

    let (input_sender, input) = unbounded();
    let (output, output_receiver) = unbounded();

    thread::scope(|s| {
        s.spawn(|_| {
            compute(program, &input, &output);
        });

        loop {
            print!(
                "{}",
                output_receiver
                    .try_iter()
                    .map(|i| i as u8 as char)
                    .collect::<String>()
            );
            stdin.read_to_string(&mut input_string).unwrap();

            for i in input_string.chars().map(|c| c as isize) {
                input_sender.send(i).unwrap();
            }
            input_string.truncate(0);
        }
    })
    .unwrap();
    0
}
