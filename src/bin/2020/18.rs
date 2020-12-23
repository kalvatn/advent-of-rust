#![allow(
  unused_variables,
  unused_imports,
  unused_assignments,
  dead_code,
  deprecated,
  unused_parens
)]

use std::time::Instant;

use common::io;
use itertools::Itertools;
use std::borrow::Borrow;
use std::collections::VecDeque;

fn read_input() -> String {
  return io::read_input("2020-18");
}

fn parse_input(input: &str) -> String {
  return input.replace(" ", "");
}


fn eval_expr_rec(stack: &mut VecDeque<char>) -> usize {
  let mut sum = 0usize;
  let mut plus = true;
  while !stack.is_empty() {
    let c = stack.pop_front().unwrap();
    if c.is_digit(10) {
      let val = c.to_digit(10).unwrap() as usize;
      sum = if plus {
        sum + val
      } else {
        sum * val
      };
    } else {
      match c {
        '+' => plus = true,
        '*' => {
          plus = false
        },
        '(' => {
          let val = eval_expr_rec(stack);
          sum = if plus {
            sum + val
          } else {
            sum * val
          };
        },
        ')' => {
          break;
        },
        _ => {}
      }
    }
  }
  sum
}

fn eval_expr(expr:&str) -> usize {
  let expr = parse_input(expr);
  let mut stack = VecDeque::<char>::new();
  for c in expr.chars() {
    stack.push_back(c);
  }
  eval_expr_rec(&mut stack)
}

fn part_one(input: &str) -> usize {
  let parsed = parse_input(input);
  let mut sum = 0;
  for line in parsed.lines() {
    sum += eval_expr(line);
  }
  return sum;
}

fn part_two(input: &str) -> usize {
  return 0;
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

  const TEST_INPUT: &str = "";

  #[test]
  fn test_eval_expr() {
    assert_eq!(eval_expr("1"), 1);
    assert_eq!(eval_expr("1 + 2"), 3);
    assert_eq!(eval_expr("1 * 2"), 2);
    assert_eq!(eval_expr("1 * 2 * 3"), 6);
    assert_eq!(eval_expr("1 + (2 * 3)"), 7);
    assert_eq!(eval_expr("(1 * 2) + (2 * 3)"), 8);
    assert_eq!(eval_expr("(1 * 2) * (2 * 3)"), 12);
    assert_eq!(eval_expr("(1 * (2+3)) * (2 * 3)"), 30);
  }

  #[test]
  fn test_part_one() {
    assert_eq!(part_one("2 * 3 + (4 * 5)"), 26);
    assert_eq!(part_one("5 + (8 * 3 + 9 + 3 * 4 * 3)"), 437);
    assert_eq!(part_one("5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))"), 12240);
    assert_eq!(part_one("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2"), 13632);
    assert_eq!(part_one(&read_input()), 3647606140187);
  }

  #[test]
  fn test_part_two() {
    assert_eq!(part_two(TEST_INPUT), 0);
    // assert_eq!(part_two(&read_input()), 0);
  }
}
