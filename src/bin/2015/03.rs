use std::collections::HashSet;
use std::time::Instant;

use common::io;

#[derive(Eq, PartialEq, Debug)]
enum Direction {
  LEFT,
  RIGHT,
  UP,
  DOWN,
}

impl From<char> for Direction {
  fn from(c: char) -> Direction {
    match c {
      '<' => Direction::LEFT,
      '>' => Direction::RIGHT,
      '^' => Direction::UP,
      'v' => Direction::DOWN,
      _ => panic!("invalid direction"),
    }
  }
}

#[derive(Eq, PartialEq, Hash, Clone, Copy)]
struct Point {
  x: i8,
  y: i8,
}

impl Point {
  fn mv(self, dir: &Direction) -> Point {
    return match dir {
      Direction::UP => Point {
        x: self.x,
        y: self.y - 1,
      },
      Direction::DOWN => Point {
        x: self.x,
        y: self.y + 1,
      },
      Direction::LEFT => Point {
        x: self.x - 1,
        y: self.y,
      },
      Direction::RIGHT => Point {
        x: self.x + 1,
        y: self.y,
      },
    };
  }
}

fn parse_input(input: &str) -> Vec<Direction> {
  return input.chars().map(|c| Direction::from(c)).collect();
}

fn read_input() -> String {
  return io::read_input("2015-03").to_string();
}

fn part_one(input: &str) -> usize {
  let directions = parse_input(input);
  let unique: HashSet<_> = directions
    .iter()
    .scan(Point { x: 0, y: 0 }, |acc, dir| {
      *acc = acc.mv(&dir);
      Some(*acc)
    })
    .collect();
  return unique.len();
}

fn part_two(input: &str) -> usize {
  let directions = parse_input(input);
  let initial_state: (u8, Point, Point) = (0, Point { x: 0, y: 0 }, Point { x: 0, y: 0 });
  let points_visited: Vec<(u8, Point, Point)> = directions
    .iter()
    .scan(initial_state, |acc, dir| {
      let (step, santa, robo) = *acc;
      if step % 2 == 0 {
        *acc = (step + 1, santa.mv(&dir), robo);
      } else {
        *acc = (step + 1, santa, robo.mv(&dir));
      }
      Some(*acc)
    })
    .collect();
  let unique: HashSet<Point> = points_visited.iter().flat_map(|t| vec![t.1, t.2]).collect();
  return unique.len();
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
  fn test_parse_input() {
    assert_eq!(parse_input("<"), vec![Direction::LEFT]);
    assert_eq!(
      parse_input("<>^v"),
      vec![
        Direction::LEFT,
        Direction::RIGHT,
        Direction::UP,
        Direction::DOWN
      ]
    );
  }

  #[test]
  fn test_part_one() {
    assert_eq!(part_one(">"), 1);
    assert_eq!(part_one("^>v<"), 4);
    assert_eq!(part_one("^v^v^v^v^v"), 2);
    assert_eq!(part_one(&read_input()), 2572);
  }

  #[test]
  fn test_part_two() {
    assert_eq!(part_two("^v"), 3);
    assert_eq!(part_two("^>v<"), 3);
    assert_eq!(part_two("^v^v^v^v^v"), 11);
    assert_eq!(part_two(&read_input()), 2631);
  }
}
