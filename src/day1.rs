use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day1)]
pub fn input_generator(input: &str) -> Vec<i32> {
    input.lines().map(|l| l.parse().unwrap()).collect()
}

#[aoc(day1, part1)]
pub fn part1(input: &[i32]) -> i32 {
    let mut increased = 0;
    let mut last = &i32::MAX;
    for x in input {
        if x > last {
            increased += 1;
        }
        last = x;
    }
    increased
}

#[aoc(day1, part2)]
pub fn part2(input: &[i32]) -> i32 {
    let mut increased = 0;
    let mut last = i32::MAX;
    for x in input.windows(3) {
        let sum: i32 = x.iter().sum();
        if sum > last {
            increased += 1;
        }
        last = sum;
    }
    increased
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_part1() {
        assert_eq!(
            part1(&[199, 200, 208, 210, 200, 207, 240, 269, 260, 263]),
            7
        );
    }
    #[test]
    fn check_part2() {
        assert_eq!(part2(&[199, 200, 208, 210, 200, 207, 240, 269, 260, 263]), 5);
    }
}
