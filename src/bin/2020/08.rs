use std::collections::HashSet;
use std::time::Instant;

use common::io;

#[derive(PartialEq, Eq, Clone)]
enum Op {
  Nop,
  Jmp,
  Acc,
}

#[derive(Clone)]
struct Instruction {
  op: Op,
  val: i16,
}

fn read_input() -> String {
  return io::read_input("2020-08");
}

fn parse_input(input: &str) -> Vec<Instruction> {
  input
    .lines()
    .map(|line| {
      let split = line.split(" ").collect::<Vec<&str>>();
      Instruction {
        op: match split[0] {
          "jmp" => Op::Jmp,
          "acc" => Op::Acc,
          "nop" => Op::Nop,
          _ => unreachable!("impossiburu"),
        },
        val: split[1].parse::<i16>().unwrap(),
      }
    })
    .collect()
}

fn run_program(instructions: &Vec<Instruction>, swap: Option<usize>) -> (i16, bool) {
  let mut acc = 0i16;
  let mut i = 0usize;
  let mut seen: HashSet<usize> = HashSet::new();
  let len = instructions.len();
  while i < len {
    let instruction = &instructions[i];
    if seen.contains(&i) {
      return (acc, false);
    }
    seen.insert(i);
    let op = if swap.is_some() && swap.unwrap() == i {
      match instruction.op {
        Op::Nop => &Op::Jmp,
        Op::Jmp => &Op::Nop,
        _ => unreachable!(),
      }
    } else {
      &instruction.op
    };
    match op {
      Op::Nop => i += 1,
      Op::Jmp => i = (i as i16 + instruction.val) as usize,
      Op::Acc => {
        acc += instruction.val;
        i += 1;
      }
    }
  }
  (acc, true)
}

fn part_one(instructions: &Vec<Instruction>) -> i16 {
  return run_program(&instructions, None).0;
}

fn part_two(original_input: &Vec<Instruction>) -> i16 {
  for (swap_index, ins) in original_input.iter().enumerate() {
    if ins.op != Op::Acc {
      let (acc, terminated_normally) = run_program(&original_input, Some(swap_index));
      if terminated_normally {
        return acc;
      }
    }
  }
  unreachable!("fail")
}

fn main() {
  let time = Instant::now();
  let input = parse_input(&read_input());
  let parse_time = time.elapsed();

  let time = Instant::now();
  let p1 = part_one(&input);
  let p1_time = time.elapsed();

  let time = Instant::now();
  let p2 = part_two(&input);
  let p2_time = time.elapsed();
  println!("parse {:?}", parse_time);
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
    assert_eq!(part_one(&parse_input(TEST_INPUT)), 5);
    assert_eq!(part_one(&parse_input(&read_input())), 1317);
  }

  #[test]
  fn test_part_two() {
    assert_eq!(part_two(&parse_input(TEST_INPUT)), 8);
    assert_eq!(part_two(&parse_input(&read_input())), 1033);
  }
}
