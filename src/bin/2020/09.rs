use std::time::Instant;

use itertools::Itertools;

use common::io;

fn read_input() -> String {
  return io::read_input("2020-09");
}

fn parse_input(input: &str) -> Vec<u64> {
  return input
    .lines()
    .map(|line| line.parse::<u64>().unwrap())
    .collect();
}

fn find_invalid(numbers: &Vec<u64>, length: usize) -> u64 {
  for (i, preamble) in numbers.windows(length).enumerate() {
    let next_number = &numbers[i + length];
    if preamble
      .into_iter()
      .combinations_with_replacement(2)
      .map(|c| *c.first().unwrap() + *c.last().unwrap())
      .all(|s| s != *next_number)
    {
      return *next_number;
    }
  }
  unreachable!("impossiburu")
}

fn part_one(numbers: &Vec<u64>) -> u64 {
  find_invalid(numbers, 25)
}

fn range_for_sum(numbers: &Vec<u64>, target: u64) -> Result<(usize, usize), &'static str> {
  let size = numbers.len();
  for i in 0..size {
    let mut sum = 0;
    let mut j = i;
    while j < size && sum < target {
      sum += numbers[j];
      if sum == target {
        return Ok((i, j));
      }
      j += 1;
    }
  }
  return Err("no range found");
}

fn part_two(numbers: &Vec<u64>, p1: u64) -> u64 {
  let (start, end) = range_for_sum(numbers, p1).unwrap();
  let slice = &numbers[start..end];
  let min = *slice.iter().min().unwrap();
  let max = *slice.iter().max().unwrap();
  return min + max;
}

fn main() {
  let time = Instant::now();
  let input = parse_input(&read_input());
  let parse_time = time.elapsed();

  let time = Instant::now();
  let p1 = part_one(&input);
  let p1_time = time.elapsed();

  let time = Instant::now();
  let p2 = part_two(&input, p1);
  let p2_time = time.elapsed();
  println!("parse {:?}", parse_time);
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
  fn test_find_invalid() {
    assert_eq!(find_invalid(&parse_input(TEST_INPUT), 5), 127);
  }

  #[test]
  fn test_range_for_sum() {
    assert_eq!(range_for_sum(&vec![1, 2, 3, 4], 6), Ok((0, 2)));
    assert_eq!(range_for_sum(&vec![1, 2, 3, 4], 3), Ok((0, 1)));
    assert_eq!(range_for_sum(&vec![1, 2, 3, 4], 7), Ok((2, 3)));
    assert_eq!(range_for_sum(&vec![1, 2, 3, 4], 10), Ok((0, 3)));
    assert_eq!(range_for_sum(&vec![1, 2, 3, 4], 8), Err("no range found"));
  }

  #[test]
  fn test_part_one() {
    assert_eq!(part_one(&parse_input(&read_input())), 31161678);
  }

  #[test]
  fn test_part_two() {
    assert_eq!(part_two(&parse_input(TEST_INPUT), 127), 62);
    assert_eq!(part_two(&parse_input(&read_input()), 31161678), 5453868);
  }
}
