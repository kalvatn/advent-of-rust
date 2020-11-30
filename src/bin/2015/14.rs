use std::time::Instant;

use lazy_static::lazy_static;
use regex::Regex;

use common::io;

lazy_static! {
  static ref RE: Regex = Regex::new(
    r"^(\w+) can fly (\d+) km/s for (\d+) seconds, but then must rest for (\d+) seconds.$"
  )
  .unwrap();
}

#[derive(Debug)]
struct Reindeer {
  name: String,
  speed: usize,
  fly_secs: usize,
  rest_secs: usize,
  state: ReindeerState,
}

#[derive(Debug)]
struct ReindeerState {
  distance: usize,
  flying_for: usize,
  resting_for: usize,
}

impl Reindeer {
  pub fn fly(&mut self) {
    if self.state.flying_for < self.fly_secs {
      self.state.flying_for += 1;
      self.state.distance += self.speed;
    } else {
      if self.state.resting_for < self.rest_secs {
        self.state.resting_for += 1
      }
      if self.state.resting_for == self.rest_secs {
        self.state.flying_for = 0;
        self.state.resting_for = 0;
      }
    }
  }
}

fn read_input() -> String {
  return io::read_input("2015-14");
}

fn parse_input(input: &str) -> Vec<Reindeer> {
  input
    .lines()
    .map(|line| {
      RE.captures(line)
        .map(|c| Reindeer {
          name: c[1].to_string(),
          speed: c[2].parse::<usize>().unwrap(),
          fly_secs: c[3].parse::<usize>().unwrap(),
          rest_secs: c[4].parse::<usize>().unwrap(),
          state: ReindeerState {
            distance: 0,
            resting_for: 0,
            flying_for: 0,
          },
        })
        .unwrap()
    })
    .collect()
}

#[allow(unused_variables)]
fn part_one(input: &str, secs: usize) -> usize {
  let mut reindeers = parse_input(input);
  for i in 1..secs + 1 {
    reindeers.iter_mut().for_each(|r| {
      r.fly();
    });
  }
  reindeers.iter().map(|r| r.state.distance).max().unwrap()
}

#[allow(unused_variables)]
fn part_two(input: &str) -> usize {
  return 0;
}

fn main() {
  let input = read_input();

  let p1_timer = Instant::now();
  println!(
    "part one {} {}ms",
    part_one(&input, 2503),
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

  const TEST_INPUT: &str =
    "Comet can fly 14 km/s for 10 seconds, but then must rest for 127 seconds.
Dancer can fly 16 km/s for 11 seconds, but then must rest for 162 seconds.";

  #[test]
  fn test_part_one() {
    assert_eq!(part_one(TEST_INPUT, 10), 160);
    assert_eq!(part_one(TEST_INPUT, 11), 176);
    assert_eq!(part_one(TEST_INPUT, 12), 176);
    assert_eq!(part_one(TEST_INPUT, 138), 176);
    assert_eq!(part_one(TEST_INPUT, 139), 176);
    assert_eq!(part_one(TEST_INPUT, 1000), 1120);
    assert_eq!(part_one(&read_input(), 2503), 2640);
  }

  #[test]
  fn test_part_two() {
    assert_eq!(part_two(&read_input()), 0);
  }
}
