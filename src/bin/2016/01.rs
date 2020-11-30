use std::time::Instant;
use std::collections::HashSet;

use common::io;

fn read_input() -> String {
  return io::read_input("2016-01");
}

fn parse_input(input: &str) -> Vec<(char,i32)> {
  input.split(",")
    .map(|s| s.trim())
    .map(|s| {
    (s.chars().nth(0).unwrap(), s[1..].parse::<i32>().unwrap())
  }).collect()
}

#[allow(unused_variables)]
fn part_one(input: &str) -> i32 {
  let mut facing = 'N';
  let mut pos:(i32,i32) = (0,0);
  parse_input(input).iter().for_each(|(turn,steps)| {
    facing = match turn {
      'R' => {
        match facing {
          'N' => 'E',
          'S' => 'W',
          'E' => 'S',
          'W' => 'N',
          _ => panic!("impossiburu")
        }
      },
      'L' => {
        match facing {
          'N' => 'W',
          'S' => 'E',
          'E' => 'N',
          'W' => 'S',
          _ => panic!("impossiburu")
        }
      },
      _ => panic!("impossiburu")
    };

    match facing {
      'N' => pos = (pos.0, pos.1 + steps),
      'E' => pos = (pos.0 + steps, pos.1),
      'S' => pos = (pos.0, pos.1 -steps),
      'W' => pos = (pos.0 -steps, pos.1),
      _ => panic!("impossiburu")
    };
  });
  return pos.0.abs() + pos.1.abs();
}

#[allow(unused_variables)]
fn part_two(input: &str) -> i32 {
  let mut facing = 'N';
  let mut pos:(i32,i32) = (0,0);
  let mut visited:HashSet<(i32,i32)> = HashSet::new();
  visited.insert(pos);
  for (turn,steps) in parse_input(input).iter() {
    facing = match turn {
      'R' => {
        match facing {
          'N' => 'E',
          'S' => 'W',
          'E' => 'S',
          'W' => 'N',
          _ => panic!("impossiburu")
        }
      },
      'L' => {
        match facing {
          'N' => 'W',
          'S' => 'E',
          'E' => 'N',
          'W' => 'S',
          _ => panic!("impossiburu")
        }
      },
      _ => panic!("impossiburu")
    };

    for i in 1..steps+1 {
      match facing {
        'N' => pos = (pos.0, pos.1 + 1),
        'E' => pos = (pos.0 + 1, pos.1),
        'S' => pos = (pos.0, pos.1 -1),
        'W' => pos = (pos.0 -1, pos.1),
        _ => panic!("impossiburu")
      };
      if visited.contains(&pos) {
        return pos.0.abs() + pos.1.abs();
      }
      visited.insert(pos);
    }
  }
  return pos.0.abs() + pos.1.abs();
}

fn main() {
  let input = read_input();
  println!("{}", input);

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

  #[test]
  fn test_part_one() {
    assert_eq!(part_one(&read_input()), 271);
  }

  #[test]
  fn test_part_two() {
    assert_eq!(part_two(&read_input()), 153);
  }
}
