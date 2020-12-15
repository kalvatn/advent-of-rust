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

const TEST_INPUT: &str = "0,3,6\n";

fn read_input() -> String {
  return io::read_input("2020-15");
}

fn parse_input(input: &str) -> Vec<usize> {
  return input
    .lines()
    .nth(0)
    .unwrap()
    .split(",")
    .map(|c| c.parse::<usize>().unwrap())
    .collect();
}

fn part_one(input: &str) -> usize {
  let mut last_spoken = HashMap::<usize, Vec<usize>>::new();
  let numbers = parse_input(input);
  let mut order = vec![];
  for (i, n) in numbers.iter().enumerate() {
    last_spoken.entry(*n).or_insert(vec![]).push(i + 1);
    order.push(*n);
  }
  let mut i = numbers.len() + 1;
  while i <= 30000000 {
    let mut last = *order.last().unwrap();
    if last_spoken.contains_key(&last) {
      let indices = last_spoken.get(&last).unwrap();
      let count = indices.len();
      if count == 1 {
        last = 0;
        last_spoken.entry(last).or_insert(vec![]).push(i);
      } else {
        let (i1, i2) = (
          indices.last().unwrap(),
          indices.iter().nth(count - 2).unwrap(),
        );
        last = (i1 - i2);
        last_spoken.entry(last).or_insert(vec![]).push(i);
      }
    } else {
      last = 0;
      last_spoken.entry(last).or_insert(vec![]).push(i);
    }
    i += 1;
    order.push(last);
  }
  // for (k,v) in last_spoken {
  //   println!("{} {:?}", k, v);
  // }

  return *order.last().unwrap();
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
