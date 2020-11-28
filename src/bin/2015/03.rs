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

#[derive(Eq, PartialEq, Hash, Clone, Copy)]
struct Point {
  pub x: i32,
  pub y: i32,
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

fn parse_directions(input: &str) -> Vec<Direction> {
  return input
    .chars()
    .map(|c| match c {
      '>' => Direction::RIGHT,
      '<' => Direction::LEFT,
      '^' => Direction::UP,
      'v' => Direction::DOWN,
      _ => panic!("impossiburu"),
    })
    .collect();
}

fn read_puzzle_input() -> String {
  return io::read_input("2015-03").to_string();
}

fn part_one(input: &str) -> usize {
  let directions = parse_directions(input);
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
  let directions = parse_directions(input);
  let initial_state: (u32, Point, Point) = (0, Point { x: 0, y: 0 }, Point { x: 0, y: 0 });
  let points_visited: Vec<(u32, Point, Point)> = directions
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
  let input = read_puzzle_input();
  let total = Instant::now();
  println!(
    "part one {} {}ms",
    part_one(&input),
    total.elapsed().as_millis()
  );
  let p2 = Instant::now();
  println!(
    "part two {} {}ms",
    part_two(&input),
    p2.elapsed().as_millis()
  );
  println!("total {}ms", total.elapsed().as_millis())
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn test_parse_input() {
    assert_eq!(parse_directions("<"), vec![Direction::LEFT]);
    assert_eq!(
      parse_directions("<>^v"),
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
    assert_eq!(part_one(&read_puzzle_input()), 2572);
  }

  #[test]
  fn test_part_two() {
    assert_eq!(part_two("^v"), 3);
    assert_eq!(part_two("^>v<"), 3);
    assert_eq!(part_two("^v^v^v^v^v"), 11);
    assert_eq!(part_two(&read_puzzle_input()), 2631);
  }
}
