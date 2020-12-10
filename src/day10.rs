use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::HashSet;

#[aoc_generator(day10)]
pub fn input_generator(input: &str) -> Vec<usize> {
    let mut input: Vec<usize> = input.lines().map(str::parse).flatten().collect();
    input.push(0);
    input.sort_unstable();
    input.push(input[input.len() - 1] + 3);
    input
}

#[aoc(day10, part1)]
pub fn part1(input: &[usize]) -> usize {
    let mut one_jolt = 0;
    let mut three_jolt = 1;

    for i in input.windows(2) {
        if i[1] - i[0] == 1 {
            one_jolt += 1;
        } else if i[1] - i[0] == 2 {
        } else if i[1] - i[0] == 3 {
            three_jolt += 1;
        }
    }
    one_jolt * three_jolt
}

// #[aoc(day10, part2)]
// pub fn part2(input: &[usize]) -> usize {
//     let mut permutations = HashSet::new();
//     permutations.insert(vec![0]);

//     for i in input {
//         dbg!(i);
//         let mut new_perms = HashSet::new();
//         for p in permutations.drain() {
//             if p.len()> 1 && p[1] + 3 >= *i {
//                 let mut new = Vec::with_capacity(p.len());
//                 new.push(*i);
//                 new.extend(&p[1..]);
//                 new_perms.insert(new);
//             }

//             if p[0] + 3 >= *i {
//                 let mut new = Vec::with_capacity(p.len()+1);
//                 new.push(*i);
//                 new.extend(p);
//                 new_perms.insert(new);
//             }
//         }
//         permutations = new_perms;
//     }

//     permutations.len()
// }

fn permutations(input: &[usize]) -> usize {
    let mut permutations = HashSet::new();
    permutations.insert(vec![input[0]]);

    for i in input[1..].iter() {
        let mut new_perms = HashSet::new();
        for p in permutations.drain() {
            if p.len() > 1 && p[1] + 3 >= *i {
                let mut new = Vec::with_capacity(p.len());
                new.push(*i);
                new.extend(&p[1..]);
                new_perms.insert(new);
            }

            if p[0] + 3 >= *i {
                let mut new = Vec::with_capacity(p.len() + 1);
                new.push(*i);
                new.extend(p);
                new_perms.insert(new);
            }
        }
        permutations = new_perms;
    }

    permutations.len()
}

#[aoc(day10, part2)]
pub fn part2(input: &[usize]) -> usize {
    let mut parts: Vec<&[usize]> = Vec::new();

    let mut start = 0;
    for index in 0..input.len() {
        if index + 1 < input.len() && input[index] + 3 == input[index + 1] {
            parts.push(&input[start..index + 1]);
            start = index + 1;
        }
    }
    parts.push(&input[start-1..input.len()-1]);
    parts.drain(0..parts.len()).map(permutations).product()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_part1() {
        let mut input = [
            28, 33, 18, 42, 31, 14, 46, 20, 48, 47, 24, 23, 49, 45, 19, 38, 39, 11, 1, 32, 25, 35,
            8, 17, 7, 9, 4, 2, 34, 10, 3, 0, 51,
        ];
        input.sort_unstable();

        assert_eq!(part1(&input), 220);
    }

    #[test]
    fn check_part2() {
        let mut input = [16, 10, 15, 5, 1, 11, 7, 19, 6, 12, 4];
        input.sort_unstable();

        assert_eq!(part2(&input), 8);

        let mut input = [
            28, 33, 18, 42, 31, 14, 46, 20, 48, 47, 24, 23, 49, 45, 19, 38, 39, 11, 1, 32, 25, 35,
            8, 17, 7, 9, 4, 2, 34, 10, 3, 0, 51,
        ];
        input.sort_unstable();

        assert_eq!(part2(&input), 19208);
    }
}
