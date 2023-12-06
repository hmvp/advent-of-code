use nom::{
    bytes::complete::tag,
    character::complete::{alpha1, digit1},
    IResult,
};


aoc::parts!(1, 2);

fn part_1(input: aoc::Input) -> impl ToString {
    let input: Vec<_> = input.lines().map(|l| password_line(l).unwrap().1).collect();

    let mut valid_passwords = 0;
    for (min, max, khar, password) in &input {
        let nr_of_chars = password.matches(*khar).count();
        if nr_of_chars >= *min && nr_of_chars <= *max {
            valid_passwords += 1;
        }
    }
    valid_passwords
}

fn part_2(input: aoc::Input) -> impl ToString {
    let input: Vec<_> = input.lines().map(|l| password_line(l).unwrap().1).collect();

    let mut valid_passwords = 0;
    for (first, second, khar, password) in &input {
        let nr_ofmatches = password
            .char_indices()
            .filter(|(index, c)| c == khar && (*index == (first-1)|| *index == (second-1)))
            .count();
        if nr_ofmatches == 1 {
            valid_passwords += 1;
        }
    }
    valid_passwords
}



fn password_line(input: &str) -> IResult<&str, (usize, usize, char, String)> {
    let (input, min_str) = digit1(input)?;
    let (input, _) = tag("-")(input)?;
    let (input, max_str) = digit1(input)?;
    let (input, _) = tag(" ")(input)?;
    let (input, char_str) = alpha1(input)?;
    let (password, _) = tag(": ")(input)?;

    Ok((
        "",
        (
            min_str.parse().unwrap(),
            max_str.parse().unwrap(),
            char_str.chars().next().unwrap(),
            password.to_string(),
        ),
    ))
}
