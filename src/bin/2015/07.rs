use std::collections::HashMap;
use std::str::FromStr;
use std::string::ParseError;
use std::time::Instant;

use common::io;

use crate::Op::{And, Assign, Lshift, Not, Or, Rshift};

#[derive(Debug, Eq, PartialEq)]
enum Op {
  Assign,
  And,
  Or,
  Not,
  Lshift,
  Rshift,
}

impl FromStr for Op {
  type Err = ParseError;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let mut op = Op::Assign;
    ["NOT", "AND", "OR", "LSHIFT", "RSHIFT"]
      .iter()
      .for_each(|opstr| {
        if s.contains(opstr) {
          op = match opstr {
            &"NOT" => Not,
            &"AND" => And,
            &"OR" => Or,
            &"LSHIFT" => Lshift,
            &"RSHIFT" => Rshift,
            _ => Assign,
          };
        }
      });
    return Ok(op);
  }
}

fn is_numeric(s: &str) -> bool {
  if s.is_empty() {
    return false;
  }
  for c in s.chars() {
    if !c.is_numeric() {
      return false;
    }
  }
  return true;
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
enum Either<T, U> {
  Left(T),
  Right(U),
}

#[derive(Eq, PartialEq, Debug)]
struct Instruction {
  op: Op,
  left: Either<u16, String>,
  right: Either<u16, String>,
  output: String,
}

impl FromStr for Instruction {
  type Err = ParseError;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    fn get_input_output(s: &str) -> (&str, String) {
      let split: Vec<&str> = s.split("->").map(|s| s.trim()).collect();
      return (split[0], split[1].to_string());
    }

    fn parse_arg(s: &str) -> Either<u16, String> {
      return if is_numeric(s) {
        Either::Left(s.parse::<u16>().expect(&*format!("error parsing {}", s)))
      } else {
        if s.is_empty() {
          Either::Right("".to_string())
        } else {
          Either::Right(s.to_string())
        }
      };
    }

    fn get_input_args(op: &Op, s: &str) -> (Either<u16, String>, Either<u16, String>) {
      let args: Vec<String> = s.split_whitespace().map(|s| s.trim().to_string()).collect();
      let a1 = args[0].to_string();
      let a2 = if args.len() > 1 {
        args[1].to_string()
      } else {
        "".to_string()
      };
      let a3 = if args.len() > 2 {
        args[2].to_string()
      } else {
        "".to_string()
      };
      let args: Vec<Either<u16, String>> = vec![a1, a2, a3]
        .iter()
        .map(|a| parse_arg(a.trim()))
        .collect();
      let (a1, a2, a3) = (args[0].to_owned(), args[1].to_owned(), args[2].to_owned());
      return match op {
        Assign => (a1, a2),
        And => (a1, a3),
        Or => (a1, a3),
        Not => (a2, a3),
        Lshift => (a1, a3),
        Rshift => (a1, a3),
      };
    }
    let (input, output) = get_input_output(s);

    let op = Op::from_str(input).unwrap();
    let (left, right) = get_input_args(&op, input);

    return Ok(Instruction {
      op,
      left,
      right,
      output,
    });
  }
}

fn parse_input(input: &str) -> Vec<Instruction> {
  return input
    .lines()
    .map(|line| Instruction::from_str(&line).unwrap())
    .collect();
}

fn get_wire(
  instructions: &HashMap<String, Instruction>,
  results: &mut HashMap<String, u16>,
  wire: &Either<u16, String>,
) -> u16 {
  match wire {
    Either::Right(key) => {
      if key.is_empty() {
        return 0;
      }
      if results.contains_key(key) {
        *results.get(key).unwrap()
      } else {
        let instruction = instructions.get(key).unwrap();
        let left = get_wire(&instructions, results, &instruction.left);
        let right = get_wire(&instructions, results, &instruction.right);
        let result = match instruction.op {
          Assign => left,
          And => (left & right),
          Or => (left | right),
          Not => (!left),
          Lshift => (left << right),
          Rshift => (left >> right),
        };
        results.insert(key.to_string(), result);
        result
      }
    }
    Either::Left(r) => *r,
  }
}

