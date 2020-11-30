use std::time::Instant;

use common::io;

fn read_input() -> String {
  return io::read_input("2020-01");
}

fn parse_input(input: &str) -> Vec<char> {
  return input.chars().collect();
}

#[allow(unused_variables)]
fn part_one(input: &str) -> usize {
  return 0;
}

#[allow(unused_variables)]
fn part_two(input: &str) -> usize {
  return 0;
}

fn main() {
  let input = read_input();
  println!("{}", input);

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
  fn test_part_one() {
    assert_eq!(part_one(&read_input()), 0);
  }

  #[test]
  fn test_part_two() {
    assert_eq!(part_two(&read_input()), 0);
  }
}
