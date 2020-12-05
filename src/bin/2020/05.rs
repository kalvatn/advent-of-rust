#![allow(unused_variables, unused_imports)]

use std::collections::{HashMap, HashSet};
use std::thread::sleep_ms;
use std::time::Instant;

use itertools::Itertools;
use lazy_static::lazy_static;
use regex::Regex;
use serde_scan::scan;

use common::io;

fn read_input() -> String {
  return io::read_input("2020-05");
}

fn parse_input(input: &str) -> &str {
  for line in input.lines() {
    // let line = "#1 @ 555,891: 18x12";
    // let parsed = scan!("#{} @ {},{}: {}x{}" <- line)?;
  }
  return input;
}

fn part_one(input: &str) -> usize {
  let mut best = 0;
  for line in input.lines() {
    let row = get_row(line);
    let seat = get_seat(line);
    let lol = row * 8 + seat;

    if lol >= best {
      best = lol;
    }
  }
  return best as usize;
}

fn part_two(input: &str) -> usize {
  let mut best = 0;
  let mut seat_ids = vec![];
  for line in input.lines() {
    let row = get_row(line);
    if row > 0 && row < 128 {
      let col = get_seat(line);
      let seat_id = row * 8 + col;
      seat_ids.push(seat_id);
    }
  }
  seat_ids.sort();
  let mut prev = seat_ids[0];
  for i in 1..seat_ids.len() {
    let x = seat_ids[i];
    if (x - prev) > 1 {
      return (prev as usize) + 1;
    }
    prev = x;
  }
  return best as usize;
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

fn get_seat(p0: &str) -> i32 {
  let mut low = 0;
  let mut high = 8;
  let mut i = 7;
  while i < p0.len() {
    let c = p0.chars().nth(i).unwrap();
    match c {
      'L' => {
        high -= (high - low) / 2;
      }
      'R' => low += (high - low) / 2,
      _ => unreachable!("invalid"),
    }
    println!("{} {} {} {}", i, c, low, high);
    i += 1;
  }
  return low;
}

fn get_row(p0: &str) -> i32 {
  let mut low = 0;
  let mut high = 128;
  let mut i = 0;
  while i < 7 {
    let c = p0.chars().nth(i).unwrap();
    match c {
      'F' => high -= (high - low) / 2,
      'B' => low += (high - low) / 2,
      _ => unreachable!("invalid"),
    }
    i += 1;
  }
  return low;
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn test_get_row() {
    assert_eq!(get_row("FBFBBFFRLR"), 44);
    assert_eq!(get_row("BFFFBBFRRR"), 70);
    assert_eq!(get_row("FFFBBBFRRR"), 14);
    assert_eq!(get_row("BBFFBBFRLL"), 102);
  }

  #[test]
  fn test_get_seat() {
    assert_eq!(get_seat("FBFBBFFRLR"), 5);
    assert_eq!(get_seat("BFFFBBFRRR"), 7);
    assert_eq!(get_seat("FFFBBBFRRR"), 7);
    assert_eq!(get_seat("BBFFBBFRLL"), 4);
  }

  #[test]
  fn test_part_one() {
    assert_eq!(part_one(&read_input()), 835);
  }

  #[test]
  fn test_part_two() {
    assert_eq!(part_two(&read_input()), 649);
  }
}
