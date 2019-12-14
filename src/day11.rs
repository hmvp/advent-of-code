use super::compute::compute;
use aoc_runner_derive::{aoc, aoc_generator};
use crossbeam_channel::unbounded;
use crossbeam_utils::thread;
use std::collections::HashMap;
use std::time::Duration;

#[aoc_generator(day11)]
pub fn input_generator(input: &str) -> Vec<isize> {
    input.split(',').map(|l| l.parse().unwrap()).collect()
}

enum Direction {
    Up,
    Down,
    Right,
    Left,
}

struct Robot {
    x: isize,
    y: isize,
    dir: Direction,
}

impl Robot {
    pub fn turn(&mut self, side: isize) {
        if side == 0 {
            self.dir = match self.dir {
                Direction::Up => Direction::Left,
                Direction::Down => Direction::Right,
                Direction::Right => Direction::Up,
                Direction::Left => Direction::Down,
            }
        } else {
            self.dir = match self.dir {
                Direction::Up => Direction::Right,
                Direction::Down => Direction::Left,
                Direction::Right => Direction::Down,
                Direction::Left => Direction::Up,
            }
        }
    }

    pub fn step(&mut self) {
        match self.dir {
            Direction::Up => self.y += 1,
            Direction::Down => self.y -= 1,
            Direction::Right => self.x += 1,
            Direction::Left => self.x -= 1,
        }
    }

    pub fn loc(&self) -> (isize, isize) {
        (self.x, self.y)
    }
}

#[aoc(day11, part1)]
pub fn part1(program: &[isize]) -> usize {
    let mut grid: HashMap<(isize, isize), isize> = HashMap::new();
    let mut robot = Robot {
        x: 0,
        y: 0,
        dir: Direction::Up,
    };

    let (input_sender, input) = unbounded();
    let (output, output_receiver) = unbounded();

    thread::scope(|s| {
        s.spawn(|_| {
            compute(program, &input, &output);
        });

        loop {
            let loc = robot.loc();
            let value = *grid.get(&loc).unwrap_or(&0);
            input_sender.send(value).unwrap();
            let color = output_receiver.recv_timeout(Duration::from_millis(1000));
            let turn_dir = output_receiver.recv_timeout(Duration::from_millis(1000));

            if let (Ok(color), Ok(turn_dir)) = (color, turn_dir) {
                grid.insert(loc, color);
                robot.turn(turn_dir);
                robot.step();
            } else {
                drop(input_sender);
                break;
            }
        }
    })
    .unwrap();
    grid.len()
}

#[aoc(day11, part2)]
pub fn part2(program: &[isize]) -> usize {
    let mut grid: HashMap<(isize, isize), isize> = HashMap::new();
    grid.insert((0, 0), 1);
    let mut robot = Robot {
        x: 0,
        y: 0,
        dir: Direction::Up,
    };

    let (input_sender, input) = unbounded();
    let (output, output_receiver) = unbounded();

    thread::scope(|s| {
        s.spawn(|_| {
            compute(program, &input, &output);
        });

        loop {
            let loc = robot.loc();
            let value = *grid.get(&loc).unwrap_or(&0);
            input_sender.send(value).unwrap();
            let color = output_receiver.recv_timeout(Duration::from_millis(1000));
            let turn_dir = output_receiver.recv_timeout(Duration::from_millis(1000));

            if let (Ok(color), Ok(turn_dir)) = (color, turn_dir) {
                grid.insert(loc, color);
                robot.turn(turn_dir);
                robot.step();
            } else {
                drop(input_sender);
                break;
            }
        }
    })
    .unwrap();

    for y in (-7..3).rev() {
        for x in -10..50 {
            print!(
                "{}",
                grid.get(&(x, y))
                    .map(|v| if v == &0 { " " } else { "#" })
                    .unwrap_or(" ")
            );
        }
        println!();
    }

    grid.len()
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
            &[
                109, 1, 204, -1, 1001, 100, 1, 100, 1008, 100, 16, 101, 1006, 101, 0, 99,
            ],
            &input,
            &output,
        );
        assert_eq!(output_receiver.recv().unwrap(), 109);

        let (_, input) = unbounded();
        let (output, output_receiver) = unbounded();
        compute(
            &[1102, 34_915_192, 34_915_192, 7, 4, 7, 99, 0],
            &input,
            &output,
        );
        assert_eq!(output_receiver.recv().unwrap(), 1_219_070_632_396_864);

        let (_, input) = unbounded();
        let (output, output_receiver) = unbounded();
        compute(&[104, 1_125_899_906_842_624, 99], &input, &output);
        assert_eq!(output_receiver.recv().unwrap(), 1_125_899_906_842_624);
    }
}
