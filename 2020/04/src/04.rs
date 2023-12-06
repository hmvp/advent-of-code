use nom::{
    branch::alt,
    bytes::complete::{is_not, tag, take_while_m_n},
    character::complete::digit1,
    multi::separated_list1,
    sequence::{pair, preceded},
    IResult,
};
use std::collections::hash_map::HashMap;
use std::iter::FromIterator;
use strum::{EnumIter, IntoEnumIterator};

aoc::parts!(1, 2);

fn part_1(input: aoc::Input) -> impl ToString {
    let input = &input_generator(input);

    let mut nr_of_valid = 0;
    'outer: for p in input {
        for f in Field::iter() {
            if f != Field::CID && !p.contains_key(&f) {
                continue 'outer;
            }
        }

        nr_of_valid += 1;
    }
    nr_of_valid
}

#[allow(clippy::needless_continue)]
fn part_2(input: aoc::Input) -> impl ToString {
    let input = &input_generator(input);

    let mut nr_of_valid = 0;
    'outer: for p in input {
        for f in Field::iter() {
            if f != Field::CID && !p.contains_key(&f) {
                continue 'outer;
            }
        }

        // byr (Birth Year) - four digits; at least 1920 and at most 2002.
        if p.get(&Field::BYR)
            .unwrap()
            .parse::<usize>()
            .ok()
            .map_or(false, |i| !(1920..=2002).contains(&i))
        {
            continue 'outer;
        }

        // iyr (Issue Year) - four digits; at least 2010 and at most 2020.
        if p.get(&Field::IYR)
            .unwrap()
            .parse::<usize>()
            .ok()
            .map_or(false, |i| !(2010..=2020).contains(&i))
        {
            continue 'outer;
        }
        // eyr (Expiration Year) - four digits; at least 2020 and at most 2030.
        if p.get(&Field::EYR)
            .unwrap()
            .parse::<usize>()
            .ok()
            .map_or(false, |i| !(2020..=2030).contains(&i))
        {
            continue 'outer;
        }

        // hgt (Height) - a number followed by either cm or in:
        //     If cm, the number must be at least 150 and at most 193.
        //     If in, the number must be at least 59 and at most 76.
        let result: IResult<&str, (&str, &str)> =
            pair(digit1, alt((tag("cm"), tag("in"))))(p.get(&Field::HGT).unwrap().as_str());
        if let Ok((rest, (height, metric))) = result {
            if !rest.is_empty() {
                continue 'outer;
            }
            let height: usize = height.parse().unwrap();

            if metric == "cm" {
                if !(150..=193).contains(&height) {
                    continue 'outer;
                }
            } else if !(59..=76).contains(&height) {
                continue 'outer;
            }
        } else {
            continue 'outer;
        }

        // hcl (Hair Color) - a # followed by exactly six characters 0-9 or a-f.
        let result: IResult<&str, &str> =
            preceded(tag("#"), take_while_m_n(6, 6, is_hex_digit))(p.get(&Field::HCL).unwrap().as_str());
        if result.ok().map_or(true, |(rest, _)| !rest.is_empty()) {
            continue 'outer;
        }
        // ecl (Eye Color) - exactly one of: amb blu brn gry grn hzl oth.
        if !VALID_EYE_COLORS.contains(&p.get(&Field::ECL).unwrap().as_str()) {
            continue 'outer;
        }

        // pid (Passport ID) - a nine-digit number, including leading zeroes.
        if p.get(&Field::PID).unwrap().parse::<usize>().is_err() || p.get(&Field::PID).unwrap().len() != 9 {
            continue 'outer;
        }

        // cid (Country ID) - ignored, missing or not.

        nr_of_valid += 1;
    }
    nr_of_valid
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, EnumIter)]
pub enum Field {
    BYR,
    IYR,
    EYR,
    HGT,
    HCL,
    ECL,
    PID,
    CID,
}

impl Field {
    fn get_tag(self) -> &'static str {
        match self {
            Field::BYR => "byr:",
            Field::IYR => "iyr:",
            Field::EYR => "eyr:",
            Field::HGT => "hgt:",
            Field::HCL => "hcl:",
            Field::ECL => "ecl:",
            Field::PID => "pid:",
            Field::CID => "cid:",
        }
    }
}

fn field(field: Field) -> impl Fn(&str) -> IResult<&str, (Field, String)> {
    move |input: &str| {
        preceded(tag(field.get_tag()), is_not(" \n"))(input)
            .map(|(rest, field_value)| (rest, (field, field_value.to_string())))
    }
}

fn passport(input: &str) -> IResult<&str, HashMap<Field, String>> {
    separated_list1(
        alt((tag(" "), tag("\n"))),
        alt((
            field(Field::BYR),
            field(Field::IYR),
            field(Field::EYR),
            field(Field::HGT),
            field(Field::HCL),
            field(Field::ECL),
            field(Field::PID),
            field(Field::CID),
        )),
    )(input)
    .map(|(rest, items)| (rest, HashMap::from_iter(items)))
}

fn passports(input: aoc::Input) -> IResult<&str, Vec<HashMap<Field, String>>> {
    separated_list1(tag("\n\n"), passport)(input.raw())
}

pub fn input_generator(input: aoc::Input) -> Vec<HashMap<Field, String>> {
    passports(input).unwrap().1
}

const VALID_EYE_COLORS: [&str; 7] = ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"];

fn is_hex_digit(c: char) -> bool {
    c.is_ascii_hexdigit()
}
