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

fn parse_input(input: &str) -> Vec<&str> {
  input.split("\n\n").collect()
}

fn has_required_fields(passport: &str) -> bool {
  let required = vec!["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];
  for n in &required {
    if !passport.contains(n) {
      return false;
    }
  }
  return true;
}

fn valid_pid(pid: &str) -> bool {
  pid.len() == 9 && pid.parse::<u32>().is_ok()
}

fn valid_hcl(hcl: &str) -> bool {
  if hcl.starts_with("#") {
    let c = hcl.trim_start_matches("#");
    return c.len() == 6 && c.chars().all(|c| c.is_ascii_hexdigit());
  }
  return false;
}

fn valid_ecl(ecl: &str) -> bool {
  let valid = vec!["amb", "blu", "brn", "gry", "grn", "hzl", "oth"];
  ecl.len() == 3 && valid.contains(&ecl)
}
fn valid_hgt(hgt: &str) -> bool {
  if hgt.ends_with("in") {
    let n = hgt.trim_end_matches("in").parse::<u8>().unwrap();
    return n >= 59 && n <= 76;
  } else if hgt.ends_with("cm") {
    let n = hgt.trim_end_matches("cm").parse::<u8>().unwrap();
    return n >= 150 && n <= 193;
  }
  return false;
}

fn valid_byr(byr: &str) -> bool {
  let n = byr.parse::<u16>().unwrap();
  n >= 1920 && n <= 2002
}

fn valid_iyr(iyr: &str) -> bool {
  let n = iyr.parse::<u16>().unwrap();
  n >= 2010 && n <= 2020
}

fn valid_eyr(eyr: &str) -> bool {
  let n = eyr.parse::<u16>().unwrap();
  n >= 2020 && n <= 2030
}

fn is_valid(passport: &str) -> bool {
  if !has_required_fields(passport) {
    return false;
  }
  for entries in passport.split_whitespace() {
    let split: Vec<&str> = entries.split(":").into_iter().collect();
    let field = split[0];
    let val = split[1];
    let valid = match field {
      "byr" => valid_byr(val),
      "iyr" => valid_iyr(val),
      "eyr" => valid_eyr(val),
      "hgt" => valid_hgt(val),
      "hcl" => valid_hcl(val),
      "ecl" => valid_ecl(val),
      "pid" => valid_pid(val),
      "cid" => true,
      _ => panic!("impossiburu"),
    };
    if !valid {
      return false;
    }
  }
  return true;
}

fn part_one(input: &str) -> usize {
  parse_input(input)
    .iter()
    .filter(|passport| has_required_fields(passport))
    .count()
}

fn part_two(input: &str) -> usize {
  parse_input(input)
    .iter()
    .filter(|passport| is_valid(passport))
    .count()
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
  fn test_has_required_fields() {
    assert!(has_required_fields(
      "ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
byr:1937 iyr:2017 cid:147 hgt:183cm"
    ));
    assert!(!has_required_fields(
      "iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884
hcl:#cfa07d byr:1929"
    ));
    assert!(has_required_fields(
      "hcl:#ae17e1 iyr:2013
eyr:2024
ecl:brn pid:760753108 byr:1931
hgt:179cm"
    ));
  }

  #[test]
  fn test_is_valid() {
    assert!(!is_valid(
      "eyr:1972 cid:100
hcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926"
    ));
    assert!(is_valid(
      "pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980
hcl:#623a2f"
    ));
    assert!(is_valid(
      "pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980
hcl:#623a2f"
    ));
  }

  #[test]
  fn test_valid_pid() {
    assert!(valid_pid("000000001"));
    assert!(!valid_pid("0123456789"));
  }

  #[test]
  fn test_valid_hcl() {
    assert!(valid_hcl("#123abc"));
    assert!(!valid_hcl("#123abz"));
  }

  #[test]
  fn test_valid_ecl() {
    assert!(valid_ecl("hzl"));
    assert!(!valid_ecl("wat"));
  }

  #[test]
  fn test_valid_hgt() {
    assert!(valid_hgt("60in"));
    assert!(!valid_hgt("190in"));
    assert!(valid_hgt("190cm"));
    assert!(!valid_hgt("190"));
  }

  #[test]
  fn test_valid_byr() {
    assert!(valid_byr("2002"));
    assert!(!valid_byr("2003"));
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
