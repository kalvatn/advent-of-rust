use aoc_2020_11::{read_input, parse_input, part_one, part_two};

fn main() {
  let input = read_input();
  let parsed = parse_input(&input);
  let p1 = part_one(&parsed);
  let p2 = part_two(&parsed);
  println!("part one {:?}", p1);
  println!("part two {:?}", p2);
}
