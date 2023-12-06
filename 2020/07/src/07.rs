#![allow(clippy::inconsistent_digit_grouping)]

use itertools::Itertools;
use std::collections::HashMap;
use std::collections::HashSet;

aoc::parts!(1, 2);

pub fn input_generator(input: aoc::Input) -> HashMap<String, HashMap<String, usize>> {
    input
        .lines()
        .map(|l| {
            let key = l.split(' ').take(2).collect();
            let values = l
                .split(' ')
                .tuples()
                .skip(1)
                .fold(HashMap::new(), |mut acc, (number, color1, color2, _)| {
                    acc.insert(color1.to_string() + color2, number.parse().unwrap());
                    acc
                });
            (key, values)
        })
        .collect()
}

fn part_1(input: aoc::Input) -> impl ToString {
    let input: &HashMap<String, HashMap<String, usize>> = &input_generator(input);

    let mut colors = colors_for_color(input, "shinygold");
    loop {
        let mut new_colors = HashSet::new();
        for color in &colors {
            new_colors.extend(colors_for_color(input, color));
        }

        let l = colors.len();
        colors.extend(new_colors);

        if l == colors.len() {
            break;
        }
    }
    colors.len()
}

fn colors_for_color<'a>(input: &'a HashMap<String, HashMap<String, usize>>, color: &'a str) -> HashSet<&'a str> {
    let mut colors = HashSet::new();
    for (bag_color, contents) in input {
        if contents.contains_key(color) {
            colors.insert(bag_color.as_str());
        }
    }
    colors
}

fn bags_in_color(input: &HashMap<String, HashMap<String, usize>>, color: &str) -> usize {
    let mut number = 1;
    let set = input.get(color).unwrap();
    for (bag_color, n) in set {
        number += n * bags_in_color(input, bag_color);
    }
    number
}

fn part_2(input: aoc::Input) -> impl ToString {
    let input: &HashMap<String, HashMap<String, usize>> = &input_generator(input);

    bags_in_color(input, "shinygold") - 1
}
