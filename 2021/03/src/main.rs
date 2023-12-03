use aoc::Input;

aoc::parts!(1, 2);

fn part_1(input: Input) -> impl ToString {
    let input: Vec<Vec<u8>> = input
    .lines()
    .map(|l| l.chars().map(|c| c.to_digit(10).unwrap() as u8).collect())
    .collect();
    let l = input.len();
    let dl = input[0].len() as u32;
    let mut counts = vec![0u32; dl as usize];
    for n in input {
        for (index, d) in n.iter().enumerate() {
            counts[index] += u32::from(*d);
        }
    }

    let gamma: u32 = counts.iter().fold(0, |mut acc, d| {
        acc += u32::from(*d as usize > l / 2);
        acc <<= 1;
        acc
    }) >> 1;
    let epsilon = !gamma & (2u32.pow(dl) - 1);
    assert_eq!(gamma & epsilon, 0);
    println!("{gamma:b} {epsilon:b} {gamma:?} {epsilon:?}",);

    gamma * epsilon
}

fn part_2(input: Input) -> impl ToString {
    let input: Vec<Vec<u8>> = input
    .lines()
    .map(|l| l.chars().map(|c| c.to_digit(10).unwrap() as u8).collect())
    .collect();
    let oxygen = u32::from(calc(&input, false));
    let co2 = u32::from(calc(&input, true));

    println!("{oxygen:b} {co2:b} {oxygen:?} {co2:?}");
    oxygen * co2
}



fn filter_rec(mut input: Vec<Vec<u8>>, index: usize, co2: bool) -> Vec<Vec<u8>> {
    let l = input.len() as f32;
    let mut count = 0;
    for n in &input {
        count += u32::from(n[index]);
    }
    let test = if count as f32 >= l / 2f32 {
        u8::from(!co2)
    } else { u8::from(co2) };

    // println!("{:?} {:?} {:?} {:?}", count, test, l, count as f32 >= l / 2f32);
    // if co2 {
    //     test = !test & 1
    // }

    input.drain(..).filter(|i| i[index] == test).collect()
}

fn calc(input: &[Vec<u8>], co2: bool) -> u8 {
    let mut input = input.to_vec();
    let dl = input[0].len();

    for i in 0usize..dl {
        input = filter_rec(input, i, co2);
        // println!("{:?}", input);

        if input.len() == 1 {
            break;
        }
    }

    input[0].iter().fold(0, |mut acc, d| {
        acc += *d;
        acc <<= 1;
        acc
    }) >> 1
}
