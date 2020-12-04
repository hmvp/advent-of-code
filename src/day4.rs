use aoc_runner_derive::{aoc, aoc_generator};
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

fn passports(input: &str) -> IResult<&str, Vec<HashMap<Field, String>>> {
    separated_list1(tag("\n\n"), passport)(input)
}

#[aoc_generator(day4)]
pub fn input_generator(input: &str) -> Vec<HashMap<Field, String>> {
    passports(input).unwrap().1
}

#[aoc(day4, part1)]
pub fn part1(input: &[HashMap<Field, String>]) -> i32 {
    let mut nr_of_valid = 0;
    'outer: for p in input {
        for f in Field::iter() {
            if f != Field::CID && !p.contains_key(&f) {
                continue 'outer;
            }
        }

        nr_of_valid += 1
    }
    nr_of_valid
}

const VALID_EYE_COLORS: [&str; 7] = ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"];

fn is_hex_digit(c: char) -> bool {
    c.is_digit(16)
}

#[aoc(day4, part2)]
pub fn part2(input: &[HashMap<Field, String>]) -> i32 {
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
            .map_or(false, |i| i < 1920 || i > 2002)
        {
            continue 'outer;
        }

        // iyr (Issue Year) - four digits; at least 2010 and at most 2020.
        if p.get(&Field::IYR)
            .unwrap()
            .parse::<usize>()
            .ok()
            .map_or(false, |i| i < 2010 || i > 2020)
        {
            continue 'outer;
        }
        // eyr (Expiration Year) - four digits; at least 2020 and at most 2030.
        if p.get(&Field::EYR)
            .unwrap()
            .parse::<usize>()
            .ok()
            .map_or(false, |i| i < 2020 || i > 2030)
        {
            continue 'outer;
        }

        // hgt (Height) - a number followed by either cm or in:
        //     If cm, the number must be at least 150 and at most 193.
        //     If in, the number must be at least 59 and at most 76.
        let result: IResult<&str, (&str, &str)> =
            pair(digit1, alt((tag("cm"), tag("in"))))(p.get(&Field::HGT).unwrap().as_str());
        if let Ok((rest, (height, metric))) = result {
            if rest != "" {
                continue 'outer;
            }
            let height: usize = height.parse().unwrap();

            if metric == "cm" {
                if height < 150 || height > 193 {
                    continue 'outer;
                }
            } else if height < 59 || height > 76 {
                continue 'outer;
            }
        } else {
            continue 'outer;
        }

        // hcl (Hair Color) - a # followed by exactly six characters 0-9 or a-f.
        let result: IResult<&str, &str> = preceded(tag("#"), take_while_m_n(6, 6, is_hex_digit))(
            p.get(&Field::HCL).unwrap().as_str(),
        );
        if result.ok().map_or(true, |(rest, value)| rest != "") {
            continue 'outer;
        }
        // ecl (Eye Color) - exactly one of: amb blu brn gry grn hzl oth.
        if !VALID_EYE_COLORS.contains(&p.get(&Field::ECL).unwrap().as_str()) {
            continue 'outer;
        }

        // pid (Passport ID) - a nine-digit number, including leading zeroes.
        if p.get(&Field::PID).unwrap().parse::<usize>().is_err()
            || p.get(&Field::PID).unwrap().len() != 9
        {
            continue 'outer;
        }

        // cid (Country ID) - ignored, missing or not.

        nr_of_valid += 1
    }
    nr_of_valid
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
byr:1937 iyr:2017 cid:147 hgt:183cm

iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884
hcl:#cfa07d byr:1929

hcl:#ae17e1 iyr:2013
eyr:2024
ecl:brn pid:760753108 byr:1931
hgt:179cm

hcl:#cfa07d eyr:2025 pid:166559648
iyr:2011 ecl:brn hgt:59in";

    #[test]
    fn check_input_generator() {
        let expected: Vec<HashMap<Field, String>> = vec![
            HashMap::from_iter(vec![
                (Field::HCL, "#fffffd".to_string()),
                (Field::BYR, "1937".to_string()),
                (Field::HGT, "183cm".to_string()),
                (Field::CID, "147".to_string()),
                (Field::IYR, "2017".to_string()),
                (Field::PID, "860033327".to_string()),
                (Field::EYR, "2020".to_string()),
                (Field::ECL, "gry".to_string()),
            ]),
            HashMap::from_iter(vec![
                (Field::HCL, "#cfa07d".to_string()),
                (Field::BYR, "1929".to_string()),
                (Field::CID, "350".to_string()),
                (Field::IYR, "2013".to_string()),
                (Field::PID, "028048884".to_string()),
                (Field::EYR, "2023".to_string()),
                (Field::ECL, "amb".to_string()),
            ]),
            HashMap::from_iter(vec![
                (Field::HCL, "#ae17e1".to_string()),
                (Field::BYR, "1931".to_string()),
                (Field::HGT, "179cm".to_string()),
                (Field::IYR, "2013".to_string()),
                (Field::PID, "760753108".to_string()),
                (Field::EYR, "2024".to_string()),
                (Field::ECL, "brn".to_string()),
            ]),
            HashMap::from_iter(vec![
                (Field::HCL, "#cfa07d".to_string()),
                (Field::HGT, "59in".to_string()),
                (Field::IYR, "2011".to_string()),
                (Field::PID, "166559648".to_string()),
                (Field::EYR, "2025".to_string()),
                (Field::ECL, "brn".to_string()),
            ]),
        ];
        assert_eq!(input_generator(TEST_INPUT), expected);
    }

    #[test]
    fn check_part1() {
        let input = input_generator(TEST_INPUT);
        assert_eq!(part1(&input), 2);
    }
    #[test]
    fn check_part2() {
        let input = input_generator(
            "eyr:1972 cid:100
hcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926

iyr:2019
hcl:#602927 eyr:1967 hgt:170cm
ecl:grn pid:012533040 byr:1946

hcl:dab227 iyr:2012
ecl:brn hgt:182cm pid:021572410 eyr:2020 byr:1992 cid:277

hgt:59cm ecl:zzz
eyr:2038 hcl:74454a iyr:2023
pid:3556412378 byr:2007

pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980
hcl:#623a2f

eyr:2029 ecl:blu cid:129 byr:1989
iyr:2014 pid:896056539 hcl:#a97842 hgt:165cm

hcl:#888785
hgt:164cm byr:2001 iyr:2015 cid:88
pid:545766238 ecl:hzl
eyr:2022

iyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:093154719",
        );
        assert_eq!(part2(&input), 4);
    }
}
