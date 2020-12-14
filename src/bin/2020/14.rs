#![allow(
  unused_variables,
  unused_imports,
  unused_assignments,
  dead_code,
  deprecated,
  unused_parens
)]

use itertools::Itertools;
use lazy_static::lazy_static;
use regex::Regex;
use std::collections::{HashMap, HashSet};
use std::time::Instant;

use common::io;

fn read_input() -> String {
  return io::read_input("2020-14");
}

fn parse_input(input: &str) -> &str {
  return input;
}

fn dec_to_bin(n: usize) -> String {
  format!("{:036b}", n)
}

fn bin_to_dec(b: &str) -> usize {
  usize::from_str_radix(b, 2).unwrap()
}

fn apply_mask(mask: &str, value: &str) -> String {
  let mut result = String::new();
  for (i, c) in mask.chars().enumerate() {
    match c {
      'X' => result.push(value.chars().nth(i).unwrap()),
      '0' | '1' => result.push(c),
      _ => unreachable!(),
    }
  }
  result
}

fn part_one(input: &str) -> usize {
  let mut mem = HashMap::<usize, usize>::new();
  let mut current_mask = "XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX";
  for line in input.lines() {
    if line.starts_with("mask") {
      let mask: &str = serde_scan::scan!("mask = {}" <- line).unwrap();
      println!("mask   {}", mask);
      current_mask = mask;
    } else {
      let (address, value): (usize, usize) = serde_scan::scan!("mem[{}] = {}" <- line).unwrap();
      let binary = format!("{:036b}", value);
      let masked = apply_mask(current_mask, &*binary);
      let decimal = bin_to_dec(&*masked);
      mem.insert(address, decimal);

      println!(
        "binary {} ( value {} -> address {} )",
        binary, value, address
      );
    }
    println!("{:?}", line);
  }
  return mem.values().sum();
}

fn part_two(input: &str) -> usize {
  return 0;
}

fn main() {
  let input = read_input();

  let time = Instant::now();
  let p1 = part_one(&input);
  let p1_time = time.elapsed();

  let time = Instant::now();
  let p2 = part_two(&input);
  let p2_time = time.elapsed();
  println!("part one {:?} {:?}", p1, p1_time);
  println!("part two {:?} {:?}", p2, p2_time);
}

#[cfg(test)]
mod test {
  use super::*;

  const TEST_INPUT: &str = "mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X
mem[8] = 11
mem[7] = 101
mem[8] = 0";

  #[test]
  fn test_dec_to_bin() {
    assert_eq!(dec_to_bin(0), "000000000000000000000000000000000000")
  }
  #[test]
  fn test_bin_to_dec() {
    assert_eq!(bin_to_dec("000000000000000000000000000000000000"), 0);
    assert_eq!(bin_to_dec("000000000000000000000000000000000001"), 1);
    assert_eq!(bin_to_dec("000000000000000000000000000001001001"), 73);
  }

  #[test]
  fn test_apply_mask() {
    assert_eq!(
      apply_mask(
        "XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X",
        "000000000000000000000000000000000000"
      ),
      "000000000000000000000000000001000000"
    )
  }

  #[test]
  fn test_part_one() {
    assert_eq!(part_one(TEST_INPUT), 165);
    assert_eq!(part_one(&read_input()), 9615006043476);
  }

  #[test]
  fn test_part_two() {
    assert_eq!(part_two(TEST_INPUT), 0);
    // assert_eq!(part_two(&read_input()), 0);
  }
}
