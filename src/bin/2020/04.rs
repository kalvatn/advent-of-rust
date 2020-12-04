#![allow(unused_variables, unused_imports)]

use std::collections::{HashMap, HashSet};
use std::time::Instant;

use itertools::Itertools;
use lazy_static::lazy_static;
use regex::Regex;

use common::io;

fn read_input() -> String {
  return io::read_input("2020-04");
}

fn parse_input(input: &str) -> &str {
  return input;
}

fn is_valid(s: &str) -> bool {
  let needed = vec!["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];
  // "cid"];
  for n in &needed {
    if !s.contains(n) {
      return false;
    }
  }
  return true;
}

fn part_one(input: &str) -> usize {
  let mut count = 0;

  let mut current = String::new();

  for line in input.lines() {
    if line.is_empty() {
      if is_valid(&current) {
        count += 1;
      }
      current = String::new();
    }
    for c in line.chars() {
      current.push(c);
    }
  }
  println!("{:?}", current);

  if is_valid(&current) {
    count += 1;
  }
  return count;
}

fn is_valid_2(s: &str) -> bool {
  let needed = vec!["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];
  println!("{:?}", s);
  for pass in s.trim().split_whitespace() {
    println!("pass {:?}", pass);
    let split: Vec<&str> = pass.split(":").map(|it| it.trim()).into_iter().collect();
    let field = split[0];
    let val = split[1];
    println!("field {:?} {:?}", field, val);
    let valid = match field {
      "byr" => {
        let n = val.parse::<u16>().unwrap();
        n >= 1920 && n <= 2002
      }
      "iyr" => {
        let n = val.parse::<u16>().unwrap();
        n >= 2010 && n <= 2020
      }
      "eyr" => {
        let n = val.parse::<u16>().unwrap();
        n >= 2020 && n <= 2030
      }
      "hgt" => {
        if val.ends_with("in") {
          let n = val.trim_end_matches("in").parse::<u8>().unwrap();
          n >= 59 && n <= 76
        } else {
          let n = val.trim_end_matches("cm").parse::<u8>().unwrap();
          n >= 150 && n <= 193
        }
      }
      "hcl" => {
        if !val.starts_with("#") {
          false
        } else {
          let c = val.trim_start_matches("#");
          c.len() == 6 && c.chars().all(|c| c.is_alphanumeric())
        }
      }
      "ecl" => {
        let valid = vec!["amb", "blu", "brn", "gry", "grn", "hzl", "oth"];
        let mut count = 0;
        for v in valid {
          if val == v {
            count += 1;
          }
        }
        count == 1
      }
      "pid" => val.trim().len() == 9 && val.parse::<u32>().unwrap() >= 0,
      "cid" => true,
      _ => panic!("impossiburu"),
    };
    if !valid {
      return false;
    }
  }
  for n in &needed {
    if !s.contains(n) {
      return false;
    }
  }
  return true;
}

fn part_two(input: &str) -> usize {
  let mut count = 0;

  let mut current = String::new();

  for line in input.lines() {
    if line.is_empty() {
      if is_valid_2(&current) {
        count += 1;
      }
      current = String::new();
    }
    for c in line.chars() {
      current.push(c);
    }
    current.push(' ');
  }

  if is_valid_2(&current) {
    count += 1;
  }
  return count;
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

  const TEST_INPUT: &str = "ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
byr:1937 iyr:2017 cid:147 hgt:183cm

iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884
hcl:#cfa07d byr:1929

hcl:#ae17e1 iyr:2013
eyr:2024
ecl:brn pid:760753108 byr:1931
hgt:179cm

hcl:#cfa07d eyr:2025 pid:166559648
iyr:2011 ecl:brn hgt:59in";

  #[test]
  fn test_is_valid() {
    assert!(is_valid(
      "ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
byr:1937 iyr:2017 cid:147 hgt:183cm"
    ));
    assert!(!is_valid(
      "iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884
hcl:#cfa07d byr:1929"
    ));
    assert!(is_valid(
      "hcl:#ae17e1 iyr:2013
eyr:2024
ecl:brn pid:760753108 byr:1931
hgt:179cm
"
    ));
  }

  #[test]
  fn test_is_valid_2() {
    assert!(!is_valid_2(
      "eyr:1972 cid:100
hcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926"
    ));
    assert!(is_valid_2(
      "pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980
hcl:#623a2f"
    ));
    assert!(is_valid_2(
      "pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980
hcl:#623a2f"
    ));
  }

  #[test]
  fn test_part_one() {
    assert_eq!(part_one(TEST_INPUT), 2);
    assert_eq!(part_one(&read_input()), 219);
  }

  #[test]
  fn test_part_two() {
    assert_eq!(
      part_two(
        "eyr:1972 cid:100
hcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926

iyr:2019
hcl:#602927 eyr:1967 hgt:170cm
ecl:grn pid:012533040 byr:1946

hcl:dab227 iyr:2012
ecl:brn hgt:182cm pid:021572410 eyr:2020 byr:1992 cid:277

hgt:59cm ecl:zzz
eyr:2038 hcl:74454a iyr:2023
pid:3556412378 byr:2007"
      ),
      0
    );
    assert_eq!(
      part_two(
        "pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980
hcl:#623a2f

eyr:2029 ecl:blu cid:129 byr:1989
iyr:2014 pid:896056539 hcl:#a97842 hgt:165cm

hcl:#888785
hgt:164cm byr:2001 iyr:2015 cid:88
pid:545766238 ecl:hzl
eyr:2022

iyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:093154719"
      ),
      4
    );
    assert_eq!(part_two(&read_input()), 127);
  }
}
