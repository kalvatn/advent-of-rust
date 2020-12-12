use std::time::Instant;

use itertools::Itertools;

use common::io;

use crate::Direction::{EAST, FORWARD, LEFT, NORTH, RIGHT, SOUTH, WEST};

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
      'N' => NORTH,
      'S' => SOUTH,
      'E' => EAST,
      'W' => WEST,
      'L' => LEFT,
      'R' => RIGHT,
      'F' => FORWARD,
      _ => unreachable!("invalid direction"),
    }
  }
}

#[derive(Eq, PartialEq, Hash, Clone, Copy, Debug)]
struct Ship {
  facing: Direction,
  pos: Point,
}

impl Ship {
  fn mv(&mut self, dir: &Direction, steps: usize) -> Ship {
    for _i in 0..steps {
      match dir {
        LEFT => match &self.facing {
          NORTH => self.facing = WEST,
          SOUTH => self.facing = EAST,
          EAST => self.facing = NORTH,
          WEST => self.facing = SOUTH,
          _ => unreachable!(),
        },
        RIGHT => match self.facing {
          NORTH => self.facing = EAST,
          SOUTH => self.facing = WEST,
          EAST => self.facing = SOUTH,
          WEST => self.facing = NORTH,
          _ => unreachable!(),
        },
        FORWARD => {
          self.pos = self.pos.mv(&self.facing);
        }
        _ => {
          self.pos = self.pos.mv(dir);
        }
      }
    }
    *self
  }
  fn mv_towards_waypoint(&mut self, waypoint: &Point, steps: usize) -> Ship {
    for _i in 0..steps {
      self.pos.x += waypoint.x;
      self.pos.y += waypoint.y;
    }
    *self
  }
}

#[derive(Eq, PartialEq, Hash, Clone, Copy, Debug)]
struct Point {
  x: isize,
  y: isize,
}

impl Point {
  fn distance(&self, other: Point) -> usize {
    (self.x.abs() + other.x.abs() + self.y.abs() + other.y.abs()) as usize
  }
  fn mv(&mut self, dir: &Direction) -> Point {
    match dir {
      NORTH => self.y -= 1,
      SOUTH => self.y += 1,
      WEST => self.x -= 1,
      EAST => self.x += 1,
      _ => unreachable!(),
    };
    return *self;
  }
  fn rotate(&mut self, dir: &Direction, degrees: isize) -> Point {
    let angle = if dir == &LEFT { degrees * -1 } else { degrees };
    let current = *self;
    let quarter_rotations = angle / 90;
    match quarter_rotations.rem_euclid(4) {
      1 => {
        // LEFT 270 | RIGHT 90 , flip and invert horizontal
        self.x = -current.y;
        self.y = current.x;
      }
      2 => {
        // 180 noflip, invert both
        self.x = -current.x;
        self.y = -current.y;
      }
      3 => {
        // RIGHT 270 | LEFT 90, flip and invert vertical
        self.x = current.y;
        self.y = -current.x;
      }
      _ => {
        // 360
      }
    };
    return *self;
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

fn angle_to_steps(instruction: &Instruction) -> usize {
  match instruction.direction {
    LEFT | RIGHT => instruction.steps / 90,
    _ => instruction.steps,
  }
}

fn part_one(input: &str) -> usize {
  let instructions = parse_input(input);
  let original_pos = Point { x: 0, y: 0 };
  let mut ship = Ship {
    pos: original_pos,
    facing: EAST,
  };
  for instruction in instructions {
    let steps = angle_to_steps(&instruction);
    ship.mv(&instruction.direction, steps);
  }
  ship.pos.distance(original_pos)
}

fn part_two(input: &str) -> usize {
  let instructions = parse_input(input);
  let original_pos = Point { x: 0, y: 0 };
  let mut ship = Ship {
    pos: original_pos,
    facing: EAST,
  };
  let mut waypoint = Point { x: 10, y: -1 };
  for instruction in instructions {
    let steps = angle_to_steps(&instruction);
    match instruction.direction {
      NORTH | SOUTH | EAST | WEST => {
        for _i in 0..steps {
          waypoint.mv(&instruction.direction);
        }
      }
      LEFT | RIGHT => {
        waypoint.rotate(&instruction.direction, instruction.steps as isize);
      }
      FORWARD => {
        ship.mv_towards_waypoint(&waypoint, steps);
      }
    }
  }

  ship.pos.distance(original_pos)
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

  const TEST_INPUT: &str = "F10
N3
F7
R90
F11
";

  #[test]
  fn test_angle_to_steps() {
    let instruction = Instruction {
      direction: LEFT,
      steps: 270,
    };
    let mut pos = Ship {
      pos: Point { x: 0, y: 0 },
      facing: EAST,
    };
    println!("{:?}", pos);
    let repeat = angle_to_steps(&instruction);
    assert_eq!(repeat, 3);
    pos = pos.mv(&instruction.direction, repeat);
    assert_eq!(
      pos,
      Ship {
        pos: Point { x: 0, y: 0 },
        facing: SOUTH,
      }
    )
  }

  #[test]
  fn test_point_rotation() {
    let mut p = Point { x: 10, y: -4 };
    assert_eq!(p.rotate(&RIGHT, 90), Point { x: 4, y: 10 });

    let mut p = Point { x: 10, y: -4 };
    assert_eq!(p.rotate(&LEFT, 270), Point { x: 4, y: 10 });

    let mut p = Point { x: 10, y: -4 };
    assert_eq!(p.rotate(&LEFT, 360), Point { x: 10, y: -4 });

    let mut p = Point { x: 10, y: -4 };
    assert_eq!(p.rotate(&LEFT, 90), Point { x: -4, y: -10 });

    let mut p = Point { x: 10, y: -4 };
    assert_eq!(p.rotate(&LEFT, 180), Point { x: -10, y: 4 });
  }

  #[test]
  fn test_part_one() {
    assert_eq!(part_one(TEST_INPUT), 25);
    assert_eq!(part_one(&read_input()), 1133);
  }

  #[test]
  fn test_part_two() {
    assert_eq!(part_two(TEST_INPUT), 286);
    assert_eq!(part_two(&read_input()), 61053);
  }
}
