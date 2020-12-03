use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day3)]
pub fn input_generator(input: &str) -> Vec<Vec<bool>> {
    input
        .lines()
        .map(|l| l.chars().map(|c| c == '#').collect())
        .collect()
}

fn check_slope(input: &[Vec<bool>], delta_x: usize, delta_y: usize) -> i32{
    let mut x = 0;
    let mut nr_of_trees = 0;
    for line in input.iter().step_by(delta_y) {
        if x >= line.len() {
            x -= line.len()
        }
        if line[x] {
            nr_of_trees += 1
        }
        x += delta_x;
    }
    nr_of_trees
}

#[aoc(day3, part1)]
pub fn part1(input: &[Vec<bool>]) -> i32 {
    check_slope(input, 3, 1)
}

#[aoc(day3, part2)]
pub fn part2(input: &[Vec<bool>]) -> i32 {
    let slope1 = check_slope(input, 1, 1);
    let slope2 = check_slope(input, 3, 1);
    let slope3 = check_slope(input, 5, 1);
    let slope4 = check_slope(input, 7, 1);
    let slope5 = check_slope(input, 1, 2);
    slope1 * slope2 * slope3 * slope4 * slope5
}

#[cfg(test)]
mod tests {
    use super::{input_generator, part1, part2};

    const TEST_INPUT: &str = "..##.........##.........##.........##.........##.........##.......
#...#...#..#...#...#..#...#...#..#...#...#..#...#...#..#...#...#..
.#....#..#..#....#..#..#....#..#..#....#..#..#....#..#..#....#..#.
..#.#...#.#..#.#...#.#..#.#...#.#..#.#...#.#..#.#...#.#..#.#...#.#
.#...##..#..#...##..#..#...##..#..#...##..#..#...##..#..#...##..#.
..#.##.......#.##.......#.##.......#.##.......#.##.......#.##.....
.#.#.#....#.#.#.#....#.#.#.#....#.#.#.#....#.#.#.#....#.#.#.#....#
.#........#.#........#.#........#.#........#.#........#.#........#
#.##...#...#.##...#...#.##...#...#.##...#...#.##...#...#.##...#...
#...##....##...##....##...##....##...##....##...##....##...##....#
.#..#...#.#.#..#...#.#.#..#...#.#.#..#...#.#.#..#...#.#.#..#...#.#";

    #[test]
    fn check_input_generator() {
        let expected: Vec<Vec<bool>> = vec![
            vec![
                false, false, true, true, false, false, false, false, false, false, false, false,
                false, true, true, false, false, false, false, false, false, false, false, false,
                true, true, false, false, false, false, false, false, false, false, false, true,
                true, false, false, false, false, false, false, false, false, false, true, true,
                false, false, false, false, false, false, false, false, false, true, true, false,
                false, false, false, false, false, false,
            ],
            vec![
                true, false, false, false, true, false, false, false, true, false, false, true,
                false, false, false, true, false, false, false, true, false, false, true, false,
                false, false, true, false, false, false, true, false, false, true, false, false,
                false, true, false, false, false, true, false, false, true, false, false, false,
                true, false, false, false, true, false, false, true, false, false, false, true,
                false, false, false, true, false, false,
            ],
            vec![
                false, true, false, false, false, false, true, false, false, true, false, false,
                true, false, false, false, false, true, false, false, true, false, false, true,
                false, false, false, false, true, false, false, true, false, false, true, false,
                false, false, false, true, false, false, true, false, false, true, false, false,
                false, false, true, false, false, true, false, false, true, false, false, false,
                false, true, false, false, true, false,
            ],
            vec![
                false, false, true, false, true, false, false, false, true, false, true, false,
                false, true, false, true, false, false, false, true, false, true, false, false,
                true, false, true, false, false, false, true, false, true, false, false, true,
                false, true, false, false, false, true, false, true, false, false, true, false,
                true, false, false, false, true, false, true, false, false, true, false, true,
                false, false, false, true, false, true,
            ],
            vec![
                false, true, false, false, false, true, true, false, false, true, false, false,
                true, false, false, false, true, true, false, false, true, false, false, true,
                false, false, false, true, true, false, false, true, false, false, true, false,
                false, false, true, true, false, false, true, false, false, true, false, false,
                false, true, true, false, false, true, false, false, true, false, false, false,
                true, true, false, false, true, false,
            ],
            vec![
                false, false, true, false, true, true, false, false, false, false, false, false,
                false, true, false, true, true, false, false, false, false, false, false, false,
                true, false, true, true, false, false, false, false, false, false, false, true,
                false, true, true, false, false, false, false, false, false, false, true, false,
                true, true, false, false, false, false, false, false, false, true, false, true,
                true, false, false, false, false, false,
            ],
            vec![
                false, true, false, true, false, true, false, false, false, false, true, false,
                true, false, true, false, true, false, false, false, false, true, false, true,
                false, true, false, true, false, false, false, false, true, false, true, false,
                true, false, true, false, false, false, false, true, false, true, false, true,
                false, true, false, false, false, false, true, false, true, false, true, false,
                true, false, false, false, false, true,
            ],
            vec![
                false, true, false, false, false, false, false, false, false, false, true, false,
                true, false, false, false, false, false, false, false, false, true, false, true,
                false, false, false, false, false, false, false, false, true, false, true, false,
                false, false, false, false, false, false, false, true, false, true, false, false,
                false, false, false, false, false, false, true, false, true, false, false, false,
                false, false, false, false, false, true,
            ],
            vec![
                true, false, true, true, false, false, false, true, false, false, false, true,
                false, true, true, false, false, false, true, false, false, false, true, false,
                true, true, false, false, false, true, false, false, false, true, false, true,
                true, false, false, false, true, false, false, false, true, false, true, true,
                false, false, false, true, false, false, false, true, false, true, true, false,
                false, false, true, false, false, false,
            ],
            vec![
                true, false, false, false, true, true, false, false, false, false, true, true,
                false, false, false, true, true, false, false, false, false, true, true, false,
                false, false, true, true, false, false, false, false, true, true, false, false,
                false, true, true, false, false, false, false, true, true, false, false, false,
                true, true, false, false, false, false, true, true, false, false, false, true,
                true, false, false, false, false, true,
            ],
            vec![
                false, true, false, false, true, false, false, false, true, false, true, false,
                true, false, false, true, false, false, false, true, false, true, false, true,
                false, false, true, false, false, false, true, false, true, false, true, false,
                false, true, false, false, false, true, false, true, false, true, false, false,
                true, false, false, false, true, false, true, false, true, false, false, true,
                false, false, false, true, false, true,
            ],
        ];
        assert_eq!(input_generator(TEST_INPUT), expected);
    }

    #[test]
    fn check_part1() {
        let input = input_generator(TEST_INPUT);
        assert_eq!(part1(&input), 7);
    }
    #[test]
    fn check_part2() {
        let input = input_generator(TEST_INPUT);
        assert_eq!(part2(&input), 336);
    }
}
