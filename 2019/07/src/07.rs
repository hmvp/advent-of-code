use compute::compute;
use crossbeam_channel::unbounded;
use crossbeam_utils::thread;
use permutohedron::heap_recursive;

aoc::parts!(1, 2);

pub fn input_generator(input: &str) -> Vec<isize> {
    input.split(',').map(|l| l.parse().unwrap()).collect()
}

fn run_amplifiers(program: &[isize], phase_settings: &[isize]) -> isize {
    let (input, input_a) = unbounded();
    let (output_a, input_b) = unbounded();
    let (output_b, input_c) = unbounded();
    let (output_c, input_d) = unbounded();
    let (output_d, input_e) = unbounded();
    let (output_e, output) = unbounded();

    input.send(phase_settings[0]).unwrap();
    output_a.send(phase_settings[1]).unwrap();
    output_b.send(phase_settings[2]).unwrap();
    output_c.send(phase_settings[3]).unwrap();
    output_d.send(phase_settings[4]).unwrap();

    input.send(0).unwrap();

    compute(program, &input_a, &output_a);
    compute(program, &input_b, &output_b);
    compute(program, &input_c, &output_c);
    compute(program, &input_d, &output_d);
    compute(program, &input_e, &output_e);
    output.recv().unwrap()
}

fn part_1(input: aoc::Input) -> impl ToString {
    let program = &input_generator(input.raw());

    let mut data = [0, 1, 2, 3, 4];
    let mut permutations = Vec::new();
    heap_recursive(&mut data, |permutation| {
        permutations.push(permutation.to_vec());
    });

    permutations
        .iter()
        .map(|p| run_amplifiers(program, p))
        .max()
        .unwrap()
}

fn run_amplifiers_part2(program: &[isize], phase_settings: &[isize]) -> isize {
    let (output_a, input_b) = unbounded();
    let (output_b, input_c) = unbounded();
    let (output_c, input_d) = unbounded();
    let (output_d, input_e) = unbounded();
    let (output_e, input_a) = unbounded();

    output_e.send(phase_settings[0]).unwrap();
    output_a.send(phase_settings[1]).unwrap();
    output_b.send(phase_settings[2]).unwrap();
    output_c.send(phase_settings[3]).unwrap();
    output_d.send(phase_settings[4]).unwrap();

    output_e.send(0).unwrap();

    thread::scope(|s| {
        s.spawn(|_| {
            compute(program, &input_a, &output_a);
        });
        s.spawn(|_| {
            compute(program, &input_b, &output_b);
        });
        s.spawn(|_| {
            compute(program, &input_c, &output_c);
        });
        s.spawn(|_| {
            compute(program, &input_d, &output_d);
        });
        s.spawn(|_| {
            compute(program, &input_e, &output_e);
        });
    })
    .unwrap();

    input_a.recv().unwrap()
}

fn part_2(input: aoc::Input) -> impl ToString {
    let program = &input_generator(input.raw());

    let mut data = [5, 6, 7, 8, 9];
    let mut permutations = Vec::new();
    heap_recursive(&mut data, |permutation| {
        permutations.push(permutation.to_vec());
    });

    permutations
        .iter()
        .map(|p| run_amplifiers_part2(program, p))
        .max()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::run_amplifiers;

    #[test]
    fn check_run_amplifiers() {
        assert_eq!(
            run_amplifiers(
                &[3, 15, 3, 16, 1002, 16, 10, 16, 1, 16, 15, 15, 4, 15, 99, 0, 0],
                &[4, 3, 2, 1, 0]
            ),
            43210
        );
        assert_eq!(
            run_amplifiers(
                &[
                    3, 23, 3, 24, 1002, 24, 10, 24, 1002, 23, -1, 23, 101, 5, 23, 23, 1, 24, 23,
                    23, 4, 23, 99, 0, 0
                ],
                &[0, 1, 2, 3, 4]
            ),
            54321
        );
        assert_eq!(
            run_amplifiers(
                &[
                    3, 31, 3, 32, 1002, 32, 10, 32, 1001, 31, -2, 31, 1007, 31, 0, 33, 1002, 33, 7,
                    33, 1, 33, 31, 31, 1, 32, 31, 31, 4, 31, 99, 0, 0, 0
                ],
                &[1, 0, 4, 3, 2]
            ),
            65210
        );
    }
}
