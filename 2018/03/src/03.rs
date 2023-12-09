use std::collections::HashMap;

aoc::parts!(1, 2);

#[derive(Debug, PartialEq)]
struct Claim {
    id: usize,
    x: usize,
    y: usize,
    width: usize,
    height: usize,
}

mod parser {
    use std::str::FromStr;

    use super::Claim;
    use nom::bytes::complete::tag;
    use nom::character::complete::digit1;
    use nom::combinator::map;
    use nom::sequence::{preceded, separated_pair};

    impl FromStr for Claim {
        type Err = String;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            let parse_point = separated_pair(
                map(digit1::<&str, ()>, |s: &str| s.parse().unwrap()),
                tag(","),
                map(digit1::<&str, ()>, |s: &str| s.parse().unwrap()),
            );
            let parse_area = separated_pair(
                map(digit1::<&str, ()>, |s: &str| s.parse().unwrap()),
                tag("x"),
                map(digit1::<&str, ()>, |s: &str| s.parse().unwrap()),
            );

            let mut parse_claim = separated_pair(
                preceded(tag("#"), map(digit1::<&str, ()>, |s: &str| s.parse().unwrap())),
                tag(" @ "),
                separated_pair(parse_point, tag(": "), parse_area),
            );

            let (id, ((x, y), (width, height))) = parse_claim(s).unwrap().1;
            Ok(Claim {
                id,
                x,
                y,
                width,
                height,
            })
        }
    }
}

fn parse_input(input: &str) -> Vec<Claim> {
    input.lines().map(|line| line.parse().unwrap()).collect()
}

fn part_1(input: aoc::Input) -> impl ToString {
    let claims = &parse_input(input.raw());

    let mut fabric = vec![[0usize; 1000]; 1000];

    for claim in claims {
        for row in fabric.iter_mut().skip(claim.x).take(claim.width) {
            for cell in row.iter_mut().skip(claim.y).take(claim.height) {
                *cell += 1;
            }
        }
    }

    fabric.iter().flat_map(|i| i.iter()).filter(|&&x| x > 1).count()
}

fn part_2(input: aoc::Input) -> impl ToString {
    let claims = &parse_input(input.raw());

    let mut fabric = HashMap::new();

    for claim in claims {
        for x in claim.x..claim.x + claim.width {
            for y in claim.y..claim.y + claim.height {
                fabric.entry((x, y)).or_insert_with(Vec::new).push(claim.id);
            }
        }
    }

    'claim: for claim in claims {
        for x in claim.x..claim.x + claim.width {
            for y in claim.y..claim.y + claim.height {
                if fabric.entry((x, y)).or_default().len() != 1 {
                    continue 'claim;
                }
            }
        }
        return claim.id;
    }

    0
}
