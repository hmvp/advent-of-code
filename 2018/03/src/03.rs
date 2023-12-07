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
    use nom::{digit, do_parse, map_res, named, non_empty, tag};

    fn from_dec(input: &str) -> Result<usize, std::num::ParseIntError> {
        input.parse::<usize>()
    }

    named!(number<&str, usize>,
        map_res!(digit, from_dec)
    );

    named!(claim<&str, Claim>,
        do_parse!(
            tag!("#")   >>
            id:   number >>
            tag!(" @ ")   >>
            x: number >>
            tag!(",")   >>
            y:  number >>
            tag!(": ")   >>
            width:  number >>
            tag!("x")   >>
            height:  map_res!(non_empty, from_dec) >>
            (Claim { id, x, y, width, height })
        )
    );

    impl FromStr for Claim {
        type Err = String;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            match claim(s) {
                Ok(value) => Ok(value.1),
                Err(e) => Err(format!("Failed on \"{s}\": {e}")),
            }
        }
    }
}

fn parse_input(input: &str) -> Vec<Claim> {
    input.lines().map(|line| line.parse().unwrap()).collect()
}

fn part_1(input: aoc::Input) -> impl ToString {
    let claims = &parse_input(input.raw());

    let mut fabric = [[0usize; 1000]; 1000];

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
