use std::collections::HashSet;

aoc::parts!(1, 2);

pub fn input_generator(input: aoc::Input) -> Vec<usize> {
    let mut input: Vec<usize> = input.lines().flat_map(str::parse).collect();
    input.push(0);
    input.sort_unstable();
    input.push(input[input.len() - 1] + 3);
    input
}


fn part_1(input: aoc::Input) -> impl ToString {
    let input = &input_generator(input);

    let mut one_jolt = 0;
    let mut three_jolt = 0;

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


fn part_2(input: aoc::Input) -> impl ToString {
    let input = &input_generator(input);

    let mut parts: Vec<&[usize]> = Vec::new();

    let mut start = 0;
    for index in 0..input.len() {
        if index + 1 < input.len() && input[index] + 3 == input[index + 1] {
            parts.push(&input[start..index + 1]);
            start = index + 1;
        }
    }
    parts.push(&input[start-1..input.len()-1]);
    parts.drain(0..parts.len()).map(permutations).product::<usize>()
}

