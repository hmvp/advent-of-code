use std::collections::HashMap;

aoc::parts!(1);

#[derive(Debug, PartialEq)]
struct Point {
    x: usize,
    y: usize,
}

mod parser {
    use std::str::FromStr;

    use super::Point;
    use nom::{digit, do_parse, map_res, named, non_empty, tag};

    fn from_dec(input: &str) -> Result<usize, std::num::ParseIntError> {
        input.parse::<usize>()
    }

    named!(number<&str, usize>,
        map_res!(digit, from_dec)
    );

    named!(claim<&str, Point>,
        do_parse!(
            x:   number >>
            tag!(", ")   >>
            y: map_res!(non_empty, from_dec) >>
            (Point { x, y, })
        )
    );

    impl FromStr for Point {
        type Err = String;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            match claim(s) {
                Ok(value) => Ok(value.1),
                Err(e) => Err(format!("Failed on \"{s}\": {e}")),
            }
        }
    }
}

fn parse_input(input: &str) -> Vec<Point> {
    input.lines().map(|line| line.parse().unwrap()).collect()
}

fn part_1(input: aoc::Input) -> impl ToString {
    let points = &parse_input(input.raw());

    let mut space = vec![[None; 400]; 400];

    for (x, row) in space.iter_mut().enumerate() {
        for (y, cell) in row.iter_mut().enumerate() {
            let mut distances = points
                .iter()
                .map(|p| (x as isize - p.x as isize + y as isize - p.y as isize).abs())
                .enumerate()
                .collect::<Vec<(usize, isize)>>();
            distances.sort_by_key(|(_, distance)| *distance);
            if distances[0].1 == distances[1].1 {
                *cell = None;
            } else {
                *cell = Some(distances[0].0);
            }
        }
    }

    // for row in space.iter() {
    //     for y in row.iter() {
    //         print!(
    //             "{} ",
    //             if let Some(id) = y {
    //                 id.to_string()
    //             } else {
    //                 ".".to_string()
    //             }
    //         );
    //     }
    //     println!();
    // }

    let sizes = space.iter().flat_map(|row| row.iter()).enumerate().fold(
        HashMap::new(),
        |mut map: HashMap<usize, isize>, (index, value)| {
            if let Some(value) = value {
                let x = index % space.len();
                let count = map.entry(*value).or_default();
                if x == 0 || x == space.len() - 1 || *value < space.len() || *value >= space.len() * space.len() - 1 {
                    *count = -99999;
                } else {
                    *count += 1;
                }
            }
            map
        },
    );

    *sizes.iter().max_by_key(|(_id, size)| *size).unwrap().0
}

// fn part_2(input: aoc::Input) -> impl ToString {
//     0
// }
