#![allow(unused_variables, unused_imports, dead_code)]

use std::collections::{HashMap, HashSet};
use std::time::Instant;

use itertools::Itertools;
use lazy_static::lazy_static;
use regex::Regex;

use common::io;

fn read_input() -> String {
  return io::read_input("2020-06");
}

fn parse_input(input: &str) -> Vec<&str> {
  input.split("\n\n").collect()
}

fn part_one(input: &str) -> usize {
  let mut lines = input.lines().into_iter();
  let mut answer = 0;
  let mut lol: HashSet<char> = HashSet::new();
  while let Some(l) = lines.next() {
    println!("{}", l);
    if l.is_empty() {
      println!("{:?}", lol);
      answer += lol.len();
      lol = HashSet::new();
    }
    let mut chars: Vec<char> = l.chars().into_iter().collect();

    for c in chars {
      lol.insert(c);
    }
    // for i in 0..count {
    //   let a = lines.next().unwrap();
    //   assert_eq!(a.len(), 1);
    // }
  }
  answer += lol.len();
  return answer;
}

fn part_two(input: &str) -> usize {
  let mut lines = input.lines().into_iter();
  let mut answer = 0;
  let mut people = 0;
  let mut lol: HashMap<char, usize> = HashMap::new();
  while let Some(l) = lines.next() {
    if l.is_empty() {
      let questions = lol.keys().len();
      println!("{:?}, people {:?}, questions {:?}", lol, people, questions);
      for (k, v) in lol {
        if v == people {
          println!("{}", people);
          answer += 1;
        }
      }
      people = 0;
      lol = HashMap::new();
      println!("{}", l);
    } else {
      let mut chars: Vec<char> = l.chars().into_iter().collect();
      people += 1;

      for c in chars {
        if lol.contains_key(&c) {
          let count = lol.get(&c).unwrap();
          lol.insert(c, count + 1);
        } else {
          lol.insert(c, 1);
        }
        lol.entry(c).or_insert(1);
      }
    }
  }
  let questions = lol.keys().len();
  println!("{:?}, people {:?}, questions {:?}", lol, people, questions);
  for (k, v) in lol {
    if v == people {
      println!("{}", people);
      answer += 1;
    }
  }
  return answer;
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
