aoc::parts!(1, 2);

pub fn input_generator(input: &str) -> Vec<usize> {
    input.split(',').map(|l| l.parse().unwrap()).collect()
}

fn compute(input: &mut [usize]) -> usize {
    for i in 0..input.len() {
        if i % 4 != 0 {
            continue;
        }

        match input[i] {
            1 => {
                let output = input[i + 3];
                input[output] = input[input[i + 1]] + input[input[i + 2]];
            }
            2 => {
                let output = input[i + 3];
                input[output] = input[input[i + 1]] * input[input[i + 2]];
            }
            99 => break,
            x => {
                dbg!(x);
                dbg!(i);
                unreachable!();
            }
        }
    }

    input[0]
}

fn part_1(input: aoc::Input) -> impl ToString {
    let input = &input_generator(input.raw());

    let mut data = vec![0; input.len()];
    data.clone_from_slice(input);

    data[1] = 12;
    data[2] = 2;

    compute(&mut data)
}

fn part_2(input: aoc::Input) -> impl ToString {
    let input = &input_generator(input.raw());

    let mut data = vec![0; input.len()];
    for noun in 0..99 {
        for verb in 0..99 {
            data.clone_from_slice(input);

            data[1] = noun;
            data[2] = verb;
            if compute(&mut data) == 19_690_720 {
                return noun * 100 + verb;
            }
        }
    }
    unreachable!();
}

#[cfg(test)]
mod tests {
    use super::compute;

    #[test]
    fn check_compute() {
        assert_eq!(compute(&mut [1, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50]), 3500);
        assert_eq!(compute(&mut [1, 0, 0, 0, 99]), 2);
        assert_eq!(compute(&mut [2, 3, 0, 3, 99]), 2);
        assert_eq!(compute(&mut [1, 1, 1, 4, 99, 5, 6, 0, 99]), 30);
    }
}
