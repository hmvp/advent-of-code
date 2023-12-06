aoc::parts!(1, 2);

fn part_1(input: aoc::Input) -> impl ToString {
    let input: Vec<usize> = input.lines().map(|l| l.parse().unwrap()).collect();

    for x in &input {
        for y in &input {
            if x + y == 2020 {
                return x * y;
            }
        }
    }
    0
}

fn part_2(input: aoc::Input) -> impl ToString {
    let input: Vec<usize> = input.lines().map(|l| l.parse().unwrap()).collect();

    for x in &input {
        for y in &input {
            for z in &input {
                if x + y + z == 2020 {
                    return x * y * z;
                }
            }
        }
    }
    0
}
