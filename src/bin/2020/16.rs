#![allow(
  unused_variables,
  unused_imports,
  unused_assignments,
  dead_code,
  deprecated,
  unused_parens,
  unused_mut
)]

use std::borrow::BorrowMut;
use std::collections::{HashMap, HashSet};
use std::time::Instant;

use futures::FutureExt;
use itertools::{all, Itertools};
use lazy_static::lazy_static;
use regex::Regex;

use common::io;

const TEST_INPUT: &str = "class: 1-3 or 5-7
row: 6-11 or 33-44
seat: 13-40 or 45-50

your ticket:
7,1,14

nearby tickets:
7,3,47
40,4,50
55,2,20
38,6,12";
const TEST_INPUT_2: &str = "class: 0-1 or 4-19
row: 0-5 or 8-19
seat: 0-13 or 16-19

your ticket:
11,12,13

nearby tickets:
3,9,18
15,1,5
5,14,9";

fn read_input() -> String {
  return io::read_input("2020-16");
}

fn parse_input(input: &str) -> (HashMap<String, Vec<usize>>, Vec<Vec<usize>>, Vec<usize>) {
  let parts = input.split("\n\n").collect::<Vec<&str>>();
  let mut rules = HashMap::new();
  for line in parts[0].lines() {
    let (rule, r1, r2, r3, r4): (&str, usize, usize, usize, usize) =
      serde_scan::scan!("{}: {}-{} or {}-{}" <- line).unwrap();
    rules.insert(rule.to_string(), (r1..r2 + 1).chain(r3..r4 + 1).collect());
  }
  let my_ticket: Vec<usize> = parts[1]
    .lines()
    .dropping(1)
    .flat_map(|line| {
      line
        .split(",")
        .map(|c| c.parse::<usize>().unwrap())
        .collect::<Vec<usize>>()
    })
    .into_iter()
    .collect();
  let nearby_tickets = parts[2]
    .lines()
    .dropping(1)
    .map(|line| {
      line
        .split(",")
        .map(|c| c.parse::<usize>().unwrap())
        .collect()
    })
    .collect();
  return (rules, nearby_tickets, my_ticket);
}

fn get_invalid(tickets: &Vec<usize>, rules: &HashMap<String, Vec<usize>>) -> Vec<usize> {
  let mut invalid = vec![];
  for field in tickets {
    if !any_rule_matches(*field, rules) {
      invalid.push(*field);
    }
  }
  return invalid;
}

fn any_rule_matches(ticket: usize, rules: &HashMap<String, Vec<usize>>) -> bool {
  for (rule, range) in rules {
    if range.contains(&ticket) {
      return true;
    }
  }
  return false;
}

fn part_one(input: &str) -> usize {
  let (rules, nearby_tickets, my_ticket) = parse_input(input);
  let nearby_tickets: Vec<usize> = nearby_tickets.into_iter().flatten().collect();

  return get_invalid(&nearby_tickets, &rules).into_iter().sum();
}

fn part_two(input: &str) -> usize {
  let (rules, nearby_tickets, my_ticket) = parse_input(input);
  let mut valid_tickets = vec![];
  for nearby in nearby_tickets {
    if get_invalid(&nearby, &rules).is_empty() {
      valid_tickets.push(nearby)
    }
  }
  valid_tickets.push(my_ticket.clone());
  let mut candidates: HashMap<usize, HashSet<String>> = HashMap::new();

  for i in 0..my_ticket.len() {
    for (rule_name, rule_range) in &rules {
      let mut add_cand = true;
      for ticket in &valid_tickets {
        if !rule_range.contains(&ticket[i]) {
          add_cand = false;
          break;
        }
      }
      if add_cand {
        candidates
          .entry(i)
          .or_insert(HashSet::new())
          .insert(rule_name.to_string());
      }
    }
  }

  let mut sorted_candidates: Vec<(usize, HashSet<String>)> = candidates
    .into_iter()
    .sorted_by(|a, b| Ord::cmp(&a.1.len(), &b.1.len()))
    .collect::<Vec<(usize, HashSet<String>)>>();

  let mut pos = HashMap::<usize, String>::new();
  let mut found = HashSet::new();
  for (index, mut field_names) in sorted_candidates {
    field_names.retain(|k| !found.contains(k));
    let field_name = field_names.iter().nth(0).unwrap().to_string().clone();
    found.insert(field_name.clone());
    pos.insert(index, field_name.clone());
  }

  let mut result = 1;
  for (k, v) in &pos {
    if v.starts_with("departure") {
      result *= my_ticket[*k];
    }
  }
  result
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
  fn test_part_one() {
    assert_eq!(part_one(TEST_INPUT), 71);
    assert_eq!(part_one(&read_input()), 22073);
  }

  #[test]
  fn test_part_two() {
    assert_eq!(part_two(TEST_INPUT), 1);
    assert_eq!(part_two(&read_input()), 1346570764607);
  }
}
