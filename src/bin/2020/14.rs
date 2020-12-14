#![allow(
  unused_variables,
  unused_imports,
  unused_assignments,
  dead_code,
  deprecated,
  unused_parens
)]

use std::collections::{HashMap, HashSet};
use std::time::Instant;

use itertools::Itertools;
use lazy_static::lazy_static;
use regex::Regex;

use common::io;
use std::borrow::BorrowMut;

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

fn apply_mask_2(mask: &str, value: &str) -> String {
  let mut result = String::new();
  for (i, c) in mask.chars().enumerate() {
    match c {
      '0' => result.push(value.chars().nth(i).unwrap()),
      'X' | '1' => result.push(c),
      _ => unreachable!(),
    }
  }
  result
}

fn possible_addresses(floating: &str) -> HashSet<String> {
  let mut perms = HashSet::<String>::new();

  if !floating.contains('X') {
    perms.insert(floating.to_string());
    return perms;
  }

  let mut s: Vec<char> = floating.chars().collect();
  for (i, c) in floating.chars().enumerate() {
    match c {
      'X' => {
        s[i] = '1';
        let floating1: String = s.iter().cloned().collect();
        let p1 = possible_addresses(floating1.as_str());
        for i in p1 {
          perms.insert(i);
        }
        s[i] = '0';
        let floating2: String = s.iter().cloned().collect();
        let p1 = possible_addresses(floating2.as_str());
        for i in p1 {
          perms.insert(i);
        }
      }
      _ => {}
    }
  }
  perms
}

fn part_one(input: &str) -> usize {
  let mut mem = HashMap::<usize, usize>::new();
  let mut current_mask = "XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX";
  for line in input.lines() {
    if line.starts_with("mask") {
      let mask: &str = serde_scan::scan!("mask = {}" <- line).unwrap();
      current_mask = mask;
    } else {
      let (address, value): (usize, usize) = serde_scan::scan!("mem[{}] = {}" <- line).unwrap();
      let binary = format!("{:036b}", value);
      let masked = apply_mask(current_mask, &*binary);
      let decimal = bin_to_dec(&*masked);
      mem.insert(address, decimal);
    }
  }
  return mem.values().sum();
}

fn part_two(input: &str) -> usize {
  let mut mem = HashMap::<usize, usize>::new();
  let mut current_mask = "XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX";
  for line in input.lines() {
    if line.starts_with("mask") {
      let mask: &str = serde_scan::scan!("mask = {}" <- line).unwrap();
      current_mask = mask;
    } else {
      let (address, value): (usize, usize) = serde_scan::scan!("mem[{}] = {}" <- line).unwrap();
      let binary = format!("{:036b}", address);
      let masked = apply_mask_2(current_mask, &*binary);
      for address in possible_addresses(&*masked) {
        let decimal = bin_to_dec(address.as_str());
        mem.insert(decimal, value);
      }
    }
  }
  return mem.values().sum();
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

  const TEST_INPUT_2: &str = "mask = 000000000000000000000000000000X1001X
mem[42] = 100
mask = 00000000000000000000000000000000X0XX
mem[26] = 1";

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
        "000000000000000000000000000000000000",
      ),
      "000000000000000000000000000001000000"
    )
  }

  #[test]
  fn test_apply_mask_2() {
    assert_eq!(
      apply_mask_2(
        "XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X",
        "000000000000000000000000000000000010",
      ),
      "XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX1X",
    );
    assert_eq!(
      apply_mask_2(
        "000000000000000000000000000000X1001X",
        "000000000000000000000000000000101010",
      ),
      "000000000000000000000000000000X1101X",
    );
  }

  #[test]
  fn test_possible_addresses() {
    let set = possible_addresses("000000000000000000000000000000X1101X");
    let mut expected = HashSet::<String>::new();
    expected.insert("000000000000000000000000000000011010".to_string());
    expected.insert("000000000000000000000000000000011011".to_string());
    expected.insert("000000000000000000000000000000111010".to_string());
    expected.insert("000000000000000000000000000000111011".to_string());
    assert_eq!(set, expected);
    let set = possible_addresses("00000000000000000000000000000001X0XX");
    let mut expected = HashSet::<String>::new();
    expected.insert("000000000000000000000000000000010000".to_string());
    expected.insert("000000000000000000000000000000010001".to_string());
    expected.insert("000000000000000000000000000000010010".to_string());
    expected.insert("000000000000000000000000000000010011".to_string());
    expected.insert("000000000000000000000000000000011000".to_string());
    expected.insert("000000000000000000000000000000011001".to_string());
    expected.insert("000000000000000000000000000000011010".to_string());
    expected.insert("000000000000000000000000000000011011".to_string());
    assert_eq!(set, expected);
  }

  #[test]
  fn test_part_one() {
    assert_eq!(part_one(TEST_INPUT), 165);
    assert_eq!(part_one(&read_input()), 9615006043476);
  }

  #[test]
  fn test_part_two() {
    assert_eq!(part_two(TEST_INPUT_2), 208);
    assert_eq!(part_two(&read_input()), 4275496544925);
  }
}
