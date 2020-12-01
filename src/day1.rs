use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day1)]
pub fn input_generator(input: &str) -> Vec<i32> {
    input.lines().map(|l| l.parse().unwrap()).collect()
}

#[aoc(day1, part1)]
pub fn part1(input: &[i32]) -> i32 {
    for x in input {
        for y in input {
            if x + y == 2020 {
                return x * y;
            }
        }
    }
    0
}

#[aoc(day1, part2)]
pub fn part2(input: &[i32]) -> i32 {
    for x in input {
        for y in input {
            for z in input {
                if x + y + z == 2020 {
                    return x * y * z;
                }
            }
      
        }
    }
    0
}

#[cfg(test)]
mod tests {
    use super::{part1, part2};

    #[test]
    fn check_part1() {
        assert_eq!(part1(&[1721, 979, 366, 299, 675, 1456]), 514579);
    }
    #[test]
    fn check_part2() {
        assert_eq!(part2(&[1721, 979, 366, 299, 675, 1456]), 241861950);
    }
}
