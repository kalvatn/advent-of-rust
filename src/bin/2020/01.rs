use std::time::Instant;

use common::io;

fn read_input() -> String {
  return io::read_input("2020-aoc-2020-01");
}

fn parse_input(input: &str) -> Vec<u32> {
  input
    .lines()
    .map(|line| line.parse::<u32>().unwrap())
    .collect()
}

fn part_one(input: &str) -> u32 {
  let numbers = parse_input(input);
  for i in &numbers {
    for j in &numbers {
      if j != i && j + i == 2020 {
        return j * i;
      }
    }
  }
  panic!("impossiburu");
}

fn part_two(input: &str) -> u32 {
  let numbers = parse_input(input);
  for i in &numbers {
    for j in &numbers {
      if i + j > 2020 {
        continue;
      }
      for k in &numbers {
        if j != i && j != k && j + i + k == 2020 {
          return j * i * k;
        }
      }
    }
  }
  panic!("impossiburu");
}

fn main() {
  let input = read_input();

  let p1_timer = Instant::now();
  println!(
    "part one {} {}µs",
    part_one(&input),
    p1_timer.elapsed().as_micros()
  );
  let p2_timer = Instant::now();
  println!(
    "part two {} {}µs",
    part_two(&input),
    p2_timer.elapsed().as_micros()
  );
  println!("total {}µs", p1_timer.elapsed().as_micros())
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn test_part_one() {
    assert_eq!(part_one(&read_input()), 32064);
  }

  #[test]
  fn test_part_two() {
    assert_eq!(part_two(&read_input()), 193598720);
  }
}
