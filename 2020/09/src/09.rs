aoc::parts!(1, 2);

pub fn input_generator(input: aoc::Input) -> Vec<usize> {
    input.lines().flat_map(str::parse).collect()
}

fn find_outlier(input: &[usize], window_size: usize) -> usize {
    input
        .windows(window_size + 1)
        .find(|window| {
            let n = window[window_size];
            for x in &window[0..window_size] {
                for y in &window[0..window_size] {
                    if x != y && x + y == n {
                        return false;
                    }
                }
            }
            true
        })
        .map(|window| window[window_size])
        .unwrap()
}

fn part_1(input: aoc::Input) -> impl ToString {
    let input = &input_generator(input);

    find_outlier(input, 25)
}

fn find_parts_id(input: &[usize], outlier: usize) -> usize {
    let parts: Vec<usize> =
        input
            .iter()
            .fold(Vec::new(), |mut acc, i| match acc.iter().sum::<usize>() {
                sum if sum < outlier => {
                    acc.push(*i);
                    acc
                }
                sum if sum == outlier => acc,
                sum if sum > outlier => {
                    while acc.iter().sum::<usize>() > outlier {
                        acc.remove(0);
                    }
                    if acc.iter().sum::<usize>() != outlier {
                        acc.push(*i);
                    }
                    acc
                }
                _ => unreachable!(),
            });

    parts.iter().min().unwrap() + parts.iter().max().unwrap()
}

fn part_2(input: aoc::Input) -> impl ToString {
    let input = &input_generator(input);

    let outlier = find_outlier(input, 25);

    find_parts_id(input, outlier)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_find_outlier() {
        assert_eq!(
            find_outlier(
                &[
                    35, 20, 15, 25, 47, 40, 62, 55, 65, 95, 102, 117, 150, 182, 127, 219, 299, 277,
                    309, 576,
                ],
                5
            ),
            127
        );
    }

    #[test]
    fn check_find_parts_id() {
        assert_eq!(
            find_parts_id(
                &[
                    35, 20, 15, 25, 47, 40, 62, 55, 65, 95, 102, 117, 150, 182, 127, 219, 299, 277,
                    309, 576,
                ],
                127
            ),
            62
        );
    }
}

