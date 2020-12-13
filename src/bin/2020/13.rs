#![allow(
  unused_variables,
  unused_imports,
  unused_assignments,
  dead_code,
  deprecated,
  unused_parens,
  unreachable_code,
  unused_mut
)]

use std::time::Instant;

use itertools::Itertools;
use num::Integer;

use common::io;

fn read_input() -> String {
  return io::read_input("2020-13");
}

fn parse_input(input: &str) -> (u64, Vec<u64>) {
  let lines: Vec<&str> = input.lines().collect();
  (
    lines[0].parse::<u64>().unwrap(),
    lines[1]
      .split(",")
      .map(|s| {
        if s == "x" {
          0u64
        } else {
          s.parse::<u64>().unwrap()
        }
      })
      .collect::<Vec<u64>>(),
  )
}

fn part_one(input: &str) -> usize {
  let (timestamp, bus_ids) = parse_input(input);
  let bus_ids = bus_ids
    .iter()
    .cloned()
    .filter(|id| id > &0u64)
    .collect::<Vec<u64>>();

  let mut second = timestamp;
  loop {
    second += 1;
    if second >= timestamp {
      for bus_id in &bus_ids {
        if second % bus_id == 0 {
          return (bus_id * (second - timestamp)) as usize;
        }
      }
    }
  }
}

fn part_two(input: &str) -> u64 {
  let (timestamp, schedule) = parse_input(input);
  let bus_ids = schedule
    .iter()
    .cloned()
    .enumerate()
    .filter(|item| item.1 > 0u64)
    .map(|item| (item.0 as u64, item.1 as u64))
    .collect::<Vec<(u64, u64)>>();

  let mut minute = 0u64; //if bus_ids[0].1 == 37 { 100000000000000u64 } else { 0u64 };
                         // let mut minute = if bus_ids[0].1 == 37 { 10000000000089u64 } else { 0u64 };

  // let vec1: Vec<(u64, u64)> = vec![(0, 7), (1, 13), (4, 59), (6, 31), (7, 19)];
  let min = bus_ids.iter().map(|item| item.1).min().unwrap();
  let max = bus_ids.iter().map(|item| item.1).max().unwrap();
  println!("{} {}", min, max);
  let mut counter = 1;
  let mut next_print = 10000;
  loop {
    // if minute % counter == 0 {
    //   counter += min;
    // }
    if minute > next_print {
      println!("{} {}", minute, counter);
      if next_print < 1000000000000 {
        next_print *= 10;
      } else {
        next_print += 100000000000;
      }
    }
    if bus_ids.iter().all(|item| {
      let yes = (minute + item.0) % item.1 == 0;
      if yes {
        if item.1 > counter && counter < max {
          if (item.1 * counter) < max {
            println!(
              "new counter @ {} {} > {}",
              minute,
              item.1,
              (item.1 * counter)
            );
            counter = (item.1 * counter);
          }
        }
        // println!("{} + {} = {} % {} == 0", minute, item.0, minute+item.0, item.1)
      }
      yes
    }) {
      return minute;
    }
    minute += counter;
  }
}

fn main() {
  // start 08:51
  // p1 09:12
  const TEST_INPUT_5: &str = "939
67,7,x,59,61";
  const TEST_INPUT_6: &str = "939
1789,37,47,1889";
  let input = TEST_INPUT_6;
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

  const TEST_INPUT: &str = "939
7,13,x,x,59,x,31,19";

  const TEST_INPUT_2: &str = "939
17,x,13,19";
  const TEST_INPUT_3: &str = "939
67,7,59,61";
  const TEST_INPUT_4: &str = "939
67,x,7,59,61";
  const TEST_INPUT_5: &str = "939
67,7,x,59,61";
  const TEST_INPUT_6: &str = "939
1789,37,47,1889";

  #[test]
  fn test_parse_input() {
    let (timestamp, bus_ids) = parse_input(TEST_INPUT);
    assert_eq!(timestamp, 939);
    assert_eq!(bus_ids, vec![7, 13, 0, 0, 59, 0, 31, 19]);
  }

  #[test]
  fn test_part_one() {
    assert_eq!(part_one(TEST_INPUT), 295);
    // assert_eq!(part_one(&read_input()), 0);
  }

  #[test]
  fn test_part_two() {
    assert_eq!(part_two(TEST_INPUT), 1068781);
    assert_eq!(part_two(TEST_INPUT_2), 3417);
    assert_eq!(part_two(TEST_INPUT_3), 754018);
    assert_eq!(part_two(TEST_INPUT_4), 779210);
    assert_eq!(part_two(TEST_INPUT_5), 1261476);
    assert_eq!(part_two(TEST_INPUT_6), 1202161486);
    // assert_eq!(part_two(&read_input()), 0);
  }
}
