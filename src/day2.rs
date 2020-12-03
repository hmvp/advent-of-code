use aoc_runner_derive::{aoc, aoc_generator};
use nom::{
    bytes::complete::tag,
    character::complete::{alpha1, digit1},
    IResult,
};

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

#[aoc_generator(day2)]
pub fn input_generator(input: &str) -> Vec<(usize, usize, char, String)> {
    input.lines().map(|l| password_line(l).unwrap().1).collect()
}

#[aoc(day2, part1)]
pub fn part1(input: &[(usize, usize, char, String)]) -> i32 {
    let mut valid_passwords = 0;
    for (min, max, khar, password) in input {
        let nr_of_chars = password.matches(*khar).count();
        if nr_of_chars >= *min && nr_of_chars <= *max {
            valid_passwords += 1;
        }
    }
    valid_passwords
}

#[aoc(day2, part2)]
pub fn part2(input: &[(usize, usize, char, String)]) -> i32 {
    let mut valid_passwords = 0;
    for (first, second, khar, password) in input {
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

#[cfg(test)]
mod tests {
    use super::{part1, part2};

    #[test]
    fn check_part1() {
        assert_eq!(
            part1(&[
                (1, 3, 'a', "abcde".to_string()),
                (1, 3, 'b', "cdefg".to_string()),
                (2, 9, 'c', "ccccccccc".to_string())
            ]),
            2
        );
    }
    #[test]
    fn check_part2() {
        assert_eq!(
            part2(&[
                (1, 3, 'a', "abcde".to_string()),
                (1, 3, 'b', "cdefg".to_string()),
                (2, 9, 'c', "ccccccccc".to_string())
            ]),
            1
        );
    }
}
