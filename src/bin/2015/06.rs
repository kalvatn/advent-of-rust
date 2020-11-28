use std::time::Instant;

use lazy_static::lazy_static;
use regex::Regex;

use common::io;
use Action::{Toggle, TurnOff, TurnOn};

#[derive(Debug, PartialEq, Eq)]
enum Action {
  TurnOn,
  TurnOff,
  Toggle,
}

impl Action {
  fn from_string(string: &str) -> Action {
    match string {
      "turn on" => TurnOn,
      "turn off" => TurnOff,
      "toggle" => Toggle,
      _ => panic!("impossiburu"),
    }
  }
}

#[derive(Debug, PartialEq, Eq)]
struct Instruction {
  row_start: usize,
  col_start: usize,
  row_end: usize,
  col_end: usize,
  action: Action,
}

impl Instruction {
  fn from_string(string: &str) -> Instruction {
    lazy_static! {
      static ref RE: Regex =
        Regex::new(r"^(turn on|turn off|toggle) (\d+),(\d+) through (\d+),(\d+)$").unwrap();
    }
    RE.captures(string)
      .map(|cap| Instruction {
        row_start: cap[2].parse().unwrap(),
        row_end: cap[4].parse::<usize>().unwrap() + 1,
        col_start: cap[3].parse().unwrap(),
        col_end: cap[5].parse::<usize>().unwrap() + 1,
        action: Action::from_string(&cap[1]),
      })
      .unwrap()
  }
}

fn read_input() -> String {
  return io::read_input("2015-06");
}

fn parse_instructions(input: &str) -> Vec<Instruction> {
  return input
    .lines()
    .map(|line| Instruction::from_string(line))
    .collect();
}

type Grid = [[i64; 1000]];

fn mutate_grid(grid: &mut Grid, instructions: Vec<Instruction>, p1: bool) {
  for instruction in instructions {
    for y in instruction.row_start..instruction.row_end {
      for x in instruction.col_start..instruction.col_end {
        match instruction.action {
          TurnOn => {
            if p1 == true {
              grid[y][x] = 1;
            } else {
              grid[y][x] += 1;
            }
          }
          TurnOff => {
            if p1 == true {
              grid[y][x] = 0;
            } else {
              let cur = grid[y][x];
              grid[y][x] -= if cur > 0 { 1 } else { 0 };
            }
          }
          Toggle => {
            if p1 == true {
              let cur = grid[y][x];
              grid[y][x] = if cur == 0 { 1 } else { 0 };
            } else {
              grid[y][x] += 2;
            }
          }
        }
      }
    }
  }
}

fn part_one(input: &str) -> usize {
  let instructions = parse_instructions(&input);
  let mut grid = vec![[0; 1000]; 1000];
  mutate_grid(&mut grid, instructions, true);
  return grid.iter().flatten().filter(|light| **light == 1).count();
}

fn part_two(input: &str) -> i64 {
  let instructions = parse_instructions(&input);
  let mut grid = vec![[0; 1000]; 1000];
  mutate_grid(&mut grid, instructions, false);
  return grid.iter().flatten().sum();
}

fn main() {
  let input = read_input();
  let timer = Instant::now();
  println!(
    "part one {} {}ms",
    part_one(&input),
    timer.elapsed().as_millis()
  );
  println!(
    "part two {} {}ms",
    part_two(&input),
    timer.elapsed().as_millis()
  );
  println!("total {}ms", timer.elapsed().as_millis())
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn test_parse_instructions() {
    assert_eq!(
      parse_instructions("turn on 0,0 through 999,999"),
      vec![Instruction {
        row_start: 0,
        col_start: 0,
        row_end: 1000,
        col_end: 1000,
        action: Action::TurnOn,
      }]
    );
  }

  #[test]
  fn test_part_one() {
    assert_eq!(part_one(&read_input()), 543903);
  }

  #[test]
  fn test_part_two() {
    assert_eq!(part_two(&read_input()), 14687245);
  }
}
