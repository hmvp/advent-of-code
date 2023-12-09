aoc::parts!(1, 2);

use compute::compute;
use crossbeam_channel::unbounded;
use crossbeam_utils::thread;
use std::collections::HashMap;
use std::time::Duration;

pub fn input_generator(input: &str) -> Vec<isize> {
    input.split(',').map(|l| l.parse().unwrap()).collect()
}

fn part_1(input: aoc::Input) -> impl ToString {
    let program = &input_generator(input.raw());

    let mut grid: HashMap<(isize, isize), isize> = HashMap::new();

    let (input_sender, input) = unbounded();
    let (output, output_receiver) = unbounded();

    thread::scope(|s| {
        s.spawn(|_| {
            compute(program, &input, &output);
        });

        loop {
            let x = output_receiver.recv_timeout(Duration::from_millis(1000));
            let y = output_receiver.recv_timeout(Duration::from_millis(1000));
            let tile = output_receiver.recv_timeout(Duration::from_millis(1000));

            if let (Ok(x), Ok(y), Ok(tile)) = (x, y, tile) {
                grid.insert((x, y), tile);
            } else {
                drop(input_sender);
                break;
            }
        }
    })
    .unwrap();

    for y in (0..23).rev() {
        for x in 0..45 {
            print!(
                "{}",
                grid.get(&(x, y)).map_or(" ", |v| match v {
                    0 => " ",
                    1 => "H",
                    2 => "#",
                    3 => "-",
                    4 => "O",
                    _ => unreachable!(),
                })
            );
        }
        println!();
    }

    grid.values().filter(|&&v| v == 2).count()
}

fn part_2(input: aoc::Input) -> impl ToString {
    let mut program = input_generator(input.raw());

    let mut grid: HashMap<(isize, isize), isize> = HashMap::new();
    program[0] = 2;

    let (input_sender, input) = unbounded();
    let (output, output_receiver) = unbounded();

    thread::scope(|s| {
        s.spawn(|_| {
            compute(&program, &input, &output);
        });

        let mut paddle: isize = 0;
        loop {
            let x = output_receiver.recv_timeout(Duration::from_millis(1000));
            let y = output_receiver.recv_timeout(Duration::from_millis(1000));
            let tile = output_receiver.recv_timeout(Duration::from_millis(1000));

            if let (Ok(x), Ok(y), Ok(tile)) = (x, y, tile) {
                grid.insert((x, y), tile);
                if tile == 4 {
                    let delta = match x {
                        x if x > paddle => 1,
                        x if x == paddle => 0,
                        _ => -1,
                    };
                    input_sender.send(delta).unwrap();
                }
                if tile == 3 {
                    paddle = x;
                }
            } else {
                drop(input_sender);
                break;
            }
        }
    })
    .unwrap();

    for y in (0..23).rev() {
        for x in 0..45 {
            print!(
                "{}",
                grid.get(&(x, y)).map_or(" ", |v| match v {
                    0 => " ",
                    1 => "H",
                    2 => "#",
                    3 => "-",
                    4 => "O",
                    _ => unreachable!(),
                })
            );
        }
        println!();
    }

    *grid.get(&(-1, 0)).unwrap()
}

#[cfg(test)]
mod tests {
    use super::compute;
    use crossbeam_channel::unbounded;

    #[test]
    fn check_compute() {
        let (input_sender, input) = unbounded();
        let (output, output_receiver) = unbounded();

        input_sender.send(1234).unwrap();
        input_sender.send(1234).unwrap();
        input_sender.send(1234).unwrap();

        compute(&[109, 20, 203, -1, 204, -1, 99], &input, &output);
        assert_eq!(output_receiver.recv().unwrap(), 1234);

        let (_, input) = unbounded();
        let (output, output_receiver) = unbounded();
        compute(
            &[109, 1, 204, -1, 1001, 100, 1, 100, 1008, 100, 16, 101, 1006, 101, 0, 99],
            &input,
            &output,
        );
        assert_eq!(output_receiver.recv().unwrap(), 109);

        let (_, input) = unbounded();
        let (output, output_receiver) = unbounded();
        compute(&[1102, 34_915_192, 34_915_192, 7, 4, 7, 99, 0], &input, &output);
        assert_eq!(output_receiver.recv().unwrap(), 1_219_070_632_396_864);

        let (_, input) = unbounded();
        let (output, output_receiver) = unbounded();
        compute(&[104, 1_125_899_906_842_624, 99], &input, &output);
        assert_eq!(output_receiver.recv().unwrap(), 1_125_899_906_842_624);
    }
}
