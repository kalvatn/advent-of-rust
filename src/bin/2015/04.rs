use std::time::Instant;

use md5;

use common::io;

fn read_input() -> String {
  return io::read_input("2015-04");
}

fn compute_hash(key: &str, i: &usize) -> String {
  return format!("{:x}", md5::compute(format!("{}{}", key, i)));
}

fn find_number_generating_hash(key: &str, starting_with: &str) -> usize {
  let mut i = 1;
  loop {
    let hash = compute_hash(&*key, &i);
    if hash.starts_with(starting_with) {
      break;
    }
    i += 1;
  }
  return i;
}

fn part_one(key: &str) -> usize {
  return find_number_generating_hash(key, "00000");
}

fn part_two(key: &str) -> usize {
  return find_number_generating_hash(key, "000000");
}

fn main() {
  let key = &read_input();
  let timer = Instant::now();
  println!(
    "part one {} {}ms",
    part_one(key),
    timer.elapsed().as_millis()
  );
  println!(
    "part two {} {}ms",
    part_two(key),
    timer.elapsed().as_millis()
  );
  println!("total {}ms", timer.elapsed().as_millis());
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  #[ignore]
  fn test_part_one() {
    assert_eq!(part_one(&read_input()), 254575);
  }

  #[test]
  #[ignore]
  fn test_part_two() {
    assert_eq!(part_two(&read_input()), 1038736);
  }
}
