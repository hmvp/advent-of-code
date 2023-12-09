use compute::compute;
use crossbeam_channel::unbounded;
use crossbeam_utils::thread;
use std::io::Read;

aoc::parts!(1);

pub fn input_generator(input: &str) -> Vec<isize> {
    input.split(',').map(|l| l.parse().unwrap()).collect()
}

fn part_1(input: aoc::Input) -> impl ToString {
    let program = &input_generator(input.raw());

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
                output_receiver.try_iter().map(|i| u8::try_from(i).unwrap() as char).collect::<String>()
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
