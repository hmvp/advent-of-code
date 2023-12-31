use bytecount::count;
use ocr::Ocr;

aoc::parts!(1, 2);

pub fn input_generator(input: &str) -> Vec<u8> {
    input.chars().map(|c| u8::try_from(c.to_digit(10).unwrap()).unwrap()).collect()
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
    let mut result = String::new();
    picture.chunks(25).for_each(|row| {
        row.iter().for_each(|&c| if c == 1 { result.push('#') } else { result.push(' ') });
    });
    result.ocr(25)
}
