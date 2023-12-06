aoc::parts!(1, 2);

pub fn input_generator(input: aoc::Input) -> Vec<Vec<bool>> {
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

fn part_1(input: aoc::Input) -> impl ToString {
    let input = &input_generator(input);

    check_slope(input, 3, 1)

}

fn part_2(input: aoc::Input) -> impl ToString {
    let input = &input_generator(input);

    let slope1 = check_slope(input, 1, 1);
    let slope2 = check_slope(input, 3, 1);
    let slope3 = check_slope(input, 5, 1);
    let slope4 = check_slope(input, 7, 1);
    let slope5 = check_slope(input, 1, 2);
    slope1 * slope2 * slope3 * slope4 * slope5
}


