use std::str::FromStr;
use std::string::ParseError;
use std::time::{Duration, Instant};

use lazy_static::lazy_static;
use regex::Regex;

use common::io;

struct Policy {
  n1: usize,
  n2: usize,
  letter: char,
  password: String,
}

impl Policy {
  fn password_has_valid_count(&self) -> bool {
    let count = &self.password.chars().filter(|c| c == &self.letter).count();
    count >= &self.n1 && count <= &self.n2
  }

  fn password_has_valid_positions(&self) -> bool {
    let first = &self.password.chars().nth(&self.n1 - 1).unwrap();
    let second = &self.password.chars().nth(&self.n2 - 1).unwrap();
    return first != second && (first == &self.letter || second == &self.letter);
  }
}

impl FromStr for Policy {
  type Err = ParseError;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    lazy_static! {
      static ref RE: Regex = Regex::new(r"^(\d+)-(\d+) (\w): (\w+)$").unwrap();
    }
    Ok(
      RE.captures(s)
        .map(|cap| Policy {
          n1: cap[1].parse::<usize>().unwrap(),
          n2: cap[2].parse::<usize>().unwrap(),
          letter: cap[3].parse::<char>().unwrap(),
          password: cap[4].to_string(),
        })
        .unwrap(),
    )
  }
}

fn read_input() -> String {
  return io::read_input("2020-02");
}

fn parse_input(input: &str) -> Vec<Policy> {
  input
    .lines()
    .map(|line| Policy::from_str(line).unwrap())
    .collect()
}

fn part_one(input: &str) -> (usize, Duration) {
  let vec = parse_input(input);
  let timer = Instant::now();
  (
    vec
      .iter()
      .filter(|policy| policy.password_has_valid_count())
      .count(),
    timer.elapsed(),
  )
}

fn part_two(input: &str) -> (usize, Duration) {
  let vec = parse_input(input);
  let timer = Instant::now();
  (
    vec
      .iter()
      .filter(|policy| policy.password_has_valid_positions())
      .count(),
    timer.elapsed(),
  )
}

fn main() {
  let input = read_input();

  let p1 = part_one(&input);
  println!("part two {:?} {:?}", p1.0, p1.1);
  let p2 = part_two(&input);
  println!("part two {:?} {:?}", p2.0, p2.1);
}

#[cfg(test)]
mod test {
  use super::*;

  const TEST_INPUT: &str = "1-3 a: abcde
1-3 b: cdefg
2-9 c: ccccccccc";

  #[test]
  fn test_password_has_valid_count() {
    let policies = parse_input(TEST_INPUT);
    assert!(policies[0].password_has_valid_count());
    assert!(!policies[1].password_has_valid_count());
    assert!(policies[2].password_has_valid_count());
  }

  #[test]
  fn test_password_has_valid_positions() {
    let policies = parse_input(TEST_INPUT);
    assert!(policies[0].password_has_valid_positions());
    assert!(!policies[1].password_has_valid_positions());
    assert!(!policies[2].password_has_valid_positions());
  }

  #[test]
  fn test_part_one() {
    assert_eq!(part_one(TEST_INPUT).0, 2);
    assert_eq!(part_one(&read_input()).0, 500);
  }

  #[test]
  fn test_part_two() {
    assert_eq!(part_two(TEST_INPUT).0, 1);
    assert_eq!(part_two(&read_input()).0, 313);
  }
}
