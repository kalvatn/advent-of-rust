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
  let mut minute = timestamp;
  loop {
    minute += 1;
    if minute >= timestamp {
      for bus_id in &bus_ids {
        if minute % bus_id == 0 {
          return (bus_id * (minute - timestamp)) as usize;
        }
      }
    }
  }
}

fn lcm_multi(numbers: Vec<u64>) -> u64 {
  match numbers.len() {
    0 | 1 => 1,
    2 => numbers[0].lcm(&numbers[1]),
    _ => {
      let lcm = numbers[0].lcm(&numbers[1]);
      let mut next: Vec<u64> = numbers.iter().dropping(2).cloned().collect();
      next.push(lcm);
      lcm_multi(next)
    }
  }
}

fn part_two(input: &str) -> u64 {
  let (_timestamp, schedule) = parse_input(input);
  let bus_ids = schedule
    .iter()
    .cloned()
    .enumerate()
    .filter(|item| item.1 > 0u64)
    .map(|item| (item.0 as u64, item.1 as u64))
    .collect::<Vec<(u64, u64)>>();
  let mut minute = 0u64;
  let mut step = bus_ids[0].1;
  loop {
    minute += step;
    let departing_ids = bus_ids
      .iter()
      .filter(|(wait, id)| (minute + wait) % id == 0)
      .map(|(_wait, id)| id)
      .cloned()
      .collect::<Vec<u64>>();

    if departing_ids.len() == bus_ids.len() {
      return minute;
    } else {
      step = lcm_multi(departing_ids);
    }
  }
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
    assert_eq!(part_one(&read_input()), 410);
  }

  #[test]
  fn test_part_two() {
    assert_eq!(part_two(TEST_INPUT), 1068781);
    assert_eq!(part_two(TEST_INPUT_2), 3417);
    assert_eq!(part_two(TEST_INPUT_3), 754018);
    assert_eq!(part_two(TEST_INPUT_4), 779210);
    assert_eq!(part_two(TEST_INPUT_5), 1261476);
    assert_eq!(part_two(TEST_INPUT_6), 1202161486);
    assert_eq!(part_two(&read_input()), 600691418730595);
  }
}
