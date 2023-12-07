use compute::compute;
use crossbeam_channel::unbounded;
use crossbeam_utils::thread;
use std::collections::HashMap;
use std::io;
use std::io::BufWriter;
use std::io::Write;
use std::time::Duration;
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;

aoc::parts!(1);

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
}

fn part_1(input: aoc::Input) -> impl ToString {
    let program = &input_generator(input.raw());

    let mut grid: HashMap<(isize, isize), isize> = HashMap::new();

    let stdout = io::stdout().into_raw_mode().unwrap();
    let mut stdout = BufWriter::new(stdout);

    // Use asynchronous stdin
    let mut stdin = termion::async_stdin().keys();

    let (input_sender, input) = unbounded();
    let (output, output_receiver) = unbounded();

    thread::scope(|s| {
        s.spawn(|_| {
            compute(program, &input, &output);
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
    })
    .unwrap();
    0
}

// #[aoc(day15, part2)]
// pub fn part2(program: &[isize]) -> usize {
//     let mut grid: HashMap<(isize, isize), isize> = HashMap::new();
//     let mut program = program.to_vec();
//     program[0] = 2;

//     let (input_sender, input) = unbounded();
//     let (output, output_receiver) = unbounded();

//     thread::scope(|s| {
//         s.spawn(|_| {
//             compute(&program, &input, &output);
//         });

//         let mut paddle: isize = 0;
//         loop {
//             let x = output_receiver.recv_timeout(Duration::from_millis(1000));
//             let y = output_receiver.recv_timeout(Duration::from_millis(1000));
//             let tile = output_receiver.recv_timeout(Duration::from_millis(1000));

//             if let (Ok(x), Ok(y), Ok(tile)) = (x, y, tile) {
//                 grid.insert((x, y), tile);
//                 if tile == 4 {
//                     let delta = if x > paddle {
//                         1
//                     } else if x == paddle {
//                         0
//                     } else {
//                         -1
//                     };
//                     input_sender.send(delta).unwrap();
//                 }
//                 if tile == 3 {
//                     paddle = x;
//                 }
//             } else {
//                 drop(input_sender);
//                 break;
//             }
//         }
//     })
//     .unwrap();

//     for y in (0..23).rev() {
//         for x in 0..45 {
//             print!(
//                 "{}",
//                 grid.get(&(x, y))
//                     .map(|v| match v {
//                         0 => " ",
//                         1 => "H",
//                         2 => "#",
//                         3 => "-",
//                         4 => "O",
//                         _ => unreachable!(),
//                     })
//                     .unwrap_or("_")
//             );
//         }
//         println!();
//     }

//     println!("Score: {}", grid.get(&(-1, 0)).unwrap());

//     grid.values()
//         .filter_map(|v| match v {
//             0 => None,
//             1 => None,
//             2 => Some(()),
//             3 => None,
//             4 => None,
//             _ => None,
//         })
//         .count()
// }

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
