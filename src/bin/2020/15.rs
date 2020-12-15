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

fn get_nth_number(initial: Vec<usize>, n: usize) -> usize {
  let mut last_spoken = HashMap::<usize, Vec<usize>>::new();
  let mut last = 0;
  for (i, n) in initial.iter().enumerate() {
    last_spoken.entry(*n).or_insert(vec![]).push(i + 1);
    last = *n;
  }
  let mut i = initial.len() + 1;
  while i <= n {
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
    i += 1;
  }
  return last;
}

fn get_nth_number_2(initial: Vec<usize>, n: usize) -> usize {
  let mut last_spoken = HashMap::<usize, (Option<usize>, Option<usize>)>::new();
  let mut last = 0;
  let nan = usize::max_value();
  for (i, n) in initial.iter().enumerate() {
    last_spoken.entry(*n).or_insert((Option::from(i + 1), None));
    last = *n;
  }
  let mut i = initial.len() + 1;
  while i <= n {
    let (i1, i2) = *last_spoken.get(&last).unwrap();
    if i2.is_none() {
      last = 0;
      let (i1, i2) = *last_spoken.get(&last).unwrap();
      last_spoken.insert(last, (i1, Option::from(i)));
    } else {
      let last_number = (i2.unwrap() - i1.unwrap());
      let (i1, i2) = *last_spoken
        .entry(last_number)
        .or_insert((Option::from(i), None));
      last = last_number;
      last_spoken.insert(last, (i2.or(i1), Option::from(i)));
    }
    i += 1;
  }
  return last;
}

fn part_one(input: &str) -> usize {
  // get_nth_number(parse_input(input), 2020)
  get_nth_number_2(parse_input(input), 2020)
}

fn part_two(input: &str) -> usize {
  // get_nth_number(parse_input(input), 30000000)
  get_nth_number_2(parse_input(input), 30000000)
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

  #[test]
  fn test_part_one() {
    assert_eq!(part_one("0,3,6"), 436);
    assert_eq!(part_one(&read_input()), 276);
  }

  #[test]
  fn test_part_two() {
    assert_eq!(part_two("0,3,6"), 175594);
    assert_eq!(part_two(&read_input()), 31916);
  }
}
