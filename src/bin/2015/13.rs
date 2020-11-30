use std::collections::HashMap;
use std::time::Instant;

use itertools::Itertools;
use lazy_static::lazy_static;
use regex::Regex;

use common::io;

lazy_static! {
  static ref RE: Regex =
    Regex::new(r"^(\w+) would (gain|lose) (\d+) happiness units by sitting next to (\w+).$")
      .unwrap();
}

#[derive(Eq, PartialEq, Hash, Debug)]
struct Person {
  name: String,
}

#[derive(Eq, PartialEq, Hash, Debug)]
struct HappinessRule {
  person1: Person,
  person2: Person,
  gain: i32,
}

fn read_input() -> String {
  return io::read_input("2015-13");
}

fn parse_input(input: &str) -> Vec<HappinessRule> {
  input
    .lines()
    .map(|line| {
      RE.captures(line)
        .map(|c| {
          let mut units = c[3].parse::<i32>().unwrap();
          if c[2].to_string() == "lose" {
            units *= -1;
          }
          HappinessRule {
            person1: Person {
              name: c[1].to_string(),
            },
            person2: Person {
              name: c[4].to_string(),
            },
            gain: units,
          }
        })
        .unwrap()
    })
    .into_iter()
    .collect()
}

#[allow(unused_variables)]
fn part_one(input: &str) -> i32 {
  let rules = parse_input(input);
  let mut lookup: HashMap<Person, HashMap<Person, i32>> = HashMap::new();
  for rule in rules {
    lookup
      .entry(rule.person1)
      .or_insert(HashMap::new())
      .insert(rule.person2, rule.gain);
  }

  let persons: Vec<&Person> = lookup.keys().into_iter().collect();
  let max_index: usize = persons.len() - 1;
  let mut best = 0;
  let permutations = persons.iter().permutations(persons.len());
  permutations.for_each(|perm| {
    let mut happiness = 0;
    let enumerate: Vec<(usize, &&&Person)> = perm.iter().enumerate().collect();
    for person_indexed in &enumerate {
      let index = person_indexed.0;
      let person = person_indexed.1;
      let (left_index, right_index) = match index {
        0 => (max_index, index + 1),
        _ if index >= max_index => (index - 1, 0),
        _ => (index - 1, index + 1),
      };
      let left = &enumerate[left_index].1;
      let right = &enumerate[right_index].1;
      happiness += lookup[&person][left];
      happiness += lookup[&person][right];
    }
    if happiness > best {
      best = happiness;
    }
  });
  return best;
}

#[allow(unused_variables)]
fn part_two(input: &str) -> i32 {
  let rules = parse_input(input);
  let mut lookup: HashMap<String, HashMap<String, i32>> = HashMap::new();
  let me = Person {
    name: "kalvatn".to_string(),
  };
  for rule in rules {
    let person1 = rule.person1;
    let person2 = rule.person2;
    lookup
      .entry(person1.name.to_string())
      .or_insert(HashMap::new())
      .insert(person2.name.to_string(), rule.gain);
    lookup
      .entry(person1.name.to_string())
      .or_insert(HashMap::new())
      .insert(me.name.to_string(), 0);
    lookup
      .entry(me.name.to_string())
      .or_insert(HashMap::new())
      .insert(person2.name.to_string(), 0);
  }

  let persons: Vec<String> = lookup.keys().into_iter().cloned().collect();
  let max_index: usize = persons.len() - 1;
  let mut best = 0;
  let permutations = persons.iter().permutations(persons.len());
  permutations.for_each(|perm| {
    let mut happiness = 0;
    let enumerate: Vec<(usize, &&String)> = perm.iter().enumerate().collect();
    for person_indexed in &enumerate {
      let index = person_indexed.0;
      let person = person_indexed.1.to_string();
      let (left_index, right_index) = match index {
        0 => (max_index, index + 1),
        _ if index >= max_index => (index - 1, 0),
        _ => (index - 1, index + 1),
      };
      let left = &enumerate[left_index].1.to_string();
      let right = &enumerate[right_index].1.to_string();
      happiness += lookup[&person][left];
      happiness += lookup[&person][right];
    }
    if happiness > best {
      best = happiness;
    }
  });
  return best;
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

  const TEST_INPUT: &str = "Alice would gain 54 happiness units by sitting next to Bob.
Alice would lose 79 happiness units by sitting next to Carol.
Alice would lose 2 happiness units by sitting next to David.
Bob would gain 83 happiness units by sitting next to Alice.
Bob would lose 7 happiness units by sitting next to Carol.
Bob would lose 63 happiness units by sitting next to David.
Carol would lose 62 happiness units by sitting next to Alice.
Carol would gain 60 happiness units by sitting next to Bob.
Carol would gain 55 happiness units by sitting next to David.
David would gain 46 happiness units by sitting next to Alice.
David would lose 7 happiness units by sitting next to Bob.
David would gain 41 happiness units by sitting next to Carol.";

  #[test]
  fn test_part_one() {
    assert_eq!(part_one(TEST_INPUT), 330);
    assert_eq!(part_one(&read_input()), 618);
  }

  #[test]
  fn test_part_two() {
    assert_eq!(part_two(&read_input()), 601);
  }
}
