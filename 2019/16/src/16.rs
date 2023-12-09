use std::iter::repeat;

aoc::parts!(1, 2);

pub fn input_generator(input: &str) -> Vec<u8> {
    input.chars().map(|c| c.to_digit(10).unwrap() as u8).collect()
}

const BASE_PATTERN: [isize; 4] = [0, 1, 0, -1];

pub fn make_pattern(nr: usize) -> impl Iterator<Item = isize> {
    BASE_PATTERN
        .iter()
        .flat_map(move |i| repeat(*i).take(nr))
        .cycle()
        .skip_while(|&x| x == 0)
}

pub fn calc_list(input: &[u8]) -> Vec<u8> {
    (1..=input.len())
        .map(|n| {
            (input
                .iter()
                .skip(n - 1)
                .zip(make_pattern(n))
                .map(|(a, b)| *a as isize * b)
                .sum::<isize>()
                .abs() as usize
                % 10) as u8
        })
        .collect()
}

pub fn fft(input: &[u8], iterations: usize, offset: usize) -> String {
    let mut list: Vec<u8> = input.to_vec();
    for _ in 0..iterations {
        list = calc_list(&list);
    }

    format!(
        "{:08}",
        list.iter()
            .skip(offset)
            .take(8)
            .fold(0, |acc, i| acc * 10 + (*i as usize))
    )
}

fn part_1(input: aoc::Input) -> impl ToString {
    let input = &input_generator(input.raw());

    fft(input, 100, 0)
}

fn part_2(input: aoc::Input) -> impl ToString {
    let input = &input_generator(input.raw());

    let signal: Vec<u8> = repeat(input).take(10000).flatten().copied().collect();
    fft(
        &signal,
        1,
        input.iter().take(7).fold(0, |acc, i| acc * 10 + (*i as usize)),
    )
}

#[cfg(test)]
mod tests {
    use super::calc_list;
    use super::fft;
    use super::input_generator;

    #[test]
    fn check_calc_list() {
        let result = calc_list(&[1, 2, 3, 4, 5, 6, 7, 8]);
        assert_eq!(result, vec![4, 8, 2, 2, 6, 1, 5, 8]);
        let result = calc_list(&result);
        assert_eq!(result, vec![3, 4, 0, 4, 0, 4, 3, 8]);
        let result = calc_list(&result);
        assert_eq!(result, vec![0, 3, 4, 1, 5, 5, 1, 8]);
        let result = calc_list(&result);
        assert_eq!(result, vec![0, 1, 0, 2, 9, 4, 9, 8]);

        assert_eq!(fft(&[1, 2, 3, 4, 5, 6, 7, 8], 4, 0), "01029498");
    }

    // #[test]
    // fn check_part1() {
    //     assert_eq!(
    //         part1(&input_generator("80871224585914546619083218645595")),
    //         "24176176"
    //     );
    //     assert_eq!(
    //         part1(&input_generator("19617804207202209144916044189917")),
    //         "73745418"
    //     );
    //     assert_eq!(
    //         part1(&input_generator("69317163492948606335995924319873")),
    //         "52432133"
    //     );
    // }
}
