aoc::parts!(1, 2);

pub fn input_generator(input: &str) -> Vec<i32> {
    input.lines().map(|l| l.parse().unwrap()).collect()
}

fn fuel_for_module(module: i32) -> i32 {
    module / 3 - 2
}

fn fuel_inclusieve(module: i32) -> i32 {
    let mut fuel = fuel_for_module(module);
    let mut total = 0;

    while fuel >= 0 {
        total += fuel;
        fuel = fuel_for_module(fuel);
    }

    total
}

fn part_1(input: aoc::Input) -> impl ToString {
    let input = &input_generator(input.raw());

    input.iter().copied().map(fuel_for_module).sum::<i32>()
}

fn part_2(input: aoc::Input) -> impl ToString {
    let input = &input_generator(input.raw());

    input.iter().copied().map(fuel_inclusieve).sum::<i32>()
}

#[cfg(test)]
mod tests {
    // use super::{part1, part2};

    // #[test]
    // fn check_numbers_are_calculated_correctly() {
    //     assert_eq!(part1(&[12]), 2);
    //     assert_eq!(part1(&[14]), 2);
    //     assert_eq!(part1(&[1969]), 654);
    //     assert_eq!(part1(&[100_756]), 33583);
    // }

    // #[test]
    // fn check_sums_correctly() {
    //     assert_eq!(part2(&[12, 14]), 4);
    // }
    // #[test]
    // fn check_part2() {
    //     assert_eq!(part2(&[9]), 1);
    //     assert_eq!(part2(&[14]), 2);
    //     assert_eq!(part2(&[12, 14]), 4);
    //     assert_eq!(part2(&[1969]), 966);
    //     assert_eq!(part2(&[100_756]), 50346);
    // }
}
