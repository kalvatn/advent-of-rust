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

use futures::StreamExt;
use itertools::Itertools;
use lazy_static::lazy_static;
use regex::Regex;

use common::io;

fn read_input() -> String {
  return io::read_input("2020-13");
}

fn parse_input(input: &str) -> (u64, Vec<u64>) {
  let lines: Vec<&str> = input.lines().collect();
  (
    lines[0].parse::<u64>().unwrap(),
    lines[1].split(",").filter_map(|s| s.parse::<u64>().ok()).collect::<Vec<u64>>()
  )
}

fn part_one(input: &str) -> usize {
  let (timestamp, bus_ids) = parse_input(input);

  let mut second = timestamp;
  loop {
    second += 1;
    if second >= timestamp {
      for bus_id in &bus_ids {
        if second % bus_id == 0 {
          return (bus_id * (second - timestamp)) as usize;
        }
      }
    }
  }
}

fn part_two(input: &str) -> usize {
  return 0;
}

fn main() {
  // start 08:51
  // p1 09:12
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

  const TEST_INPUT: &str = "939
7,13,x,x,59,x,31,19";

  #[test]
  fn test_parse_input() {
    let (timestamp, bus_ids) = parse_input(TEST_INPUT);
    assert_eq!(timestamp, 939);
    assert_eq!(bus_ids, vec![7, 13, 59, 31, 19]);
  }

  #[test]
  fn test_part_one() {
    assert_eq!(part_one(TEST_INPUT), 295);
    // assert_eq!(part_one(&read_input()), 0);
  }

  #[test]
  fn test_part_two() {
    assert_eq!(part_two(TEST_INPUT), 0);
    // assert_eq!(part_two(&read_input()), 0);
  }
}
