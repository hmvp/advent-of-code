use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::HashMap;
use std::num::ParseIntError;
use std::str::FromStr;

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub enum Op {
  Mask(Vec<Option<bool>>),
  Write(usize, usize),
}

impl FromStr for Op {
  type Err = ParseIntError;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let mut splits = s.trim().split('=');
    let op = splits.next().unwrap();
    let nr = splits.next().unwrap();

    Ok(if op == "mask " {
      Op::Mask(
        nr.trim()
          .chars()
          .map(|c| match c {
            'X' => None,
            '1' => Some(true),
            '0' => Some(false),
            _ => unreachable!(),
          })
          .collect(),
      )
    } else {
      let address = op.split('[').nth(1).unwrap().strip_suffix("] ").unwrap().parse().unwrap();
      let value = nr.trim().parse().unwrap();

      Op::Write(address, value)
    })
  }
}

impl Op {}

#[aoc_generator(day14)]
pub fn input_generator(input: &str) -> Vec<Op> {
  input.lines().flat_map(str::parse).collect()
}

fn apply_mask(mask: &Option<Vec<Option<bool>>>, mut n: usize) -> usize {
  if let Some(mask) = mask {
    for (index, bit) in mask.iter().rev().enumerate() {
      if let Some(bit) = bit {
        if *bit {
          n |= 1 << index
        } else {
          n &= usize::MAX ^ 1 << index
        }
      }
    }
  }
  n
}

#[aoc(day14, part1)]
pub fn part1(input: &[Op]) -> usize {
  let mut memory: HashMap<usize, usize> = HashMap::new();
  let mut mask = None;
  for op in input {
    match op {
      Op::Mask(m) => mask = Some(m.clone()),
      Op::Write(a, n) => {
        memory.insert(*a, apply_mask(&mask, *n));
      }
    }
  }
  memory.values().sum()
}

fn make_addresses(mask: &Option<Vec<Option<bool>>>, mut address: usize) -> Vec<usize>{
  let mut addresses = Vec::new();
  if let Some(mask) = mask {
    for (index, bit) in mask.iter().rev().enumerate() {
      if let Some(bit) = bit {
        if *bit {
          address |= 1 << index
        }
      }
    }
    addresses.push(address);
    for (index, bit) in mask.iter().rev().enumerate() {
      if bit.is_none() {
          addresses = addresses.iter().flat_map(|address| vec![address | 1<<index, address & (usize::MAX ^ 1 << index)]).collect();
      }
    }
  }
  addresses
}


#[aoc(day14, part2)]
pub fn part2(input: &[Op]) -> usize {
  let mut memory: HashMap<usize, usize> = HashMap::new();
  let mut mask = None;
  for op in input {
    match op {
      Op::Mask(m) => mask = Some(m.clone()),
      Op::Write(a, n) => {
        for address in make_addresses(&mask, *a) {
          memory.insert(address, *n);
        }
      }
    }
  }
  memory.values().sum()
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn check_apply_mask() {
    if let Op::Mask(mask) = &input_generator("mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X")[0] {
      assert_eq!(apply_mask(&Some(mask.clone()), 11), 73);
      assert_eq!(apply_mask(&Some(mask.clone()), 101), 101);
      assert_eq!(apply_mask(&Some(mask.clone()), 0), 64);
    } else {
      unreachable!()
    }
  }

  #[test]
  fn check_part1() {
    assert_eq!(part1(&input_generator("mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X\nmem[8] = 11\nmem[7] = 101\nmem[8] = 0")), 165);
  }

  #[test]
  fn check_make_addresses() {
    if let Op::Mask(mask) = &input_generator("mask = 000000000000000000000000000000X1001X")[0] {
      assert_eq!(make_addresses(&Some(mask.clone()), 42), vec![59, 27, 58, 26]);
    } else {
      unreachable!()
    }

    if let Op::Mask(mask) = &input_generator("mask = 00000000000000000000000000000000X0XX")[0] {
      assert_eq!(make_addresses(&Some(mask.clone()), 26), vec![27, 19, 25, 17, 26, 18, 24, 16]);
    } else {
      unreachable!()
    }
  }

  #[test]
  fn check_part2() {
    assert_eq!(part2(&input_generator("mask = 000000000000000000000000000000X1001X\nmem[42] = 100\nmask = 00000000000000000000000000000000X0XX\nmem[26] = 1")), 208);
  }
}
