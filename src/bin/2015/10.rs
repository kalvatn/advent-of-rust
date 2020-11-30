#![allow(unused_variables)]

use std::time::Instant;

use common::io;
use std::char;

fn read_input() -> String {
  return io::read_input("2015-10");
}

fn look_and_say(s: &str) -> String {
  let mut new = String::new();
  let mut current = s.chars().nth(0).unwrap();
  let mut count = 0u32;
  for c in s.chars() {
    if c != current {
      new.push(char::from_digit(count, 10).unwrap());
      new.push(current);
      count = 0;
      current = c;
    }
    count += 1;
  }
  new.push(char::from_digit(count, 10).unwrap());
  new.push(current);
  new
}

fn look_and_say_repeat(s: &str, n: u32) -> usize {
  let mut new = s.to_owned();
  for i in 0..n {
    let tmp = look_and_say(&new);
    new = tmp;
  }
  new.len()
}

fn parse_input(input: &str) -> &str {
  input.lines().nth(0).unwrap()
}

fn part_one(input: &str) -> usize {
  look_and_say_repeat(parse_input(&read_input()), 40)
}

fn part_two(input: &str) -> usize {
  look_and_say_repeat(parse_input(&read_input()), 50)
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

  #[test]
  fn test_look_and_say() {
    assert_eq!(look_and_say("1"), "11");
    assert_eq!(look_and_say("11"), "21");
    assert_eq!(look_and_say("21"), "1211");
    assert_eq!(look_and_say("1211"), "111221");
  }

  #[test]
  fn test_part_one() {
    assert_eq!(part_one(&read_input()), 492982);
  }

  #[test]
  fn test_part_two() {
    assert_eq!(part_two(&read_input()), 6989950);
  }
}
