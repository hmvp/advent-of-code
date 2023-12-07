use super::compute::compute;
use aoc_runner_derive::{aoc, aoc_generator};
use crossbeam_channel::{unbounded, Receiver, Sender};
use crossbeam_utils::thread;

#[aoc_generator(day23)]
pub fn input_generator(input: &str) -> Vec<isize> {
    input.split(',').map(|l| l.parse().unwrap()).collect()
}

struct Nic {
    input: Sender<isize>,
    output: Receiver<isize>,
}

// #[aoc(day23, part1)]
// pub fn part1(program: &[isize]) -> isize {
//     let mut result = 0;
//     let mut nics: Vec<Nic> = Vec::new();
//     thread::scope(|s| {
//         (0..50).for_each(|i| {
//             let (input, input_receiver) = unbounded();
//             let (output_sender, output) = unbounded();

//             input.send(i).unwrap();

//             s.spawn(move |_| {
//                 compute(program, &input_receiver, &output_sender);
//             });

//             nics.push(Nic { input, output });
//         });

//         loop {
//             for nic in nics.iter() {
//                 if nic.input.len() == 0 {
//                     nic.input.send(-1).unwrap();
//                 }
//                 let target = nic.output.try_recv();
//                 let x = nic.output.try_recv();
//                 let y = nic.output.try_recv();

//                 if let (Ok(target), Ok(x), Ok(y)) = (target, x, y) {
//                     if (target as usize) < nics.len() {
//                         nics[target as usize].input.send(x).unwrap();
//                         nics[target as usize].input.send(y).unwrap();
//                     } else {
//                         dbg!(target, &y);
//                         result = y;

//                         nics.drain(..).for_each(drop);
//                         dbg!("aa");
//                         break;
//                     }
//                 }
//             }
//         }
//     })
//     .unwrap();
//     result
// }
#[aoc(day23, part2)]
#[allow(clippy::len_zero)]
pub fn part2(program: &[isize]) -> isize {
    let result = 0;
    let mut nics: Vec<Nic> = Vec::new();
    let mut nat = None;
    let mut last_nat = None;
    thread::scope(|s| {
        (0..50).for_each(|i| {
            let (input, input_receiver) = unbounded();
            let (output_sender, output) = unbounded();

            input.send(i as isize).unwrap();

            s.spawn(move |_| {
                compute(program, &input_receiver, &output_sender);
            });

            nics.push(Nic { input, output });
        });

        loop {
            if let (Some((x, y)), true) = (nat, nics.iter().all(|n| n.input.len() == 0)) {
                nics[0].input.send(x).unwrap();
                nics[0].input.send(y).unwrap();
            }

            for nic in nics.iter() {
                let target = nic.output.try_recv();
                let x = nic.output.try_recv();
                let y = nic.output.try_recv();

                if let (Ok(target), Ok(x), Ok(y)) = (target, x, y) {
                    if target == 255 {
                        last_nat = nat;
                        nat = Some((x, y));

                        if let (Some(last_nat), Some(nat)) = (last_nat, nat) {
                            if last_nat.1 == nat.1 {
                                dbg!(last_nat.1);
                            }
                        }
                    } else {
                        assert!((target as usize) < nics.len());
                        let nic = &nics[target as usize];
                        nic.input.send(x).unwrap();
                        nic.input.send(y).unwrap();
                    }
                }
            }
        }
    })
    .unwrap();
    result
}
