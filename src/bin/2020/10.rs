use std::collections::{HashMap, HashSet, VecDeque};
use std::time::Instant;

use itertools::Itertools;

use common::io;

fn read_input() -> String {
  return io::read_input("2020-10");
}

fn parse_input(input: &str) -> Vec<u8> {
  let mut numbers: Vec<u8> = input
    .lines()
    .map(|line| line.parse::<u8>().unwrap())
    .collect();
  let max = *numbers.iter().max().unwrap();
  numbers.push(0);
  numbers.push(max + 3);
  numbers.sort();
  return numbers;
}

fn find_joltage_diffs(input: &str) -> (u8, u8) {
  let numbers = parse_input(input);
  let (mut one, mut three) = (0, 0);
  let mut rating = 0;
  for i in 0..numbers.len() {
    let adapter = numbers[i] as i32;
    let diff = adapter as i32 - rating;
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

fn find_adapter_arrangements(numbers: &Vec<u8>) -> u64 {
  let mut counts: HashMap<u8, u64> = HashMap::new();
  counts.insert(0, 1);

  for n in numbers.into_iter().dropping(1) {
    let mut count = 0;
    for i in 1..=3 {
      if n >= &i {
        count += counts.get(&(n - i)).unwrap_or(&0);
      }
    }
    counts.insert(*n, count);
  }
  *counts.get(numbers.last().unwrap()).unwrap()
}

#[allow(unused)]
fn find_adapter_arrangements_oom(input: &str) -> u64 {
  let mut numbers = parse_input(input);
  let max = *numbers.iter().max().unwrap();
  let min = *numbers.iter().min().unwrap();
  numbers.push(0);
  numbers.push(max + 3);
  numbers.sort();
  numbers.reverse();

  let mut queue: VecDeque<(usize, Vec<u8>)> = VecDeque::new();
  let mut seen: HashSet<Vec<u8>> = HashSet::new();
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

fn part_one(input: &str) -> u16 {
  let (one, three) = find_joltage_diffs(input);
  return one as u16 * three as u16;
}

fn part_two(input: &str) -> u64 {
  let numbers = parse_input(input);
  return find_adapter_arrangements(&numbers);
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
    assert_eq!(find_adapter_arrangements(&vec![0, 1]), 1);
    assert_eq!(find_adapter_arrangements(&vec![0, 3]), 1);
    assert_eq!(find_adapter_arrangements(&vec![0, 3, 5]), 1);
    assert_eq!(find_adapter_arrangements(&vec![0, 1, 2, 3]), 4);
    assert_eq!(find_adapter_arrangements(&vec![0, 1, 2, 3, 4]), 7);
    assert_eq!(find_adapter_arrangements(&vec![0, 1, 2, 3, 4, 5]), 13);
    assert_eq!(find_adapter_arrangements(&vec![0, 3, 4, 5, 8]), 2);
    assert_eq!(find_adapter_arrangements(&vec![0, 3, 4, 5, 6, 8]), 6);
    assert_eq!(find_adapter_arrangements(&parse_input(TEST_INPUT)), 8);
    assert_eq!(find_adapter_arrangements(&parse_input(TEST_INPUT_2)), 19208);
    assert_eq!(find_adapter_arrangements_oom(TEST_INPUT_2), 19208);
  }

  #[test]
  fn test_part_one() {
    assert_eq!(part_one(TEST_INPUT), 7 * 5);
    assert_eq!(part_one(TEST_INPUT_2), 22 * 10);
    assert_eq!(part_one(&read_input()), 2100);
  }

  #[test]
  fn test_part_two() {
    assert_eq!(part_two(TEST_INPUT), 8);
    assert_eq!(part_two(TEST_INPUT_2), 19208);
    assert_eq!(part_two(&read_input()), 16198260678656);
  }
}
