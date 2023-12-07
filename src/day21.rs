use super::compute::compute;
use aoc_runner_derive::{aoc, aoc_generator};
use crossbeam_channel::unbounded;
use crossbeam_utils::thread;
use std::time::Duration;

#[aoc_generator(day21)]
pub fn input_generator(input: &str) -> Vec<isize> {
    input.split(',').map(|l| l.parse().unwrap()).collect()
}

#[aoc(day21, part1)]
pub fn part1(program: &[isize]) -> usize {
    let (input_sender, input) = unbounded();
    let (output, output_receiver) = unbounded();

    let message = "NOT A J
NOT A J
NOT B T
OR T J
NOT C T
OR T J
AND D J
WALK
";

    for c in message.chars() {
        print!("{}", c as isize);
        input_sender.send(c as isize).unwrap();
    }

    let mut result = 0;

    thread::scope(|s| {
        s.spawn(|_| {
            compute(program, &input, &output);
        });

        loop {
            let tile = output_receiver.recv_timeout(Duration::from_millis(1000));

            if let Ok(tile) = tile {
                if tile > u8::max_value() as isize {
                    result = tile as usize;
                    break;
                } else {
                    print!("{}", tile as u8 as char);
                }
            } else {
                drop(input_sender);
                break;
            }
        }
    })
    .unwrap();
    result
}

#[aoc(day21, part2)]
pub fn part2(program: &[isize]) -> usize {
    let (input_sender, input) = unbounded();
    let (output, output_receiver) = unbounded();

    let message = "NOT A J
NOT A J
NOT B T
OR T J
NOT C T
OR T J
AND D J
AND H J
NOT E T
OR T J
RUN
";

    for c in message.chars() {
        print!("{}", c as isize);
        input_sender.send(c as isize).unwrap();
    }

    let mut result = 0;

    thread::scope(|s| {
        s.spawn(|_| {
            compute(program, &input, &output);
        });

        loop {
            let tile = output_receiver.recv_timeout(Duration::from_millis(1000));

            if let Ok(tile) = tile {
                if tile > u8::max_value() as isize {
                    result = tile as usize;
                    break;
                } else {
                    print!("{}", tile as u8 as char);
                }
            } else {
                drop(input_sender);
                break;
            }
        }
    })
    .unwrap();
    result
}
