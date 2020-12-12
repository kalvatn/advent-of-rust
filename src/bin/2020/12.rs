#![allow(
  unused_variables,
  unused_imports,
  unused_assignments,
  dead_code,
  deprecated,
  unused_parens
)]

use itertools::Itertools;
use std::time::Instant;

use crate::Direction::{EAST, FORWARD, LEFT, NORTH, RIGHT, SOUTH, WEST};
use common::io;

#[derive(Eq, PartialEq, Debug, Copy, Clone, Hash)]
enum Direction {
  NORTH,
  SOUTH,
  EAST,
  WEST,
  LEFT,
  RIGHT,
  FORWARD,
}

impl From<char> for Direction {
  fn from(c: char) -> Direction {
    match c {
      'N' => Direction::NORTH,
      'S' => Direction::SOUTH,
      'E' => Direction::EAST,
      'W' => Direction::WEST,
      'L' => Direction::LEFT,
      'R' => Direction::RIGHT,
      'F' => Direction::FORWARD,
      _ => panic!("invalid direction"),
    }
  }
}

#[derive(Eq, PartialEq, Hash, Clone, Copy, Debug)]
struct Point {
  facing: Direction,
  x: i16,
  y: i16,
}

impl Point {
  fn mv(self, dir: &Direction) -> Point {
    return match dir {
      Direction::NORTH => Point {
        facing: self.facing,
        x: self.x,
        y: self.y - 1,
      },
      Direction::SOUTH => Point {
        facing: self.facing,
        x: self.x,
        y: self.y + 1,
      },
      Direction::WEST => Point {
        facing: self.facing,
        x: self.x - 1,
        y: self.y,
      },
      Direction::EAST => Point {
        facing: self.facing,
        x: self.x + 1,
        y: self.y,
      },
      Direction::LEFT => match self.facing {
        Direction::NORTH => Point {
          x: self.x,
          y: self.y,
          facing: WEST,
        },
        Direction::SOUTH => Point {
          x: self.x,
          y: self.y,
          facing: EAST,
        },
        Direction::EAST => Point {
          x: self.x,
          y: self.y,
          facing: NORTH,
        },
        Direction::WEST => Point {
          x: self.x,
          y: self.y,
          facing: SOUTH,
        },
        // Direction::LEFT => Point{ x: self.x, y: self.y, facing : WEST},
        // Direction::RIGHT => Point{ x: self.x, y: self.y, facing : WEST},
        // Direction::FORWARD => Point{ x: self.x, y: self.y, facing : WEST},
        Direction::LEFT => unreachable!("impossiburu"),
        Direction::RIGHT => unreachable!("impossiburu"),
        Direction::FORWARD => unreachable!("impossiburu"),
      },
      Direction::RIGHT => match self.facing {
        Direction::NORTH => Point {
          x: self.x,
          y: self.y,
          facing: EAST,
        },
        Direction::SOUTH => Point {
          x: self.x,
          y: self.y,
          facing: WEST,
        },
        Direction::EAST => Point {
          x: self.x,
          y: self.y,
          facing: SOUTH,
        },
        Direction::WEST => Point {
          x: self.x,
          y: self.y,
          facing: NORTH,
        },
        // Direction::LEFT => Point{ x: self.x, y: self.y, facing : WEST},
        // Direction::RIGHT => Point{ x: self.x, y: self.y, facing : WEST},
        // Direction::FORWARD => Point{ x: self.x, y: self.y, facing : WEST},
        Direction::LEFT => unreachable!("impossiburu"),
        Direction::RIGHT => unreachable!("impossiburu"),
        Direction::FORWARD => unreachable!("impossiburu"),
      },
      Direction::FORWARD => match self.facing {
        Direction::NORTH => Point {
          x: self.x,
          y: self.y - 1,
          facing: NORTH,
        },
        Direction::SOUTH => Point {
          x: self.x,
          y: self.y + 1,
          facing: SOUTH,
        },
        Direction::EAST => Point {
          x: self.x + 1,
          y: self.y,
          facing: EAST,
        },
        Direction::WEST => Point {
          x: self.x - 1,
          y: self.y,
          facing: WEST,
        },
        // Direction::LEFT => Point{ x: self.x, y: self.y, facing : LEFT},
        // Direction::RIGHT => Point{ x: self.x, y: self.y, facing : RIGHT},
        // Direction::FORWARD => Point{ x: self.x, y: self.y, facing : FORWARD},
        Direction::LEFT => unreachable!("impossiburu"),
        Direction::RIGHT => unreachable!("impossiburu"),
        Direction::FORWARD => unreachable!("impossiburu"),
      },
    };
  }
}

#[derive(Debug)]
struct Instruction {
  direction: Direction,
  steps: usize,
}

fn read_input() -> String {
  return io::read_input("2020-12");
}

fn parse_input(input: &str) -> Vec<Instruction> {
  return input
    .lines()
    .map(|line| Instruction {
      direction: Direction::from(line.chars().nth(0).unwrap()),
      steps: line.chars().dropping(1).as_str().parse::<usize>().unwrap(),
    })
    .collect();
}

fn get_repeat(instruction: &Instruction) -> usize {
  match instruction.direction {
    LEFT => instruction.steps / 90,
    RIGHT => instruction.steps / 90,
    _ => instruction.steps,
  }
}

fn part_one(input: &str) -> usize {
  let instructions = parse_input(input);
  let p0 = Point {
    x: 0,
    y: 0,
    facing: Direction::EAST,
  };
  let mut pos = p0;
  for instruction in instructions {
    let steps = get_repeat(&instruction);
    for i in 0..steps {
      pos = pos.mv(&instruction.direction);
    }
    // println!("{:>15?}  {} -> {}       ->  {:?}", instruction.direction, instruction.steps, steps, pos);
    println!(
      "{:?} {}  ->  {:?}",
      instruction.direction, instruction.steps, pos
    );
  }

  let horizontal = (pos.x.abs() + p0.x.abs()) as usize;
  let vertical = (pos.y.abs() + p0.y.abs()) as usize;
  println!("{} {}", horizontal, vertical);
  (horizontal + vertical) as usize
}

fn part_two(input: &str) -> usize {
  return 0;
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
  use regex::internal::Inst;

  const TEST_INPUT: &str = "F10
N3
F7
R90
F11
";

  #[test]
  fn test_get_repeat() {
    let instruction = Instruction {
      direction: LEFT,
      steps: 270,
    };
    let mut pos = Point {
      x: 0,
      y: 0,
      facing: NORTH,
    };
    println!("{:?}", pos);
    let repeat = get_repeat(&instruction);
    assert_eq!(repeat, 3);
    for i in 0..repeat {
      pos = pos.mv(&instruction.direction);
      println!("{:?}", pos);
    }
    assert_eq!(
      pos,
      Point {
        x: 0,
        y: 0,
        facing: EAST
      }
    )
  }

  #[test]
  fn test_part_one() {
    assert_eq!(part_one(TEST_INPUT), 25);
    assert_eq!(part_one(&read_input()), 1133);
  }

  #[test]
  fn test_part_two() {
    assert_eq!(part_two(TEST_INPUT), 0);
    // assert_eq!(part_two(&read_input()), 0);
  }
}
