#![allow(
  unused_variables,
  unused_imports,
  unused_assignments,
  dead_code,
  deprecated,
  unused_parens,
  unused_mut
)]

use std::collections::{HashMap, HashSet};
use std::time::Instant;

use futures::FutureExt;
use itertools::{all, Itertools};
use lazy_static::lazy_static;
use regex::Regex;

use common::io;
use std::borrow::BorrowMut;

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

fn filter_columns(
  rules: &HashMap<String, Vec<usize>>,
  my_ticket: &Vec<usize>,
  valid_tickets: &Vec<Vec<usize>>,
  found: &mut HashMap<usize, String>,
) -> HashMap<usize, HashSet<String>> {
  let mut field_types = HashMap::<usize, HashSet<String>>::new();
  for (rule, valid_range) in rules {
    for (i, field) in my_ticket.iter().enumerate() {
      if found.contains_key(&i) {
        for (k, v) in field_types.borrow_mut() {
          if k != &i {
            v.remove(&found[&i]);
          }
        }
        field_types.insert(i, HashSet::new());
        field_types.get_mut(&i).unwrap().insert(found[&i].clone());
      } else {
        let mut all_match = valid_range.contains(field);
        if all_match {
          for ticket in valid_tickets {
            if !valid_range.contains(&ticket[i]) {
              all_match = false;
            }
          }
          if all_match {
            field_types
              .entry(i)
              .or_insert(HashSet::new())
              .insert(rule.to_string());
          }
        }
      }
    }
  }
  let mut all_found = true;
  for (i, r) in &field_types {
    if r.len() == 1 || found.contains_key(i) {
      let name = r.into_iter().nth(0).unwrap().to_string();
      found.insert(*i, name.to_string());
    } else {
      all_found = false;
    }
  }
  if all_found {
    return field_types;
  }
  return filter_columns(rules, my_ticket, valid_tickets, found);
}

fn part_two(input: &str) -> usize {
  let (rules, nearby_tickets, my_ticket) = parse_input(input);
  let mut valid_tickets = vec![];
  for nearby in nearby_tickets {
    if get_invalid(&nearby, &rules).is_empty() {
      valid_tickets.push(nearby)
    }
  }
  let mut found = HashMap::<usize, String>::new();
  let field_types = filter_columns(&rules, &my_ticket, &valid_tickets, &mut found);
  let mut result = 1;
  for (k, v) in &field_types {
    if v.iter().nth(0).unwrap().starts_with("departure") {
      result *= my_ticket[*k];
    }
  }
  result
}

fn main() {
  // let input = TEST_INPUT_2;
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
    // assert_eq!(part_two(TEST_INPUT_2), 0);
    assert_eq!(part_two(&read_input()), 1346570764607);
  }
}
