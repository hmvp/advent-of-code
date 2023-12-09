use compute::compute;
use crossbeam_channel::unbounded;
use std::collections::HashMap;
use std::io;
use std::io::BufWriter;
use std::io::Write;
use std::time::Duration;
use std::usize;
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;

aoc::parts!(1, 2);

pub fn input_generator(input: &str) -> Vec<isize> {
    input.split(',').map(|l| l.parse().unwrap()).collect()
}

enum Dir {
    Up,
    Down,
    Left,
    Right,
}

impl Dir {
    pub fn to_input(&self) -> isize {
        match self {
            Self::Up => 1,
            Self::Down => 2,
            Self::Left => 3,
            Self::Right => 4,
        }
    }

    fn next(&self, hit_wall: bool) -> Self {
        use Dir::*;
        if hit_wall {
            match self {
                Up => Right,
                Right => Down,
                Down => Left,
                Left => Up,
            }
        } else {
            match self {
                Up => Left,
                Left => Down,
                Down => Right,
                Right => Up,
            }
        }
    }
}

fn compute_map(input: aoc::Input) -> HashMap<(isize, isize), isize> {
    let program: Vec<isize> = input_generator(input.raw());

    let mut grid: HashMap<(isize, isize), isize> = HashMap::new();

    let (input_sender, input) = unbounded();
    let (output, output_receiver) = unbounded();

    std::thread::spawn(move || {
        compute(&program, &input, &output);
    });

    let mut x = 0;
    let mut y = 0;
    let mut dir = Dir::Right;
    loop {
        input_sender.send(dir.to_input()).unwrap();
        let result = output_receiver.recv().unwrap();

        match result {
            0 => match dir {
                Dir::Up => {
                    grid.insert((x, y + 1), 2);
                    dir = dir.next(false);
                }
                Dir::Down => {
                    grid.insert((x, y - 1), 2);
                    dir = dir.next(false);
                }
                Dir::Left => {
                    grid.insert((x - 1, y), 2);
                    dir = dir.next(false);
                }
                Dir::Right => {
                    grid.insert((x + 1, y), 2);
                    dir = dir.next(false);
                }
            },
            1 => {
                match dir {
                    Dir::Up => y += 1,
                    Dir::Down => y -= 1,
                    Dir::Left => x -= 1,
                    Dir::Right => x += 1,
                };
                grid.insert((x, y), 0);
                dir = dir.next(true);
            }
            2 => {
                match dir {
                    Dir::Up => y += 1,
                    Dir::Down => y -= 1,
                    Dir::Left => x -= 1,
                    Dir::Right => x += 1,
                };
                grid.insert((x, y), 1);
                dir = dir.next(true);
            }
            _ => unreachable!(),
        };

        if x == 0 && y == 0 && result == 1 {
            break;
        }
    }

    // for py in (1..=41).rev() {
    //     for px in 1..=41 {
    //         print!(
    //             "{}",
    //             grid.get(&((px - 22), (py - 20))).map_or("#", |v| match v {
    //                 0 => ".",
    //                 1 => "@",
    //                 2 => "#",
    //                 _ => unreachable!(),
    //             })
    //         );
    //     }
    //     println!();
    // }

    grid
}

fn part_1_terminal(input: aoc::Input) -> impl ToString {
    let program: Vec<isize> = input_generator(input.raw());

    let mut grid: HashMap<(isize, isize), isize> = HashMap::new();

    let stdout = io::stdout().into_raw_mode().unwrap();
    let mut stdout = BufWriter::new(stdout);

    // Use asynchronous stdin
    let mut stdin = termion::async_stdin().keys();

    let (input_sender, input) = unbounded();
    let (output, output_receiver) = unbounded();

    std::thread::spawn(move || {
        compute(&program, &input, &output);
    });

    let mut x = 0;
    let mut y = 0;
    write!(stdout, "\n\r").unwrap();
    'outer: loop {
        let term_size = termion::terminal_size().unwrap();
        let width = term_size.0 as isize;
        let height = term_size.1 as isize;

        write!(stdout, "{}", termion::cursor::Goto(1, 1)).unwrap();
        for py in (0..height).rev() {
            for px in 0..width {
                if px - 50 == x && py - 30 == y {
                    write!(stdout, "X").unwrap();
                } else {
                    write!(
                        stdout,
                        "{}",
                        grid.get(&((px - 50), (py - 30))).map_or(" ", |v| match v {
                            0 => ".",
                            1 => "@",
                            2 => "#",
                            _ => "!",
                        })
                    )
                    .unwrap();
                }
            }
            write!(stdout, "\n\r").unwrap();
        }
        stdout.flush().unwrap();

        for key in stdin.by_ref() {
            let dir = match key {
                Ok(Key::Up) => Some(Dir::Up),
                Ok(Key::Down) => Some(Dir::Down),
                Ok(Key::Left) => Some(Dir::Left),
                Ok(Key::Right) => Some(Dir::Right),
                Ok(Key::Home) => {
                    drop(input_sender);

                    break 'outer;
                }
                _ => None,
            };

            if let Some(dir) = dir {
                input_sender.send(dir.to_input()).unwrap();
                let result = output_receiver.recv().unwrap();

                match result {
                    0 => match dir {
                        Dir::Up => grid.insert((x, y + 1), 2),
                        Dir::Down => grid.insert((x, y - 1), 2),
                        Dir::Left => grid.insert((x - 1, y), 2),
                        Dir::Right => grid.insert((x + 1, y), 2),
                    },
                    1 => {
                        match dir {
                            Dir::Up => y += 1,
                            Dir::Down => y -= 1,
                            Dir::Left => x -= 1,
                            Dir::Right => x += 1,
                        };
                        grid.insert((x, y), 0)
                    }
                    2 => {
                        match dir {
                            Dir::Up => y += 1,
                            Dir::Down => y -= 1,
                            Dir::Left => x -= 1,
                            Dir::Right => x += 1,
                        };
                        grid.insert((x, y), 1)
                    }
                    _ => None,
                };
            }
        }

        std::thread::sleep(Duration::from_millis(100));
    }
    236
}

