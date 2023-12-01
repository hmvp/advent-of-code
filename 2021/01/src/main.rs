use aoc::Input;

aoc::parts!(1, 2);

fn part_1(input: Input) -> impl ToString {
    let input: Vec<u32> = input.lines().map(|l| l.parse().unwrap()).collect();
    let mut increased = 0;
    let mut last = u32::MAX;
    for x in input {
        if x > last {
            increased += 1;
        }
        last = x;
    }
    increased
}

fn part_2(input: Input) -> impl ToString {
    let input: Vec<u32> = input.lines().map(|l| l.parse().unwrap()).collect();

    let mut increased = 0;
    let mut last = u32::MAX;
    for x in input.windows(3) {
        let sum: u32 = (*x).iter().sum();
        if sum > last {
            increased += 1;
        }
        last = sum;
    }
    increased
}
