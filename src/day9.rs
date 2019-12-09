use super::compute::compute;
use aoc_runner_derive::{aoc, aoc_generator};
use crossbeam_channel::unbounded;

#[aoc_generator(day9)]
pub fn input_generator(input: &str) -> Vec<isize> {
    input.split(',').map(|l| l.parse().unwrap()).collect()
}

#[aoc(day9, part1)]
pub fn part1(program: &[isize]) -> isize {
    let (input_sender, input) = unbounded();
    let (output, output_receiver) = unbounded();

    input_sender.send(1).unwrap();

    compute(program, &input, &output);

    output_receiver.recv().unwrap()
}

#[aoc(day9, part2)]
pub fn part2(program: &[isize]) -> isize {
    let (input_sender, input) = unbounded();
    let (output, output_receiver) = unbounded();

    input_sender.send(2).unwrap();

    compute(program, &input, &output);

    output_receiver.try_recv().unwrap()
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