fn part_one(input: &str) -> u16 {
  let mut instructions: HashMap<String, Instruction> = HashMap::new();
  let mut results: HashMap<String, u16> = HashMap::new();
  for x in parse_input(input) {
    instructions.insert(x.output.to_string(), x);
  }
  return get_wire(&instructions, &mut results, &Either::Right("a".to_string()));
}

fn part_two(input: &str) -> u16 {
  let part_one = part_one(input);
  let mut instructions: HashMap<String, Instruction> = HashMap::new();
  let mut results: HashMap<String, u16> = HashMap::new();
  for x in parse_input(input) {
    instructions.insert(x.output.to_string(), x);
  }
  instructions.insert(
    "b".to_string(),
    Instruction {
      op: Assign,
      left: Either::Left(part_one),
      right: Either::Left(0),
      output: "b".to_string(),
    },
  );
  return get_wire(&instructions, &mut results, &Either::Right("a".to_string()));
}

fn read_input() -> String {
  return io::read_input("2015-07");
}

fn main() {
  let input = read_input();
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
  fn test_parse_op() {
    assert_eq!(Op::from_str("123").unwrap(), Op::Assign);
  }

  #[test]
  fn test_parse_input() {
    assert_eq!(
      parse_input("456 -> y"),
      vec![Instruction {
        op: Op::Assign,
        left: Either::Left(456),
        right: Either::Right("".to_string()),
        output: "y".to_string(),
      }]
    );
    assert_eq!(
      parse_input("NOT 456 -> y"),
      vec![Instruction {
        op: Op::Not,
        left: Either::Left(456),
        right: Either::Right("".to_string()),
        output: "y".to_string(),
      }]
    );

    assert_eq!(
      parse_input(
        "123 -> a
456 -> y
a AND y -> d
a OR y -> e
a LSHIFT 2 -> f
y RSHIFT 2 -> g
NOT a -> h
NOT y -> i"
      ),
      vec![
        Instruction {
          op: Assign,
          left: Either::Left(123),
          right: Either::Right("".to_string()),
          output: "a".to_string()
        },
        Instruction {
          op: Assign,
          left: Either::Left(456),
          right: Either::Right("".to_string()),
          output: "y".to_string()
        },
        Instruction {
          op: And,
          left: Either::Right("a".to_string()),
          right: Either::Right("y".to_string()),
          output: "d".to_string()
        },
        Instruction {
          op: Or,
          left: Either::Right("a".to_string()),
          right: Either::Right("y".to_string()),
          output: "e".to_string()
        },
        Instruction {
          op: Lshift,
          left: Either::Right("a".to_string()),
          right: Either::Left(2),
          output: "f".to_string()
        },
        Instruction {
          op: Rshift,
          left: Either::Right("y".to_string()),
          right: Either::Left(2),
          output: "g".to_string()
        },
        Instruction {
          op: Not,
          left: Either::Right("a".to_string()),
          right: Either::Right("".to_string()),
          output: "h".to_string()
        },
        Instruction {
          op: Not,
          left: Either::Right("y".to_string()),
          right: Either::Right("".to_string()),
          output: "i".to_string()
        },
      ]
    );
    assert_eq!(parse_input(&read_input()).len(), 339);
  }

  #[test]
  fn test_part_one() {
    assert_eq!(
      part_one(
        "123 -> a
456 -> y
a AND y -> d
a OR y -> e
a LSHIFT 2 -> f
y RSHIFT 2 -> g
NOT a -> h
NOT y -> i"
      ),
      123
    );
    assert_eq!(part_one(&read_input()), 16076);
  }

  #[test]
  fn test_part_two() {
    assert_eq!(part_two(&read_input()), 2797);
  }
}
