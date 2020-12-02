use std::time::Instant;

use regex::Regex;

use common::io;

fn read_input() -> String {
  return io::read_input("2020-02");
}

struct Instruction {
  min: usize,
  max: usize,
  c: char,
  s: String,
}

fn parse_input(input: &str) -> Vec<Instruction> {
  let re: Regex = Regex::new(r"^(\d+)-(\d+) (\w): (\w+)$").unwrap();
  input
    .lines()
    .map(|line| {
      re.captures(line)
        .map(|cap| Instruction {
          min: cap[1].parse::<usize>().unwrap(),
          max: cap[2].parse::<usize>().unwrap(),
          c: cap[3].parse::<char>().unwrap(),
          s: cap[4].to_string(),
        })
        .unwrap()
    })
    .collect()
}

fn has_count(min: usize, max: usize, c: char, s: &str) -> bool {
  let mut count = 0usize;
  for key in s.chars() {
    if key == c {
      count += 1;
    }
  }
  count >= min && count <= max
}

fn at_index(min: usize, max: usize, c: char, s: &str) -> bool {
  let first = s.chars().nth((min - 1 as usize) as usize).unwrap();
  let second = s.chars().nth((max - 1 as usize) as usize).unwrap();
  return first != second && (first == c || second == c);
}

fn part_one(input: &str) -> usize {
  parse_input(input)
    .iter()
    .filter(|instruction| {
      has_count(
        instruction.min,
        instruction.max,
        instruction.c,
        &*instruction.s,
      )
    })
    .count()
}

fn part_two(input: &str) -> usize {
  parse_input(input)
    .iter()
    .filter(|instruction| {
      at_index(
        instruction.min,
        instruction.max,
        instruction.c,
        &*instruction.s,
      )
    })
    .count()
}

fn main() {
  let input = read_input();

  let p1_timer = Instant::now();
  println!(
    "part one {} {}ms",
    part_one(&input),
    p1_timer.elapsed().as_millis()
  );
  let p2_timer = Instant::now();
  println!(
    "part two {} {}ms",
    part_two(&input),
    p2_timer.elapsed().as_millis()
  );
  println!("total {}ms", p1_timer.elapsed().as_millis())
}

#[cfg(test)]
mod test {
  use super::*;

  const TEST_INPUT: &str = "1-3 a: abcde
1-3 b: cdefg
2-9 c: ccccccccc";

  #[test]
  fn test_has_count() {
    assert!(has_count(1, 3, 'a', "abcde"));
    assert!(!has_count(1, 3, 'b', "cdefg"));
    assert!(has_count(3, 3, 'c', "ccc"));
    assert!(has_count(2, 9, 'c', "ccccccccc"));
  }

  #[test]
  fn test_at_index() {
    assert!(at_index(1, 3, 'a', "abcde"));
    assert!(!at_index(1, 3, 'b', "cdefg"));
    assert!(!at_index(1, 3, 'c', "ccccccccc"));
  }

  #[test]
  fn test_part_one() {
    assert_eq!(part_one(TEST_INPUT), 2);
    assert_eq!(part_one(&read_input()), 500);
  }

  #[test]
  fn test_part_two() {
    assert_eq!(part_two(TEST_INPUT), 1);
    assert_eq!(part_two(&read_input()), 313);
  }
}
