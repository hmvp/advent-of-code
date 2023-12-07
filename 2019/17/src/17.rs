use compute::compute;
use crossbeam_channel::unbounded;
use crossbeam_utils::thread;
use std::collections::HashMap;
use std::time::Duration;

aoc::parts!(1, 2);

pub fn input_generator(input: &str) -> Vec<isize> {
    input.split(',').map(|l| l.parse().unwrap()).collect()
}

fn part_1(input: aoc::Input) -> impl ToString {
    let program = &input_generator(input.raw());

    let mut grid: HashMap<(usize, usize), char> = HashMap::new();
    let mut x: usize = 0;
    let mut y: usize = 0;
    let mut width: usize = 0;

    let (input_sender, input) = unbounded();
    let (output, output_receiver) = unbounded();

    thread::scope(|s| {
        s.spawn(|_| {
            compute(program, &input, &output);
        });

        loop {
            let tile = output_receiver.recv_timeout(Duration::from_millis(1000));

            if let Ok(tile) = tile {
                match tile {
                    10 => {
                        if x >= width {
                            width = x;
                        }
                        x = 0;
                        y += 1;
                    }
                    tile => {
                        grid.insert((x, y), tile as u8 as char);
                        x += 1;
                    }
                }
            } else {
                drop(input_sender);
                break;
            }
        }
    })
    .unwrap();

    for y in (0..y).rev() {
        for x in 0..width {
            if *grid.get(&(x, y)).unwrap_or(&' ') == '#'
                && *grid.get(&(x - 1, y)).unwrap_or(&' ') == '#'
                && *grid.get(&(x + 1, y)).unwrap_or(&' ') == '#'
                && *grid.get(&(x, y - 1)).unwrap_or(&' ') == '#'
                && *grid.get(&(x, y + 1)).unwrap_or(&' ') == '#'
            {
                grid.insert((x, y), 'O');
            }

            print!("{}", grid.get(&(x, y)).unwrap_or(&' '));
        }
        println!();
    }

    grid.drain()
        .filter_map(|((x, y), v)| match v {
            'O' => Some(x * y),
            _ => None,
        })
        .sum::<usize>()
}

fn part_2(input: aoc::Input) -> impl ToString {
    let program = &input_generator(input.raw());

    let mut grid: HashMap<(usize, usize), char> = HashMap::new();
    let mut x: usize = 0;
    let mut y: usize = 0;
    let mut width: usize = 0;

    let (input_sender, input) = unbounded();
    let (output, output_receiver) = unbounded();

    thread::scope(|s| {
        s.spawn(|_| {
            compute(program, &input, &output);
        });

        loop {
            let tile = output_receiver.recv_timeout(Duration::from_millis(1000));

            if let Ok(tile) = tile {
                match tile {
                    10 => {
                        if x >= width {
                            width = x;
                        }
                        x = 0;
                        y += 1;
                    }
                    tile => {
                        grid.insert((x, y), tile as u8 as char);
                        x += 1;
                    }
                }
            } else {
                drop(input_sender);
                break;
            }
        }
    })
    .unwrap();

    for y in (0..y).rev() {
        for x in 0..width {
            if *grid.get(&(x, y)).unwrap_or(&' ') == '#'
                && *grid.get(&(x - 1, y)).unwrap_or(&' ') == '#'
                && *grid.get(&(x + 1, y)).unwrap_or(&' ') == '#'
                && *grid.get(&(x, y - 1)).unwrap_or(&' ') == '#'
                && *grid.get(&(x, y + 1)).unwrap_or(&' ') == '#'
            {
                grid.insert((x, y), 'O');
            }

            print!("{}", grid.get(&(x, y)).unwrap_or(&' '));
        }
        println!();
    }

    grid.drain()
        .filter_map(|((x, y), v)| match v {
            'O' => Some(x * y),
            _ => None,
        })
        .sum::<usize>()
}
