use compute::simple;

aoc::parts!(1, 2);

pub fn input_generator(input: &str) -> Vec<isize> {
    input.split(',').map(|l| l.parse().unwrap()).collect()
}

fn part_1(input: aoc::Input) -> impl ToString {
    let program = &input_generator(input.raw());

    let output = simple(program, &[1]);

    *output.last().unwrap()
}

fn part_2(input: aoc::Input) -> impl ToString {
    let program = &input_generator(input.raw());

    let output = simple(program, &[5]);

    *output.last().unwrap()
}

#[cfg(test)]
mod tests {
    use super::simple as compute;

    #[test]
    fn check_compute_equals_pos() {
        let program = [3, 9, 8, 9, 10, 9, 4, 9, 99, -1, 8];
        let output = compute(&program, &[1]);
        assert_eq!(output[0], 0);

        let output = compute(&program, &[8]);
        assert_eq!(output[0], 1);
    }
    #[test]
    fn check_compute_less_than_pos() {
        let program = [3, 9, 7, 9, 10, 9, 4, 9, 99, -1, 8];
        let output = compute(&program, &[9]);
        assert_eq!(output[0], 0);

        let output = compute(&program, &[7]);
        assert_eq!(output[0], 1);

        let output = compute(&program, &[8]);
        assert_eq!(output[0], 0);
    }
    #[test]
    fn check_compute_equals_immediate() {
        let program = [3, 3, 1108, -1, 8, 3, 4, 3, 99];
        let output = compute(&program, &[1]);
        assert_eq!(output[0], 0);

        let output = compute(&program, &[8]);
        assert_eq!(output[0], 1);
    }
    #[test]
    fn check_compute_less_than_immediate() {
        let program = [3, 3, 1107, -1, 8, 3, 4, 3, 99];
        let output = compute(&program, &[9]);
        assert_eq!(output[0], 0);

        let output = compute(&program, &[7]);
        assert_eq!(output[0], 1);

        let output = compute(&program, &[8]);
        assert_eq!(output[0], 0);
    }
    #[test]
    fn check_compute_jmp_pos() {
        let program = [3, 12, 6, 12, 15, 1, 13, 14, 13, 4, 13, 99, -1, 0, 1, 9];
        let output = compute(&program, &[0]);
        assert_eq!(output[0], 0);

        let output = compute(&program, &[2]);
        assert_eq!(output[0], 1);
    }
    #[test]
    fn check_compute_jmp_immediate() {
        let program = [3, 3, 1105, -1, 9, 1101, 0, 0, 12, 4, 12, 99, 1];
        let output = compute(&program, &[0]);
        assert_eq!(output[0], 0);

        let output = compute(&program, &[2]);
        assert_eq!(output[0], 1);
    }
    #[test]
    fn check_compute_big() {
        let program = [
            3, 21, 1008, 21, 8, 20, 1005, 20, 22, 107, 8, 21, 20, 1006, 20, 31, 1106, 0, 36, 98, 0, 0, 1002, 21, 125,
            20, 4, 20, 1105, 1, 46, 104, 999, 1105, 1, 46, 1101, 1000, 1, 20, 4, 20, 1105, 1, 46, 98, 99,
        ];
        let output = compute(&program, &[7]);
        assert_eq!(output[0], 999);

        let output = compute(&program, &[8]);
        assert_eq!(output[0], 1000);

        let output = compute(&program, &[9]);
        assert_eq!(output[0], 1001);
    }
}
