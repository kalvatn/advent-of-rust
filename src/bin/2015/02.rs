use common::io;

use std::string::ParseError;

use std::str::FromStr;

use lazy_static::lazy_static;
use regex::Regex;

use std::time::Instant;

#[derive(Debug, PartialEq, Eq)]
struct Dimension {
  l: u32,
  w: u32,
  h: u32,
}

impl FromStr for Dimension {
  type Err = ParseError;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    lazy_static! {
      static ref RE: Regex = Regex::new(r"^(\d+)x(\d+)x(\d+)$").unwrap();
    }

    Ok(
      RE.captures(s)
        .map(|cap| Dimension {
          l: cap[1].parse::<u32>().unwrap(),
          w: cap[2].parse::<u32>().unwrap(),
          h: cap[3].parse::<u32>().unwrap(),
        })
        .unwrap(),
    )
  }
}
impl Dimension {
  fn calc_paper(&self) -> u32 {
    let sides = vec![&self.l * &self.w, &self.w * &self.h, &self.h * &self.l];
    let min = sides.iter().min();
    (2 * sides[0] + 2 * sides[1] + 2 * sides[2]) + min.unwrap()
  }

  fn calc_ribbon(&self) -> u32 {
    let mut sides: Vec<u32> = vec![self.l, self.w, self.h];
    sides.sort();
    return (sides[0] * 2) + (sides[1] * 2) + (&self.l * &self.w * &self.h);
  }
}

fn read_input() -> String {
  return io::read_input("2015-02");
}

fn parse_input(input: &str) -> Vec<Dimension> {
  input
    .lines()
    .map(|l| Dimension::from_str(l).unwrap())
    .collect()
}

fn part_one(input: &str) -> u32 {
  parse_input(input).iter().map(|dim| dim.calc_paper()).sum()
}

fn part_two(input: &str) -> u32 {
  parse_input(input).iter().map(|dim| dim.calc_ribbon()).sum()
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
  fn test_dimension_from_str() {
    assert_eq!(
      Dimension::from_str("2x3x4").unwrap(),
      Dimension { l: 2, w: 3, h: 4 }
    );
    assert_eq!(
      Dimension::from_str("1x1x10").unwrap(),
      Dimension { l: 1, w: 1, h: 10 }
    );
  }

  #[test]
  fn test_calc_paper() {
    assert_eq!(Dimension { l: 2, w: 3, h: 4 }.calc_paper(), 52 + 6);
    assert_eq!(Dimension { l: 1, w: 1, h: 10 }.calc_paper(), 42 + 1);
  }

  #[test]
  fn test_calc_ribbon() {
    assert_eq!(Dimension { l: 2, w: 3, h: 4 }.calc_ribbon(), 10 + 24);
    assert_eq!(Dimension { l: 1, w: 1, h: 10 }.calc_ribbon(), 4 + 10);
  }

  #[test]
  fn test_part_one() {
    assert_eq!(part_one("2x3x4\n1x1x10"), 58 + 43);
    assert_eq!(part_one(&read_input()), 1606483);
  }

  #[test]
  fn test_part_two() {
    assert_eq!(part_two("2x3x4\n1x1x10"), 34 + 14);
    assert_eq!(part_two(&read_input()), 3842356);
  }
}
