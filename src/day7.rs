#![allow(clippy::inconsistent_digit_grouping)]

use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;
use std::collections::HashMap;
use std::collections::HashSet;

#[aoc_generator(day7)]
pub fn input_generator(input: &str) -> HashMap<String, HashMap<String, usize>> {
    input
        .lines()
        .map(|l| {
            let key = l.split(' ').take(2).collect();
            let values = l.split(' ').tuples().skip(1).fold(
                HashMap::new(),
                |mut acc, (number, color1, color2, _)| {
                    acc.insert(color1.to_string() + color2, number.parse().unwrap());
                    acc
                },
            );
            (key, values)
        })
        .collect()
}

fn colors_for_color<'a>(
    input: &'a HashMap<String, HashMap<String, usize>>,
    color: &'a str,
) -> HashSet<&'a str> {
    let mut colors = HashSet::new();
    for (bag_color, contents) in input {
        if contents.contains_key(color) {
            colors.insert(bag_color.as_str());
        }
    }
    colors
}

#[aoc(day7, part1)]
pub fn part1(input: &HashMap<String, HashMap<String, usize>>) -> usize {
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

fn bags_in_color(input: &HashMap<String, HashMap<String, usize>>, color: &str) -> usize {
    let mut number = 1;
    let set = input.get(color).unwrap();
    for (bag_color, n) in set {
        number += n * bags_in_color(input, bag_color)
    }
    number
}

#[aoc(day7, part2)]
pub fn part2(input: &HashMap<String, HashMap<String, usize>>) -> usize {
    bags_in_color(input, "shinygold") -1
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::iter::FromIterator;

    #[test]
    fn check_input_generator() {
        assert_eq!(
            input_generator(
                "dark orange bags contain 3 bright white bags, 4 muted yellow bags.
bright white bags contain 1 shiny gold bag.
faded blue bags contain no other bags."
            ),
            HashMap::from_iter(vec![
                (
                    "darkorange".to_string(),
                    HashMap::from_iter(vec![
                        ("brightwhite".to_string(), 3),
                        ("mutedyellow".to_string(), 4)
                    ])
                ),
                (
                    "brightwhite".to_string(),
                    HashMap::from_iter(vec![("shinygold".to_string(), 1),])
                ),
                ("fadedblue".to_string(), HashMap::new())
            ])
        );
    }

    #[test]
    fn check_part1() {
        assert_eq!(
            part1(&input_generator(
                "light red bags contain 1 bright white bag, 2 muted yellow bags.
dark orange bags contain 3 bright white bags, 4 muted yellow bags.
bright white bags contain 1 shiny gold bag.
muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
dark olive bags contain 3 faded blue bags, 4 dotted black bags.
vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
faded blue bags contain no other bags.
dotted black bags contain no other bags."
            )),
            4
        );
    }

    #[test]
    fn check_part2() {
        assert_eq!(
            part2(&input_generator(
                "shiny gold bags contain 2 dark red bags.
dark red bags contain 2 dark orange bags.
dark orange bags contain 2 dark yellow bags.
dark yellow bags contain 2 dark green bags.
dark green bags contain 2 dark blue bags.
dark blue bags contain 2 dark violet bags.
dark violet bags contain no other bags."
            )),
            126
        );
    }
}
