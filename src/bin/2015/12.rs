use std::time::Instant;

use lazy_static::lazy_static;
use regex::Regex;

use common::io;
use serde_json::Value;

lazy_static! {
  static ref RE: Regex = Regex::new(r"([-?\d+]+)").unwrap();
}

fn read_input() -> String {
  return io::read_input("2015-12");
}

fn extract_numbers(json: &str) -> Vec<i32> {
  return RE
    .captures_iter(json)
    .map(|cap| cap[1].parse::<i32>().unwrap())
    .collect();
}

fn extract_numbers_no_red(json: &str) -> Vec<i32> {
  let mut sum: Vec<i32> = Vec::new();
  match json.parse::<i32>() {
    Ok(n) => {
      sum.push(n);
    }
    Err(_) => {
      let v: Value = serde_json::from_str(json).unwrap();
      if v.is_array() {
        let a = v.as_array().unwrap();
        a.iter()
          .for_each(|v| sum.extend(extract_numbers_no_red(v.to_string().as_str())));
      } else if v.is_object() {
        let o = v.as_object().unwrap();
        if !o
          .values()
          .any(|v| v.is_string() && v.as_str().unwrap() == "red")
        {
          o.values()
            .for_each(|v| sum.extend(extract_numbers_no_red(v.to_string().as_str())));
        }
      }
    }
  }
  return sum;
}

#[allow(unused_variables)]
fn part_one(input: &str) -> i32 {
  return extract_numbers(input).iter().sum();
}

#[allow(unused_variables)]
fn part_two(input: &str) -> i32 {
  return extract_numbers_no_red(input).iter().sum();
}

fn main() {
  let input = read_input();

  let p1_timer = Instant::now();
  println!(
    "part one {} {}ms",
    part_one(&input),
    p1_timer.elapsed().as_millis()
  );
  let p2_timer = Instant::now();
  println!(
    "part two {} {}ms",
    part_two(&input),
    p2_timer.elapsed().as_millis()
  );
  println!("total {}ms", p1_timer.elapsed().as_millis())
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn test_extract_numbers() {
    assert_eq!(extract_numbers("[1,2,3]"), vec![1, 2, 3]);
    assert_eq!(extract_numbers("{\"a\":2,\"b\":4}"), vec![2, 4]);
    assert_eq!(extract_numbers("{\"a\":[-1,1]}"), vec![-1, 1]);
    assert_eq!(extract_numbers("[-1,{\"a\":1}]"), vec![-1, 1]);
  }

  #[test]
  fn test_extract_numbers_no_red2() {
    assert_eq!(extract_numbers_no_red("[1,2,3]"), vec![1, 2, 3]);
    assert_eq!(
      extract_numbers_no_red("[1,{\"c\":\"red\",\"b\":2},3]"),
      vec![1, 3]
    );
    let empty: Vec<i32> = vec![];
    assert_eq!(
      extract_numbers_no_red("{\"d\":\"red\",\"e\":[1,2,3,4],\"f\":5}"),
      empty
    );
    assert_eq!(extract_numbers_no_red("[1,\"red\",5]"), vec![1, 5]);
  }

  #[test]
  fn test_part_one() {
    assert_eq!(part_one(&read_input()), 191164);
  }

  #[test]
  fn test_part_two() {
    assert_eq!(part_two(&read_input()), 87842);
  }
}
