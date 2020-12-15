use std::{env, io};

use aoc_2015_01::{parse_input, part_one, part_two};
use aoc_common::io::{read_input, read_stdin};

fn main() {
  let stdin_flag = env::args().nth(1);
  let input = if stdin_flag.is_some() && stdin_flag.unwrap().eq("-") {
    read_stdin()
  } else {
    read_input("2015-01")
  };

  let parsed = parse_input(&input);
  let p1 = part_one(&parsed);
  let p2 = part_two(&parsed);
  println!("part one {:?}", p1);
  println!("part two {:?}", p2);
}
