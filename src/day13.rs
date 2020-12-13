use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day13)]
pub fn input_generator(input: &str) -> (usize, Vec<(usize, usize)>) {
    let mut lines = input.lines();
    let start_time = lines.next().unwrap().parse().unwrap();
    let busses = lines
        .next()
        .unwrap()
        .split(',')
        .enumerate()
        .filter_map(|i| i.1.parse().ok().map(|n| (i.0, n)))
        .collect();

    (start_time, busses)
}

#[aoc(day13, part1)]
pub fn part1(input: &(usize, Vec<(usize, usize)>)) -> usize {
    let mut earliest_bus = 0;
    let mut wait_time = usize::MAX;
    for (_index, bus) in &input.1 {
        let bus_wait = bus - (input.0 % bus);
        if bus_wait < wait_time {
            wait_time = bus_wait;
            earliest_bus = *bus;
        }
    }
    earliest_bus * wait_time
}

// fn correct_timestamp(input: &[(usize, usize)], timestamp: usize) -> bool {
//     for (index, bus) in &input[1..] {
//         if (timestamp + index) % bus != 0 {
//             return false;
//         }
//     }
//     true
// }

// #[aoc(day13, part2)]
// pub fn part2(input: &(usize, Vec<(usize, usize)>)) -> usize {
//     let mut timestamp = 100000000000003;
//     while !correct_timestamp(&input.1, timestamp) {
//         timestamp += input.1[0].1;
//     }

//     timestamp
// }



// from: https://rosettacode.org/wiki/Chinese_remainder_theorem#Rust
fn egcd(a: isize, b: isize) -> (isize, isize, isize) {
    if a == 0 {
      (b, 0, 1)
    } else {
      let (g, x, y) = egcd(b % a, a);
      (g, y - (b / a) * x, x)
    }
  }
  
  fn mod_inv(x: isize, n: isize) -> Option<isize> {
    let (g, x, _) = egcd(x, n);
    if g == 1 {
      Some((x % n + n) % n)
    } else {
      None
    }
  }
  
  fn chinese_remainder(residues: &[isize], modulii: &[isize]) -> Option<isize> {
    let prod = modulii.iter().product::<isize>();
    let mut sum = 0;
    for (&residue, &modulus) in residues.iter().zip(modulii) {
      let p = prod / modulus;
      sum += residue * mod_inv(p, modulus)? * p
    }
    Some(sum % prod)
  }
  

#[aoc(day13, part2)]
pub fn part2(input: &(usize, Vec<(usize, usize)>)) -> isize {
    let u: Vec<isize> = input.1.iter().map(|(i, bus)| (*bus - *i) as isize).collect();
    let m: Vec<isize> = input.1.iter().map(|(_i, bus)| *bus as isize).collect();
    chinese_remainder(&u, &m).unwrap().abs()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_part1() {
        assert_eq!(part1(&input_generator("939\n7,13,x,x,59,x,31,19")), 295);
    }

    #[test]
    fn check_part2() {
        assert_eq!(part2(&input_generator("0\n7,13,x,x,59,x,31,19")), 1068781);
        assert_eq!(part2(&input_generator("0\n17,x,13,19")), 3417);
        assert_eq!(part2(&input_generator("0\n67,7,59,61")), 754018);
        assert_eq!(part2(&input_generator("0\n67,x,7,59,61")), 779210);
        assert_eq!(part2(&input_generator("0\n67,7,x,59,61")), 1261476);
        assert_eq!(part2(&input_generator("0\n1789,37,47,1889")), 1202161486);
    }
}
