use crossbeam_channel::{unbounded, Receiver, Sender};

fn get_argument(memory: &mut Vec<isize>, ip: usize, nr: u32, relative_base: usize) -> isize {
    let address = match memory[ip] / isize::pow(10, nr + 1) % 10 {
        0 => memory[ip + nr as usize] as usize,
        1 => ip + nr as usize,
        2 => (relative_base as isize + memory[ip + nr as usize]) as usize,
        _ => unreachable!(),
    };

    if address >= memory.len() {
        memory.resize_with(address + 1, Default::default);
    }

    memory[address]
}

fn set_result(memory: &mut Vec<isize>, ip: usize, nr: u32, relative_base: usize, value: isize) {
    let address = match memory[ip] / isize::pow(10, nr + 1) % 10 {
        0 => memory[ip + nr as usize] as usize,
        1 => ip + nr as usize,
        2 => (relative_base as isize + memory[ip + nr as usize]) as usize,
        _ => unreachable!(),
    };

    if address >= memory.len() {
        memory.resize_with(address + 1, Default::default);
    }

    memory[address] = value;
}

pub fn compute(program: &[isize], input: &Receiver<isize>, output: &Sender<isize>) {
    let mut memory = vec![0; program.len()];
    memory.clone_from_slice(program);
    let mut relative_base = 0;

    let mut ip = 0;
    loop {
        let instruction = memory[ip] % 100;

        match instruction {
            op @ 1..=2 => {
                // Multiply and Add
                let value1 = get_argument(&mut memory, ip, 1, relative_base);
                let value2 = get_argument(&mut memory, ip, 2, relative_base);
                if op == 1 {
                    set_result(&mut memory, ip, 3, relative_base, value1 + value2);
                } else {
                    set_result(&mut memory, ip, 3, relative_base, value1 * value2);
                }
                ip += 4;
            }
            3 => {
                // Input
                let input = input.recv();
                if let Ok(input) = input {
                    set_result(&mut memory, ip, 1, relative_base, input);

                    ip += 2;
                } else {
                    break;
                }
            }
            4 => {
                // Output
                let value1 = get_argument(&mut memory, ip, 1, relative_base);
                output.send(value1).unwrap();
                ip += 2;
            }
            op @ 5..=6 => {
                // Jump if
                let value1 = get_argument(&mut memory, ip, 1, relative_base);
                let value2 = get_argument(&mut memory, ip, 2, relative_base);

                if op == 5 && value1 != 0 || op == 6 && value1 == 0 {
                    ip = value2 as usize;
                } else {
                    ip += 3;
                }
            }
            op @ 7..=8 => {
                // Cmp
                let value1 = get_argument(&mut memory, ip, 1, relative_base);
                let value2 = get_argument(&mut memory, ip, 2, relative_base);
                if op == 7 && value1 < value2 || op == 8 && value1 == value2 {
                    set_result(&mut memory, ip, 3, relative_base, 1);
                } else {
                    set_result(&mut memory, ip, 3, relative_base, 0);
                }
                ip += 4;
            }
            9 => {
                relative_base = (relative_base as isize
                    + get_argument(&mut memory, ip, 1, relative_base))
                    as usize;
                ip += 2;
            }
            99 => return,
            x => {
                dbg!(x);
                dbg!(ip);
                unreachable!();
            }
        }
    }
}

pub fn simple_compute(program: &[isize], input_data: &[isize]) -> Vec<isize> {
    let (input_sender, input) = unbounded();
    let (output, output_receiver) = unbounded();

    for i in input_data {
        input_sender.send(*i).unwrap();
    }
    compute(program, &input, &output);

    let mut output = vec![];
    while !output_receiver.is_empty() {
        output.push(output_receiver.recv().unwrap());
    }

    output
}
