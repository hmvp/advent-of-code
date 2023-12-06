use std::collections::HashMap;
use std::ops::Range;

aoc::parts!(1, 2);

fn parse_constraints(input: &str) -> HashMap<String, (Range<usize>, Range<usize>)> {
    input
        .lines()
        .map(|l| {
            let mut splits = l.split(": ");
            let field_name = splits.next().unwrap().to_string();

            let mut range_splits = splits.next().unwrap().split(" or ");

            let mut range1_splits = range_splits.next().unwrap().split('-');
            let range1 = Range {
                start: range1_splits.next().unwrap().parse().unwrap(),
                end: range1_splits.next().unwrap().parse::<usize>().unwrap() + 1,
            };

            let mut range2_splits = range_splits.next().unwrap().split('-');
            let range2 = Range {
                start: range2_splits.next().unwrap().parse().unwrap(),
                end: range2_splits.next().unwrap().parse::<usize>().unwrap() + 1,
            };

            (field_name, (range1, range2))
        })
        .collect()
}

fn parse_ticket(input: &str) -> Vec<usize> {
    input.split(',').flat_map(str::parse).collect()
}

#[derive(Debug, PartialEq)]
pub struct Input {
    pub constraints: HashMap<String, (Range<usize>, Range<usize>)>,
    pub ticket: Vec<usize>,
    pub other_tickets: Vec<Vec<usize>>,
}

pub fn input_generator(input: &str) -> Input {
    let mut blocks = input.trim().split("\n\n");

    let constraints = parse_constraints(blocks.next().unwrap());
    let ticket = blocks.next().unwrap().lines().skip(1).map(parse_ticket).next().unwrap();
    let other_tickets = blocks.next().unwrap().lines().skip(1).map(parse_ticket).collect();

    Input {
        constraints,
        ticket,
        other_tickets,
    }
}

fn in_constraints(constraints: &HashMap<String, (Range<usize>, Range<usize>)>, i: usize) -> bool {
    constraints.values().any(|(r1, r2)| r1.contains(&i) || r2.contains(&i))
}

fn part_1(input: aoc::Input) -> impl ToString {
    let input = &input_generator(input.raw());

    input
        .other_tickets
        .iter()
        .flatten()
        .copied()
        .filter(|i| !in_constraints(&input.constraints, *i))
        .sum::<usize>()
}

// fn is_valid(
//   constraints: &HashMap<String, (Range<usize>, Range<usize>)>,
//   keys: &[&String],
//   ticket: &[usize],
// ) -> bool {
//   keys.iter().zip(ticket.iter()).all(|(key, value)| {
//     constraints.get(*key).unwrap().0.contains(value)
//       || constraints.get(*key).unwrap().1.contains(value)
//   })
// }

// fn find_valid_field_order(input: &Input) -> Vec<&String> {
//   let valid_tickets: Vec<&Vec<usize>> = input
//     .other_tickets
//     .iter()
//     .filter(|t| {
//       (**t)
//         .iter()
//         .fold(Vec::new(), |mut acc, i| {
//           if in_constraints(&input.constraints, *i) {
//             acc.push(i)
//           };
//           acc
//         })
//         .iter()
//         .copied()
//         .sum::<usize>()
//         > 0
//     })
//     .collect();
//   let valid_tickets = &valid_tickets;

//   let mut data: Vec<&String> = input.constraints.keys().collect();
//   let mut permutations = Heap::new(&mut data);

//   permutations
//     .find(|p| {
//       valid_tickets.iter().fold(true, |acc, ticket| {
//         is_valid(&input.constraints, p, ticket) && acc
//       })
//     })
//     .unwrap()
//     .clone()
// }

fn find_valid_field_order(input: &Input) -> Vec<&String> {
    let valid_tickets = &input
        .other_tickets
        .iter()
        .filter(|t| (**t).iter().all(|i| in_constraints(&input.constraints, *i)))
        .collect::<Vec<&Vec<usize>>>();

    let mut valid_constraints: Vec<(usize, Vec<&String>)> = Vec::new();
    for i in 0..input.ticket.len() {
        let mut valid_keys = Vec::new();
        for (key, constraint) in &input.constraints {
            if valid_tickets
                .iter()
                .map(|ticket| ticket[i])
                .all(|value| constraint.0.contains(&value) || constraint.1.contains(&value))
            {
                valid_keys.push(key);
            }
        }
        valid_constraints.push((i, valid_keys));
    }

    valid_constraints.sort_by(|a, b| a.1.len().cmp(&b.1.len()));
    let mut valid_order: Vec<Option<&String>> = Vec::new();
    valid_order.resize_with(input.ticket.len(), || None);
    for (index, mut o) in valid_constraints {
        valid_order[index] = Some(o.drain(..).find(|key| !valid_order.contains(&Some(key))).unwrap())
    }
    valid_order.drain(..).flatten().collect()
}

fn part_2(input: aoc::Input) -> impl ToString {
    let input = &input_generator(input.raw());

    let valid_order = find_valid_field_order(input);
    valid_order
        .iter()
        .enumerate()
        .filter(|(_, key)| key.starts_with("departure"))
        .map(|(index, _)| input.ticket[index])
        .product::<usize>()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::iter::FromIterator;

    #[test]
    fn check_input_generator() {
        let input = input_generator(
            "
class: 1-3 or 5-7
row: 6-11 or 33-44
seat: 13-40 or 45-50

your ticket:
7,1,14

nearby tickets:
7,3,47
40,4,50
55,2,20
38,6,12
      ",
        );

        assert_eq!(
            input,
            Input {
                constraints: HashMap::from_iter(vec![
                    ("row".to_string(), (6..12, 33..45)),
                    ("seat".to_string(), (13..41, 45..51)),
                    ("class".to_string(), (1..4, 5..8))
                ]),
                ticket: vec![7, 1, 14],
                other_tickets: vec![vec![7, 3, 47], vec![40, 4, 50], vec![55, 2, 20], vec![38, 6, 12]]
            }
        );
    }

    //   #[test]
    //   fn check_is_valid() {
    //     let input = input_generator(
    //       "
    // class: 0-1 or 4-19
    // row: 0-5 or 8-19
    // seat: 0-13 or 16-19

    // your ticket:
    // 11,12,13

    // nearby tickets:
    // 3,9,18
    // 15,1,5
    // 5,14,9
    //         ",
    //     );

    //     assert_eq!(
    //       is_valid(
    //         &input.constraints,
    //         &[
    //           &"row".to_string(),
    //           &"class".to_string(),
    //           &"seat".to_string()
    //         ],
    //         &[3, 9, 18]
    //       ),
    //       true
    //     );
    //     assert_eq!(
    //       is_valid(
    //         &input.constraints,
    //         &[
    //           &"row".to_string(),
    //           &"class".to_string(),
    //           &"seat".to_string()
    //         ],
    //         &[3, 3, 18]
    //       ),
    //       false
    //     );
    //   }

    #[test]
    fn check_find_valid_field_order() {
        let input = input_generator(
            "
class: 0-1 or 4-19
row: 0-5 or 8-19
seat: 0-13 or 16-19

your ticket:
11,12,13

nearby tickets:
1,1,2
200,1,2
3,9,18
15,1,5
5,14,9
        ",
        );

        assert_eq!(find_valid_field_order(&input), vec!["row", "class", "seat"]);
    }
}
