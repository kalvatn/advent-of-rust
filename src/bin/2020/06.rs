use std::collections::{HashMap, HashSet};
use std::time::Instant;

use common::io;
use itertools::__std_iter::FromIterator;
use std::slice::SliceIndex;

fn read_input() -> String {
  return io::read_input("2020-06");
}

fn parse_input(input: &str) -> Vec<&str> {
  input.split("\n\n").collect()
}

fn part_one(input: &str) -> usize {
  // let mut count = 0;
  // let mut yes: HashSet<char> = HashSet::new();
  // for l in input.lines() {
  //   if l.is_empty() {
  //     count += yes.len();
  //     yes.clear()
  //   }
  //   for c in l.chars() {
  //     yes.insert(c);
  //   }
  // }
  // count += yes.len();
  // return count;
  input.split("\n\n").fold(0, |acc, line| {
    let map = line.lines().flat_map(|l| l.chars().into_iter());
    let set = HashSet::<char>::from_iter(map);
    acc + set.len()
  })
}

fn part_two(input: &str) -> usize {
  let mut count = 0;
  let mut people = 0;
  let mut yes_by_q: HashMap<char, usize> = HashMap::new();
  for l in input.lines() {
    if l.is_empty() {
      count += &yes_by_q.values().filter(|v| v == &&people).count();
      people = 0;
      yes_by_q.clear();
    } else {
      people += 1;
      for c in l.chars() {
        let count = **&yes_by_q.entry(c).or_insert(0);
        yes_by_q.insert(c, count + 1);
      }
    }
  }
  count += &yes_by_q.values().filter(|v| v == &&people).count();
  return count;
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

  const TEST_INPUT: &str = "abc

a
b
c

ab
ac

a
a
a
a

b";

  #[test]
  fn test_part_one() {
    assert_eq!(part_one(TEST_INPUT), 11);
    assert_eq!(part_one(&read_input()), 6457);
  }

  #[test]
  fn test_part_two() {
    assert_eq!(part_two(TEST_INPUT), 6);
    assert_eq!(part_two(&read_input()), 3260);
  }
}
