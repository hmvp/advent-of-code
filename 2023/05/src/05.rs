use aoc::Input;
use itertools::Itertools;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{alphanumeric1, digit1, newline, space0, space1};
use nom::combinator::map;
use nom::multi::{many1, separated_list1};
use nom::sequence::{delimited, pair, preceded};

aoc::parts!(1, 2);

#[derive(Debug)]
struct Conversion {
    source_start: usize,
    dest_start: usize,
    range_length: usize,
}

impl Conversion {
    fn end(&self) -> usize {
        self.source_start + self.range_length
    }
}
#[derive(Debug)]
struct Map {
    conversions: Vec<Conversion>,
}

impl Map {
    fn new(conversions: Vec<Conversion>) -> Self {
        Map { conversions }
    }

    fn convert(&self, number: &usize) -> usize {
        for conversion in &self.conversions {
            if (conversion.source_start) <= *number && *number < conversion.end() {
                return number - conversion.source_start + conversion.dest_start;
            }
        }

        *number
    }

    fn _convert_range(&self, numbers: (usize, usize)) -> Vec<(usize, usize)> {
        let (start, size) = numbers;
        let mut prev = 0;
        let mut group_start = 0;

        let mut result = Vec::new();
        for (group_start, group) in &(start..(start + size))
            .map(|number| self.convert(&number))
            .group_by(|&item| {
                if prev + 1 == item {
                    prev = item;
                    group_start
                } else {
                    group_start = item;
                    prev = item;
                    item
                }
            })
        {
            result.push((group_start, group.count()))
        }

        result
    }

    fn convert_range_smart(&self, numbers: (usize, usize)) -> Vec<(usize, usize)> {
        let (start, size) = numbers;

        let (todo, mut result) = self
            .conversions
            .iter()
            .filter(|c| c.source_start <= start + size && c.end() > start)
            .fold((vec![numbers], Vec::new()), |(todo, mut result), c| {
                let mut new_todo = Vec::new();

                for (start, size) in &todo {
                    let end = start + size;
                    match (
                        c.source_start >= *start,  // Conversion starts after todo range start
                        c.source_start < end, // But not after its end
                        c.end() < start + size, // Conversion ends before todo range ends
                        c.end() > *start,// But not before its start
                    ) {
                        (true, true, true, true) => {
                            new_todo.push((*start, c.source_start - start));
                            result.push((self.convert(&c.source_start), c.range_length));
                            new_todo.push((c.end(), end - c.end()));
                        }
                        (true, true, false, true) => {
                            new_todo.push((*start, c.source_start - start));
                            result.push((self.convert(&c.source_start), end - c.source_start));
                        }
                        (false, true, true, true) => {
                            result.push((self.convert(start), c.end() - start));
                            new_todo.push((c.end(), end - c.end()));
                        }
                        (false, true, false, true) => result.push((self.convert(start), *size)),
                        _ => new_todo.push((*start, *size)),
                    }
                }
                (new_todo, result)
            });

        result.extend(&todo);
        result.sort();
        result
    }
}

fn parse_input(input: Input) -> (Vec<usize>, Vec<Map>) {
    let parse_seeds = preceded(
        tag("seeds:"),
        many1(map(preceded(space1, digit1::<&str, ()>), |string: &str| {
            string.parse::<usize>().unwrap()
        })),
    );

    let parse_conversion = map(
        many1(map(preceded(space0, digit1::<&str, ()>), |string: &str| {
            string.parse().unwrap()
        })),
        |numbers| Conversion {
            dest_start: numbers[0],
            source_start: numbers[1],
            range_length: numbers[2],
        },
    );

    let parse_map = preceded(
        delimited(
            many1(newline),
            many1(alt((alphanumeric1, space1, tag("-")))),
            preceded(tag(":"), newline),
        ),
        map(separated_list1(newline, parse_conversion), |conversions| {
            Map::new(conversions)
        }),
    );

    let mut parse_almanac = pair(parse_seeds, many1(parse_map));

    parse_almanac(input.raw()).unwrap().1
}

fn part_1(input: aoc::Input) -> impl ToString {
    let (mut seeds, maps) = parse_input(input);

    for map in maps {
        seeds = seeds.iter().map(|seed| map.convert(seed)).collect();
    }

    seeds.drain(..).min().unwrap()
}

fn part_2(input: aoc::Input) -> impl ToString {
    let (seeds, maps) = parse_input(input);

    let mut seeds: Vec<(usize, usize)> = seeds.chunks(2).map(|pair| (pair[0], pair[1])).collect();

    for map in maps {
        seeds = seeds
            .iter()
            .flat_map(|seed_range| {
                map.convert_range_smart(*seed_range)
            })
            .collect();
    }

    seeds
        .drain(..)
        .flat_map(|(start, size)| start..(start + size))
        .min()
        .unwrap()
}
