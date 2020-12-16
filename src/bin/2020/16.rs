use std::collections::HashMap;
use std::time::Instant;

use itertools::Itertools;

use common::io;

fn read_input() -> String {
  return io::read_input("2020-16");
}

fn parse_input(input: &str) -> (HashMap<String, Vec<usize>>, Vec<Vec<usize>>, Vec<usize>) {
  let sections = input.split("\n\n").collect::<Vec<&str>>();

  let field_rules = sections[0]
    .lines()
    .map(|line| {
      let (rule, r1, r2, r3, r4): (&str, usize, usize, usize, usize) =
        serde_scan::scan!("{}: {}-{} or {}-{}" <- line).unwrap();
      (rule.to_string(), (r1..r2 + 1).chain(r3..r4 + 1).collect())
    })
    .collect();

  let my_ticket: Vec<usize> = sections[1]
    .lines()
    .nth(1) // skip 'your ticket:'
    .map(|line| {
      line
        .split(",")
        .map(|c| c.parse::<usize>().unwrap())
        .collect::<Vec<usize>>()
    })
    .unwrap();

  let nearby_tickets = sections[2]
    .lines()
    .dropping(1) // drop nearby tickets:
    .map(|line| {
      line
        .split(",")
        .map(|c| c.parse::<usize>().unwrap())
        .collect()
    })
    .collect();
  return (field_rules, nearby_tickets, my_ticket);
}

fn invalid_fields_for_ticket(
  ticket: &Vec<usize>,
  field_rules: &HashMap<String, Vec<usize>>,
) -> Vec<usize> {
  ticket
    .into_iter()
    .filter(|field| !valid_field(field, field_rules))
    .cloned()
    .collect()
}

fn valid_ticket(ticket: &Vec<usize>, field_rules: &HashMap<String, Vec<usize>>) -> bool {
  ticket.iter().all(|field| valid_field(field, field_rules))
}

fn valid_field(field: &usize, field_rules: &HashMap<String, Vec<usize>>) -> bool {
  field_rules.iter().any(|(_, range)| range.contains(field))
}

fn rule_applies_to_all_tickets(
  tickets: &Vec<Vec<usize>>,
  field_index: &usize,
  rule_range: &Vec<usize>,
) -> bool {
  tickets
    .into_iter()
    .map(|ticket| ticket[*field_index])
    .all(|field| rule_range.contains(&field))
}

fn part_one(field_rules: &HashMap<String, Vec<usize>>, nearby_tickets: &Vec<Vec<usize>>) -> usize {
  let all_nearby_ticket_fields = nearby_tickets.into_iter().flatten().cloned().collect();
  return invalid_fields_for_ticket(&all_nearby_ticket_fields, field_rules)
    .into_iter()
    .sum();
}

fn part_two(
  field_rules: &HashMap<String, Vec<usize>>,
  nearby_tickets: &Vec<Vec<usize>>,
  my_ticket: &Vec<usize>,
  find_fields: Vec<&str>,
) -> usize {
  let valid_tickets = nearby_tickets
    .into_iter()
    .filter(|ticket| valid_ticket(ticket, &field_rules))
    .cloned()
    .collect();

  let field_pos_candidates_sorted = (0..my_ticket.len())
    .map(|index| {
      (
        index,
        field_rules
          .into_iter()
          .filter(|(_, valid_range)| {
            rule_applies_to_all_tickets(&valid_tickets, &index, valid_range)
          })
          .map(|(field_name, _)| field_name)
          .collect::<Vec<&String>>(),
      )
    })
    .into_iter()
    .sorted_by(|a, b| Ord::cmp(&a.1.len(), &b.1.len()))
    .collect::<Vec<(usize, Vec<&String>)>>();

  let mut field_positions = HashMap::<usize, String>::new();
  let mut found = vec![];
  for (index, mut field_names) in field_pos_candidates_sorted {
    field_names.retain(|k| !found.contains(*k));
    found.extend(field_names.into_iter().cloned());
    field_positions.insert(index, found.last().unwrap().clone());
  }

  field_positions
    .iter()
    .filter(|(_, v)| find_fields.iter().any(|f| v.starts_with(f)))
    .map(|(k, _)| my_ticket[*k])
    .product()
}

fn main() {
  let input = read_input();
  let (field_rules, nearby_tickets, my_ticket) = parse_input(&input);

  let time = Instant::now();
  let p1 = part_one(&field_rules, &nearby_tickets);
  let p1_time = time.elapsed();

  let time = Instant::now();
  let p2 = part_two(&field_rules, &nearby_tickets, &my_ticket, vec!["departure"]);
  let p2_time = time.elapsed();
  println!("part one {:?} {:?}", p1, p1_time);
  println!("part two {:?} {:?}", p2, p2_time);
}

#[cfg(test)]
mod test {
  use super::*;

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

  #[test]
  fn test_part_one() {
    let (field_rules, nearby_tickets, _) = parse_input(TEST_INPUT);
    assert_eq!(part_one(&field_rules, &nearby_tickets), 71);
    let (field_rules, nearby_tickets, _) = parse_input(&read_input());
    assert_eq!(part_one(&field_rules, &nearby_tickets), 22073);
  }

  #[test]
  fn test_part_two() {
    let (field_rules, nearby_tickets, my_ticket) = parse_input(TEST_INPUT_2);
    assert_eq!(
      part_two(
        &field_rules,
        &nearby_tickets,
        &my_ticket,
        vec!["row", "seat", "class"]
      ),
      11 * 12 * 13
    );
    let (field_rules, nearby_tickets, my_ticket) = parse_input(&read_input());
    assert_eq!(
      part_two(&field_rules, &nearby_tickets, &my_ticket, vec!["departure"]),
      1346570764607
    );
  }
}
