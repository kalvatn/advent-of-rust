use std::collections::HashSet;
use std::time::Instant;

use common::io;

fn read_input() -> String {
  return io::read_input("2020-08");
}

fn parse_input(input: &str) -> Vec<(String, i32)> {
  input
    .lines()
    .map(|line| {
      let split = line.split(" ").collect::<Vec<&str>>();
      (split[0].to_string(), split[1].parse::<i32>().unwrap())
    })
    .collect()
}

fn part_one(input: &str) -> i32 {
  let instructions = parse_input(input);
  return run_program(&instructions).0;
}

fn run_program(instructions: &Vec<(String, i32)>) -> (i32, bool) {
  let mut acc = 0i32;
  let mut i = 0usize;
  let mut seen: HashSet<usize> = HashSet::new();
  while i < instructions.len() {
    let (op, v) = &instructions[i as usize];
    if seen.contains(&i) {
      return (acc, false);
    }
    seen.insert(i);
    match op.as_str() {
      "nop" => i += 1,
      "acc" => {
        acc += v;
        i += 1;
      }
      "jmp" => i = (i as i32 + v) as usize,
      _ => unreachable!("impossiburu"),
    }
  }
  (acc, true)
}

fn part_two(input: &str) -> i32 {
  let original_input = parse_input(input);
  let nop_jmp: &Vec<(usize, &(String, i32))> = &original_input
    .iter()
    .enumerate()
    .filter(|(_i, (op, _v))| op == "jmp" || op == "nop")
    .collect();
  for replace in nop_jmp {
    let replace_index = replace.0;
    let (original_op, original_value) = &original_input[replace_index];
    let new_op = match original_op.as_str() {
      "jmp" => "nop",
      "nop" => "jmp",
      _ => unreachable!("not a 'jmp' or 'nop'"),
    };
    let mut instructions = original_input.clone();
    instructions[replace.0] = (new_op.to_string(), *original_value);
    let (acc, terminated_normally) = run_program(&instructions);
    if terminated_normally {
      return acc;
    }
  }
  unreachable!("fail")
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

  const TEST_INPUT: &str = "nop +0
acc +1
jmp +4
acc +3
jmp -3
acc -99
acc +1
jmp -4
acc +6";

  #[test]
  fn test_part_one() {
    assert_eq!(part_one(TEST_INPUT), 5);
    assert_eq!(part_one(&read_input()), 1317);
  }

  #[test]
  fn test_part_two() {
    assert_eq!(part_two(TEST_INPUT), 8);
    assert_eq!(part_two(&read_input()), 1033);
  }
}
