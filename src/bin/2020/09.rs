use std::time::Instant;

use itertools::Itertools;

use common::io;

fn read_input() -> String {
  return io::read_input("2020-09");
}

fn parse_input(input: &str) -> Vec<u64> {
  return input.lines().map(|l| l.parse::<u64>().unwrap()).collect();
}

fn find_invalid(numbers: Vec<u64>, length: usize) -> u64 {
  let chunks = numbers.windows(length);
  for (i, preamble) in chunks.enumerate() {
    let sums = preamble
      .into_iter()
      .combinations_with_replacement(2)
      .map(|c| *c.first().unwrap() + *c.last().unwrap())
      .collect::<Vec<u64>>();
    let x = &numbers[i + length];
    if !sums.contains(&x) {
      return *x;
    }
  }
  unreachable!("impossiburu")
}

fn part_one(input: &str) -> u64 {
  let numbers = parse_input(input);
  find_invalid(numbers, 25)
}

fn part_two(input: &str, p1: u64) -> u64 {
  let numbers = parse_input(input);
  for i in 0..numbers.len() {
    for j in i + 1..numbers.len() {
      let slice = &numbers[i..j];
      let sum = slice.iter().sum::<u64>();
      if sum > p1 {
        continue;
      }
      if sum == p1 {
        let min = *slice.iter().min().unwrap();
        let max = *slice.iter().max().unwrap();
        return min + max;
      }
    }
  }
  return 0;
}

fn main() {
  let input = read_input();

  let time = Instant::now();
  let p1 = part_one(&input);
  let p1_time = time.elapsed();

  let time = Instant::now();
  let p2 = part_two(&input, p1);
  let p2_time = time.elapsed();
  println!("part one {:?} {:?}", p1, p1_time);
  println!("part two {:?} {:?}", p2, p2_time);
}

#[cfg(test)]
mod test {
  use super::*;

  const TEST_INPUT: &str = "35
20
15
25
47
40
62
55
65
95
102
117
150
182
127
219
299
277
309
576";

  #[test]
  fn test_process_preamble() {
    assert_eq!(find_invalid(parse_input(TEST_INPUT), 5), 127);
  }

  #[test]
  fn test_part_one() {
    assert_eq!(part_one(&read_input()), 31161678);
  }

  #[test]
  fn test_part_two() {
    assert_eq!(part_two(TEST_INPUT, 127), 62);
    assert_eq!(part_two(&read_input(), 31161678), 5453868);
  }
}
