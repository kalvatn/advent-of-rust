use aoc_common::io;

pub fn read_input() -> String {
  return io::read_input("2015-01");
}

pub fn parse_input(input: &str) -> Vec<i16> {
  return input
    .chars()
    .map(|c| match c {
      '(' => 1,
      ')' => -1,
      _ => 0,
    })
    .collect();
}

pub fn part_one(input: &Vec<i16>) -> i16 {
  input.into_iter().sum()
}

pub fn part_two(input: &Vec<i16>) -> usize {
  // let mut floor = 0i32;
  // for (i, c) in input.iter().enumerate() {
  //     floor += c;
  //     if floor == -1 {
  //       return i + 1;
  //     }
  // }
  // return 0;
  return input
    .iter()
    .scan(0, |acc, &n| {
      *acc = *acc + n;
      Some(*acc)
    })
    .take_while(|x| x != &-1)
    .count()
    + 1;
}


#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn test_directions_to_int() {
    assert_eq!(parse_input("(())"), vec![1, 1, -1, -1]);
  }

  #[test]
  fn test_part_one() {
    assert_eq!(part_one(&parse_input("(())")), 0);
    assert_eq!(part_one(&parse_input(&read_input())), 138);
  }

  #[test]
  fn test_part_two() {
    assert_eq!(part_two(&parse_input(")")), 1);
    assert_eq!(part_two(&parse_input("()())")), 5);
    assert_eq!(part_two(&parse_input(&read_input())), 1771);
  }
}
