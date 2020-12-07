#![allow(unused_variables, unused_imports)]

use itertools::Itertools;
use lazy_static::lazy_static;
use regex::Regex;
use std::collections::{HashMap, HashSet, VecDeque};
use std::time::Instant;

use common::io;
use std::borrow::{Borrow, BorrowMut};
use std::collections::hash_map::RandomState;

#[derive(Debug)]
struct Bag {
  name: String,
  contains: HashMap<String, u32>,
  contained_by: Vec<String>,
}

fn read_input() -> String {
  return io::read_input("2020-07");
}

fn parse_input(input: &str) -> HashMap<String, Vec<(u32, String)>> {
  lazy_static! {
    static ref RE_HAS_BAGS: Regex = Regex::new(r"^([\w+\s]+) bags contain").unwrap();
    static ref RE_YES_BAGS: Regex = Regex::new(r"(\d+) ([\w+\s]+) bags?").unwrap();
  }
  let mut map: HashMap<String, Vec<(u32, String)>> = HashMap::new();
  for line in input.lines() {
    if RE_HAS_BAGS.is_match(line) {
      RE_HAS_BAGS.captures(line).map(|cap| {
        let name: String = cap[1].to_string();
        let contains: Vec<(u32, String)> = vec![];
        let vec = map.entry(name.clone()).or_insert(contains);
        let split = line.split(",").collect::<Vec<&str>>();
        split.iter().for_each(|l| {
          RE_YES_BAGS.captures(l.trim()).map(|cap| {
            let count = cap[1].parse::<u32>().unwrap();
            let other_name: String = cap[2].to_string();
            vec.push((count, other_name.clone()));
          });
        });
      });
    }
  }
  return map;
}

fn part_one(input: &str) -> usize {
  let input_map = parse_input(input);
  let mut bag_map: HashMap<String, Bag> = HashMap::new();
  for (k, v) in &input_map {
    println!("{} contains {:?}", k, v);
    let mut map1 = HashMap::new();
    for (count, other) in v {
      map1.insert(other.to_owned(), count.to_owned());
    }
    let bag = Bag {
      name: k.to_string(),
      contains: map1.to_owned(),
      contained_by: vec![],
    };
    bag_map.insert(k.to_string(), bag);
  }
  let mut contained_by: HashMap<String, HashSet<String>> = HashMap::new();
  for x in bag_map.values() {
    for c in x.contains.keys() {
      contained_by
        .entry(c.to_string())
        .or_insert(HashSet::new())
        .insert(x.name.to_string());
    }
  }
  for (k, mut v) in bag_map.borrow_mut() {
    if contained_by.contains_key(k) {
      for x in contained_by.get(k).unwrap() {
        v.contained_by.push(x.to_string());
      }
    }
    println!("{} {:?}", k, v);
  }

  let mut colors = HashSet::<String>::new();
  let mut queue = VecDeque::<&Bag>::new();
  queue.push_front(bag_map.get("shiny gold").unwrap());
  while !queue.is_empty() {
    let bag = queue.pop_front().unwrap();
    for b in &bag.contained_by {
      colors.insert(b.to_string());
      queue.push_back(bag_map.get(b).unwrap())
    }
  }
  return colors.len();
}

fn part_two(input: &str) -> usize {
  let input_map = parse_input(input);
  let mut bag_map: HashMap<String, Bag> = HashMap::new();
  for (k, v) in &input_map {
    println!("{} contains {:?}", k, v);
    let mut map1 = HashMap::new();
    for (count, other) in v {
      map1.insert(other.to_owned(), count.to_owned());
    }
    let bag = Bag {
      name: k.to_string(),
      contains: map1.to_owned(),
      contained_by: vec![],
    };
    bag_map.insert(k.to_string(), bag);
  }
  let mut contained_by: HashMap<String, HashSet<String>> = HashMap::new();
  for x in bag_map.values() {
    for c in x.contains.keys() {
      contained_by
        .entry(c.to_string())
        .or_insert(HashSet::new())
        .insert(x.name.to_string());
    }
  }
  for (k, v) in bag_map.borrow_mut() {
    if contained_by.contains_key(k) {
      for x in contained_by.get(k).unwrap() {
        v.contained_by.push(x.to_string());
      }
    }
    println!("{} {:?}", k, v);
  }

  let mut queue = VecDeque::<&Bag>::new();
  queue.push_front(bag_map.get("shiny gold").unwrap());
  let mut count = 0;
  while !queue.is_empty() {
    let bag = queue.pop_front().unwrap();
    println!("{:?}", bag);
    for (k, v) in &bag.contains {
      count += v;
      for i in 0..*v {
        queue.push_back(bag_map.get(k).unwrap())
      }
    }
  }
  return count as usize;
}

fn main() {
  let input = read_input();
  println!("{:?}", input);

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

  const TEST_INPUT: &str = "light red bags contain 1 bright white bag, 2 muted yellow bags.
dark orange bags contain 3 bright white bags, 4 muted yellow bags.
bright white bags contain 1 shiny gold bag.
muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
dark olive bags contain 3 faded blue bags, 4 dotted black bags.
vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
faded blue bags contain no other bags.
dotted black bags contain no other bags.";

  #[test]
  fn test_part_one() {
    assert_eq!(part_one(TEST_INPUT), 4);
    assert_eq!(part_one(&read_input()), 326);
  }

  #[test]
  fn test_part_two() {
    assert_eq!(part_two(TEST_INPUT), 32);
    assert_eq!(part_two(&read_input()), 5635);
  }
}