fn fill_oxygen(
    mut grid: HashMap<(isize, isize), isize>,
    early_exit_cond: Option<fn(x: isize, y: isize) -> bool>,
) -> usize {
    let mut count = 0;

    // Oxygen machine
    for value in grid.values_mut() {
        if value == &1 {
            *value = 3;
        }
    }

    'outer: loop {
        let mut found_empty = false;

        let mut new_grid = grid.clone();
        for ((x, y), value) in &grid {
            if value == &3 {
                if grid.get(&(x - 1, *y)).unwrap_or(&2) == &0 {
                    found_empty = true;

                    new_grid.insert((x - 1, *y), 3);
                }

                if grid.get(&(x + 1, *y)).unwrap_or(&2) == &0 {
                    found_empty = true;

                    new_grid.insert((x + 1, *y), 3);
                }

                if grid.get(&(*x, y - 1)).unwrap_or(&2) == &0 {
                    found_empty = true;

                    new_grid.insert((*x, y - 1), 3);
                }

                if grid.get(&(*x, y + 1)).unwrap_or(&2) == &0 {
                    found_empty = true;

                    new_grid.insert((*x, y + 1), 3);
                }

                if let Some(exit_cond) = early_exit_cond {
                    if exit_cond(*x, *y) {
                        break 'outer;
                    }
                }
            }
        }

        grid = new_grid;

        // for py in (1..=41).rev() {
        //     for px in 1..=41 {
        //         print!(
        //             "{}",
        //             grid.get(&((px - 22), (py - 20))).map_or("#", |v| match v {
        //                 0 => ".",
        //                 1 => "@",
        //                 2 => "#",
        //                 3 => "O",
        //                 _ => unreachable!(),
        //             })
        //         );
        //     }
        //     println!();
        // }

        if !found_empty {
            break 'outer;
        }

        count += 1;
    }

    count
}

fn part_1(input: aoc::Input) -> impl ToString {
    let grid = compute_map(input);

    fill_oxygen(grid, Some(|x: isize, y| x == 0 && y == 0))
}

fn part_2(input: aoc::Input) -> impl ToString {
    let grid = compute_map(input);

    fill_oxygen(grid, None)
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

//           ######### ######### ### ##### #
//          #         #         #   #  ===# #
//         ## # ##### # ####### ### # #=#=# #
//        #   #   #   #   #   #   #   #=#===#
//        # #   # ### ### ### ### #####=###=#
//        #     #   #   #   #     #=====#@==#
//         #     ## # ##### # #####=####### #
//                # # #==== #     #=#=====# #
//             #### # #=## ###### #=#=###=# #
//            #     # #=# #       #===# #=# #
//       ###### ##### #=# # ####### ### #=# #
//      #     # #     #=#   #     #     #=# #
//      # ### # # #####=# ### # ##### ###=# #
//      # # # # #   #===#   # # #     #===# #
//      # # # # ### #=##### # # # #####=### #
//      # # # # #   #=#   # # #       #=#   #
//      # # # # # ###=# ### # #########=# ##
//      # #   # # # #=#   # #   #=======# # #
//      # # ### # # #=### # #####=####### # #
//      # #   # #   #===#     #===#   #   # #
//      # ### # #######=##### #=### # # ### #
//      # # # #       #=#X#   #   # #   #   #
//      # # # ####### #=#=# ### # ##### # # #
//      # #         # #=#=# #   #     #   # #
//      # ######### # #=#=# ### ##### ##### #
//      #   # #   # # #===#   #     # #   # #
//       ## # # # # # ####### ##### # # # # #
//        # #   #           #     # #   # # #
//     #### ############### ##### ####### # #
//    #   #         #   # # #   # #   #   # #
//   ## # ######### # # # # # # # # # # ####
//  #   #           # # # # # # #   # # #   #
//  # ### ########### # # # # # ##### # # # #
//  # #   #           # #   # #     # #   # #
//  # ##### ########### # ### ##### # ##### #
//  # #     #         # #   # # #   # #   # #
//  # # ####          # ##### # # ### # # # #
//  #   #             # #   # # # #   # #   #
//  # ###             # # # # # # # ### ### #
//  #                 #   #     #       #   #
//   ####              ### ##### ####### ###
