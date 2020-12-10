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
use std::collections::{HashMap, HashSet, VecDeque};
use std::time::Instant;

use common::io;
use std::str::FromStr;
use std::string::ParseError;

enum Op {
  Jmp,
  Nop,
  Acc,
}

struct Instruction {
  op: Op,
  val: i32,
}

fn read_input() -> String {
  return io::read_input("2020-10");
}

fn parse_input(input: &str) -> Vec<u32> {
  return input
    .lines()
    .map(|line| line.parse::<u32>().unwrap())
    .collect();
}

fn find_joltage_diffs(input: &str) -> (u32, u32) {
  let mut numbers = parse_input(input);
  let max = *numbers.iter().max().unwrap();
  numbers.sort();
  numbers.push(max + 3);
  let (mut one, mut three) = (0, 0);
  let mut rating = 0;
  for i in 0..numbers.len() {
    let adapter = numbers[i] as i32;
    let diff: i32 = (adapter as i32 - rating as i32) as i32;
    println!("{} {}", adapter, diff);

    if diff == 1 {
      one += 1;
    }
    if diff == 3 {
      three += 1;
    }
    rating = adapter;
  }
  return (one, three);
}

fn find_adapter_arrangements(input: &str) -> u64 {
  let mut numbers = parse_input(input);
  let max = *numbers.iter().max().unwrap();
  let min = *numbers.iter().min().unwrap();
  numbers.push(0);
  numbers.push(max + 3);
  numbers.sort();
  numbers.reverse();

  let mut queue: VecDeque<(usize, Vec<u32>)> = VecDeque::new();
  let mut seen: HashSet<Vec<u32>> = HashSet::new();
  let mut valid: HashSet<Vec<u32>> = HashSet::new();
  let mut valid: u64 = 0;
  queue.push_back((0, vec![numbers[0]]));
  while !queue.is_empty() {
    let (index, current) = queue.pop_front().unwrap();
    if seen.contains(&current) {
      continue;
    }
    seen.insert(current.clone());

    if index == numbers.len() - 1 {
      valid += 1;
      continue;
    }

    for i in index + 1..numbers.len() {
      let next = numbers[i];
      let diff = current.last().unwrap() - next;
      if diff <= 3 {
        let mut path = current.clone();
        path.push(next);
        queue.push_back((i, path));
      }
    }
  }
  valid
}

fn part_one(input: &str) -> u32 {
  let (one, three) = find_joltage_diffs(input);
  return one * three;
}

fn part_two(input: &str) -> usize {
  return find_adapter_arrangements(input) as usize;
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

  const TEST_INPUT: &str = "16
10
15
5
1
11
7
19
6
12
4";

  const TEST_INPUT_2: &str = "28
33
18
42
31
14
46
20
48
47
24
23
49
45
19
38
39
11
1
32
25
35
8
17
7
9
4
2
34
10
3";

  #[test]
  fn test_find_joltage_diffs() {
    assert_eq!(find_joltage_diffs(TEST_INPUT), (7, 5));
    assert_eq!(find_joltage_diffs(TEST_INPUT_2), (22, 10));
  }

  #[test]
  fn test_find_adapter_arr() {
    assert_eq!(find_adapter_arrangements(TEST_INPUT), 8);
    assert_eq!(find_adapter_arrangements(TEST_INPUT_2), 19208);
  }
  #[test]
  fn test_part_one() {
    assert_eq!(part_one(TEST_INPUT), 7 * 5);
    assert_eq!(part_one(TEST_INPUT_2), 22 * 10);
    // assert_eq!(part_one(&read_input()), 0);
  }

  #[test]
  fn test_part_two() {
    assert_eq!(part_two(TEST_INPUT), 8);
    // assert_eq!(part_two(TEST_INPUT_2), 19208);
    // assert_eq!(part_two(&read_input()), 0);
  }
}
