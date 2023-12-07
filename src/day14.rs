use aoc_runner_derive::{aoc, aoc_generator};
use nom::bytes::complete::tag;
use nom::character::complete::alpha1;
use nom::character::complete::digit1;
use nom::multi::separated_nonempty_list;
use nom::sequence::separated_pair;
use nom::IResult;
use std::collections::HashMap;

#[derive(Debug, Hash, PartialEq, Eq)]
pub struct Ingredient {
    nr: usize,
    name: String,
}

fn ingredient(input: &str) -> IResult<&str, Ingredient> {
    let (rest, result) = separated_pair(digit1, tag(" "), alpha1)(input)?;

    Ok((
        rest,
        Ingredient {
            nr: result.0.parse().unwrap(),
            name: result.1.to_string(),
        },
    ))
}

fn reaction(input: &str) -> IResult<&str, (Ingredient, Vec<Ingredient>)> {
    let (rest, list) = separated_nonempty_list(tag(", "), ingredient)(input)?;
    let (rest, _) = tag(" => ")(rest)?;
    let (rest, result) = ingredient(rest)?;

    IResult::Ok((rest, (result, list)))
}

#[aoc_generator(day14)]
pub fn input_generator(input: &str) -> HashMap<String, (usize, Vec<Ingredient>)> {
    input
        .lines()
        .map(|l| {
            let reaction = reaction(l).unwrap().1;
            (reaction.0.name, (reaction.0.nr, reaction.1))
        })
        .collect()
}

fn calculate_ore_for_fuel(nr: usize, input: &HashMap<String, (usize, Vec<Ingredient>)>) -> usize {
    let mut needed = HashMap::new();
    let mut leftovers = HashMap::new();
    needed.insert("FUEL", nr);
    while needed.keys().count() > 1 || !needed.contains_key("ORE") {
        let iter = needed.drain().collect::<Vec<(&str, usize)>>();
        for (i, mut needed_count) in iter {
            if i == "ORE" {
                needed
                    .entry("ORE")
                    .and_modify(|count| *count += needed_count)
                    .or_insert(needed_count);
                continue;
            }

            if leftovers.contains_key(i) {
                leftovers.entry(i).and_modify(|count: &mut usize| {
                    if *count > needed_count {
                        *count -= needed_count;
                        needed_count = 0;
                    } else {
                        needed_count -= *count;
                        *count = 0;
                    }
                });
            }

            if needed_count == 0 {
                continue;
            }

            let (result_count, ingredients) = input.get(i).unwrap();

            let (x, leftover) = if *result_count >= needed_count {
                (1, result_count - needed_count)
            } else if needed_count % result_count == 0 {
                ((needed_count / result_count), 0)
            } else {
                (
                    1 + (needed_count / result_count),
                    result_count - (needed_count % result_count),
                )
            };

            leftovers
                .entry(i)
                .and_modify(|c| *c += leftover)
                .or_insert(leftover);

            for needed_i in ingredients {
                let needed_nr = needed_i.nr * x;
                needed
                    .entry(&needed_i.name)
                    .and_modify(|count| *count += needed_nr)
                    .or_insert(needed_nr);
            }
        }
    }
    *needed.get("ORE").unwrap()
}

#[aoc(day14, part1)]
pub fn part1(input: &HashMap<String, (usize, Vec<Ingredient>)>) -> usize {
    calculate_ore_for_fuel(1, input)
}

#[aoc(day14, part2)]
pub fn part2(input: &HashMap<String, (usize, Vec<Ingredient>)>) -> usize {
    let result = (0..30_000_000)
        .collect::<Vec<usize>>()
        .binary_search_by_key(&1_000_000_000_000, |x| calculate_ore_for_fuel(*x, input));

    if let Ok(nr) = result {
        nr
    } else {
        result.unwrap_err() - 1
    }
}

#[cfg(test)]
mod tests {

    use super::{input_generator, part1};

    #[test]
    fn check_part1() {
        let reactions = input_generator("10 ORE => 10 A\n1 ORE => 1 B\n7 A, 1 B => 1 C\n7 A, 1 C => 1 D\n7 A, 1 D => 1 E\n7 A, 1 E => 1 FUEL");
        assert_eq!(part1(&reactions), 31);

        let reactions = input_generator("9 ORE => 2 A\n8 ORE => 3 B\n7 ORE => 5 C\n3 A, 4 B => 1 AB\n5 B, 7 C => 1 BC\n4 C, 1 A => 1 CA\n2 AB, 3 BC, 4 CA => 1 FUEL");
        assert_eq!(part1(&reactions), 165);
    }
}
