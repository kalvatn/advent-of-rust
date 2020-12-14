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
  return io::read_input("2020-15");
}

fn parse_input(input: &str) -> &str {
  return input;
}

fn part_one(input: &str) -> usize {
  return 0;
}

fn part_two(input: &str) -> usize {
  return 0;
}

fn main() {
  let input = read_input();
  println!("{:?}", input);

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

  const TEST_INPUT: &str = "";

  #[test]
  fn test_part_one() {
    assert_eq!(part_one(TEST_INPUT), 0);
    // assert_eq!(part_one(&read_input()), 0);
  }

  #[test]
  fn test_part_two() {
    assert_eq!(part_two(TEST_INPUT), 0);
    // assert_eq!(part_two(&read_input()), 0);
  }
}
