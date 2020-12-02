use common::io;
use std::time::Instant;


fn read_input() -> String {
  return io::read_input("2015-01");
}

fn parse_input(input:&str) -> Vec<i32> {
  return input.chars().into_iter()
    .map(|c| -> i32 {
      match c {
        '(' => 1,
        ')' => -1,
        _ => 0,
      }
    })
    .collect();
}

fn part_one(input : &str) -> i32 {
  return parse_input(input).iter().sum();
}

fn part_two(input: &str) -> usize {
  return parse_input(input)
    .iter()
    .scan(0, |acc, &n| {
      *acc = *acc + n;
      Some(*acc)
    })
    .take_while(|x| x != &-1)
    .count()
    + 1;
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

  #[test]
  fn test_directions_to_int() {
    assert_eq!(
      parse_input("(())"),
      vec![1, 1, -1, -1]
    );
  }

  #[test]
  fn test_part_one() {
    assert_eq!(part_one("(())"), 0);
    assert_eq!(part_one(&read_input()), 138);
  }

  #[test]
  fn test_part_two() {
    assert_eq!(part_two(")"), 1);
    assert_eq!(part_two("()())"), 5);
    assert_eq!(part_two(&read_input()), 1771);
  }
}
