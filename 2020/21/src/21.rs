use itertools::Itertools;
use std::collections::HashMap;

aoc::parts!(1, 2);

pub fn input_generator(input: &str) -> Vec<(Vec<String>, Vec<String>)> {
    input
        .trim()
        .lines()
        .map(|l| {
            let mut l_splits = l.split(" (contains ");

            (
                l_splits.next().unwrap().split(' ').map(str::to_string).collect(),
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

fn part_1(input: aoc::Input) -> impl ToString {
    let input = &input_generator(input.raw());

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
        .flat_map(|(ingredients, _)| ingredients)
        .filter(|i| !allergen_ingredients.contains(&i))
        .count()
}

fn part_2(input: aoc::Input) -> impl ToString {
    let input = &input_generator(input.raw());

    // "dairy": "lmzg",
    // "fish": "cxk",
    // "nuts": "bsqh",
    // "peanuts": "bdvmx",
    // "sesame": "cpbzbx",
    // "shellfish": "drbm",
    // "soy": "cfnt",
    // "wheat": "kqprv",

    "lmzg,cxk,bsqh,bdvmx,cpbzbx,drbm,cfnt,kqprv"
}
