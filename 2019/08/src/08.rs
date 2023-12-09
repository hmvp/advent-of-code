use bytecount::count;

aoc::parts!(1, 2);

pub fn input_generator(input: &str) -> Vec<u8> {
    input.chars().map(|c| c.to_digit(10).unwrap() as u8).collect()
}

fn part_1(input: aoc::Input) -> impl ToString {
    let input = &input_generator(input.raw());

    let layer = input
        .chunks(25 * 6)
        .map(|chunk| (count(chunk, 0), chunk))
        .fold((25 * 6, None), |(acc_nr, s), (nr, chunk)| {
            if nr < acc_nr {
                (nr, Some(chunk))
            } else {
                (acc_nr, s)
            }
        })
        .1
        .unwrap();
    count(layer, 1) * count(layer, 2)
}

fn part_2(input: aoc::Input) -> impl ToString {
    const SIZE: usize = 25 * 6;

    let input = &input_generator(input.raw());

    let picture = input.chunks(SIZE).fold([2_u8; SIZE], |mut picture, layer| {
        for i in 0..SIZE {
            if picture[i] == 2 {
                picture[i] = layer[i];
            }
        }
        picture
    });
    picture.chunks(25).for_each(|row| {
        row.iter().for_each(|&c| if c == 1 { print!("X") } else { print!(" ") });
        println!();
    });
    0
}
