use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::HashMap;
use std::iter::FromIterator;

#[aoc_generator(day15)]
pub fn input_generator(input: &str) -> Vec<usize> {
  input.split(',').flat_map(str::parse).collect()
}

fn calc(input: &[usize], nth: usize) -> usize {
  let mut ages: HashMap<usize, usize> = HashMap::from_iter(
    input[..input.len() - 1]
      .iter()
      .enumerate()
      .map(|(n, i)| (*i, n)),
  );
  let mut last_spoken_index = input.len() - 1;
  let mut last_spoken = *input.last().unwrap();
  while last_spoken_index < nth - 1 {
    let entry = ages.insert(last_spoken, last_spoken_index);
    let new_nr = if let Some(old_index) = entry {
      last_spoken_index - old_index
    } else {
      0
    };
    last_spoken = new_nr;
    last_spoken_index += 1;
  }
  last_spoken
}

#[aoc(day15, part1)]
pub fn part1(input: &[usize]) -> usize {
  calc(input, 2020)
}

#[aoc(day15, part2)]
pub fn part2(input: &[usize]) -> usize {
  calc(input, 30000000)
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn check_part1() {
    assert_eq!(part1(&[0, 3, 6]), 436);
    assert_eq!(part1(&[1, 3, 2]), 1);
    assert_eq!(part1(&[2, 1, 3]), 10);
    assert_eq!(part1(&[1, 2, 3]), 27);
    assert_eq!(part1(&[2, 3, 1]), 78);
    assert_eq!(part1(&[3, 2, 1]), 438);
    assert_eq!(part1(&[3, 1, 2]), 1836);
  }

  //   #[test]
  //   fn check_part2() {
  //     assert_eq!(part2(&[0,3,6]), 175594);
  //     assert_eq!(part2(&[1,3,2]), 2578);
  //     assert_eq!(part2(&[2,1,3]), 3544142);
  //     assert_eq!(part2(&[1,2,3]), 261214);
  //     assert_eq!(part2(&[2,3,1]), 6895259);
  //     assert_eq!(part2(&[3,2,1]), 18);
  //     assert_eq!(part2(&[3,1,2]), 362);  }
}
