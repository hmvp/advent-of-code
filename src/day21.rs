use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;
use std::collections::HashMap;

#[aoc_generator(day21)]
pub fn input_generator(input: &str) -> Vec<(Vec<String>, Vec<String>)> {
    input
        .trim()
        .lines()
        .map(|l| {
            let mut l_splits = l.split(" (contains ");

            (
                l_splits
                    .next()
                    .unwrap()
                    .split(' ')
                    .map(str::to_string)
                    .collect(),
                l_splits
                    .next()
                    .unwrap()
                    .trim_end_matches(')')
                    .split(", ")
                    .map(str::to_string)
                    .collect(),
            )
        })
        .collect()
}

#[aoc(day21, part1)]
pub fn part1(input: &[(Vec<String>, Vec<String>)]) -> usize {
    let mut allergen_map: HashMap<&str, Vec<&Vec<String>>> = HashMap::new();

    for (ingredients, allergens) in input {
        for allergen in allergens {
            allergen_map.entry(allergen).or_default().push(ingredients)
        }
    }

    let mut allergen_ingredient_map: HashMap<&str, Vec<&String>> = HashMap::new();

    for (allergen, lists_of_ingredients) in allergen_map {
        let grouping = lists_of_ingredients
            .iter()
            .flat_map(|l| l.iter())
            .sorted()
            .group_by(|k| *k);
        allergen_ingredient_map.insert(
            allergen,
            grouping
                .into_iter()
                .map(|(i, it)| (i, it.count()))
                .sorted_by_key(|(_, c)| *c)
                .filter(|(_, c)| *c == lists_of_ingredients.len())
                .map(|(i, _)| i)
                .collect(),
        );
    }

    dbg!(&allergen_ingredient_map);
    let allergen_ingredients = dbg!(allergen_ingredient_map.values().flatten().collect_vec());
    input
        .iter()
        .map(|(ingredients, _)| ingredients)
        .flatten()
        .filter(|i| !allergen_ingredients.contains(&i))
        .count()
}

#[aoc(day21, part2)]
pub fn part2(input: &[(Vec<String>, Vec<String>)]) -> usize {
    
    // "dairy": "lmzg",
    // "fish": "cxk",
    // "nuts": "bsqh",
    // "peanuts": "bdvmx",
    // "sesame": "cpbzbx",
    // "shellfish": "drbm",
    // "soy": "cfnt",
    // "wheat": "kqprv",

    // lmzg,cxk,bsqh,bdvmx,cpbzbx,drbm,cfnt,kqprv

    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_part1() {
        assert_eq!(
            part1(&input_generator(
                "
mxmxvkd kfcds sqjhc nhms (contains dairy, fish)
trh fvjkl sbzzf mxmxvkd (contains dairy)
sqjhc fvjkl (contains soy)
sqjhc mxmxvkd sbzzf (contains fish)
"
            )),
            5
        );
    }
}
