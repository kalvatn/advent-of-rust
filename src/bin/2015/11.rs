use std::char;
use std::time::Instant;

use common::io;

fn contains_iol(string: &str) -> bool {
  for c in "iol".chars() {
    if string.contains(c) {
      return true;
    }
  }
  return false;
}

fn is_xx(xx: &str) -> bool {
  return xx.len() == 2 && xx.chars().nth(0) == xx.chars().nth(1);
}

fn is_abc(xyz: &str) -> bool {
  if xyz.len() != 3 {
    return false;
  }
  let chars: Vec<char> = xyz.chars().collect();
  let c1 = chars[0] as i32;
  let c2 = chars[1] as i32;
  let c3 = chars[2] as i32;
  return c3 - c2 == 1 && c2 - c1 == 1;
}

fn read_input() -> String {
  return io::read_input("2015-11");
}

fn parse_input(input: &str) -> String {
  input.lines().nth(0).unwrap().to_string()
}

fn inc_char(c: char) -> char {
  if c == 'z' {
    'a'
  } else {
    char::from_u32(c as u32 + 1).unwrap()
  }
}

fn increment_str(s: &str) -> String {
  let mut chars: Vec<char> = s.chars().collect();
  for i in (0..chars.len()).rev() {
    let c = chars[i];
    let cc = inc_char(c);
    chars[i] = cc;
    if cc != 'a' {
      break;
    }
  }
  let s: String = chars.into_iter().collect();
  return s;
}

fn has_abc(s: &str) -> bool {
  for w in s.chars().collect::<Vec<char>>().windows(3) {
    let s: String = w.into_iter().collect();
    if is_abc(&s) {
      return true;
    }
  }
  return false;
}

fn has_xx(s: &str) -> bool {
  let mut first: (usize, String) = (0, String::new());
  for w in s.chars().collect::<Vec<char>>().windows(2).enumerate() {
    let i: usize = w.0;
    let s: String = w.1.into_iter().collect();
    if is_xx(&s) {
      if first.1 == "" {
        first.1 = s;
      } else if i - first.0 > 1 && s != first.1 {
        return true;
      }
    }
  }
  return false;
}

fn next_password(current: &str) -> String {
  let mut new = current.to_owned();
  loop {
    new = increment_str(&new);
    if !contains_iol(&new) && has_xx(&new) && has_abc(&new) {
      return new.to_owned();
    }
  }
}

fn part_one(input: &str) -> String {
  next_password(input)
}

fn part_two(input: &str) -> String {
  next_password(&next_password(input))
}

fn main() {
  let input = &parse_input(&read_input());
  let timer = Instant::now();
  println!(
    "part one {} {}ms",
    part_one(input),
    timer.elapsed().as_millis()
  );
  println!(
    "part two {} {}ms",
    part_two(input),
    timer.elapsed().as_millis()
  );
  println!("total {}ms", timer.elapsed().as_millis());
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn test_parse_input() {
    assert_eq!(parse_input(&read_input()), "hepxcrrq");
  }

  #[test]
  fn test_inc_char() {
    assert_eq!(inc_char('a'), 'b');
    assert_eq!(inc_char('z'), 'a');
  }

  #[test]
  fn test_increment() {
    assert_eq!(increment_str("a"), "b");
    assert_eq!(increment_str("ab"), "ac");
    assert_eq!(increment_str("xx"), "xy");
    assert_eq!(increment_str("xy"), "xz");
    assert_eq!(increment_str("xz"), "ya");
    assert_eq!(increment_str("hepxcrrq"), "hepxcrrr");
  }

  #[test]
  fn test_is_abc() {
    assert!(is_abc("abc"));
    assert!(is_abc("xyz"));

    assert!(!is_abc("abd"));
    assert!(!is_abc("abe"));
  }

  #[test]
  fn test_has_abc() {
    assert!(has_abc("abcdef"));
    assert!(has_abc("abdxyz"));

    assert!(!has_abc("abd"));
    assert!(!has_abc("abe"));
  }

  #[test]
  fn test_has_xx() {
    assert!(!has_xx("xx"));
    assert!(!has_xx("aa"));
    assert!(!has_xx("abbcegjk"));

    assert!(has_xx("abbceffg"));
    assert!(has_xx("aabb"));
    assert!(has_xx("aabbcc"));
    assert!(has_xx("abbbcc"));
  }

  #[test]
  fn test_part_one() {
    assert_eq!(part_one("abcdefgh"), "abcdffaa");
    assert_eq!(part_one("ghijklmn"), "ghjaabcc");
    assert_eq!(part_one(&parse_input(&read_input())), "hepxxyzz");
  }

  #[test]
  fn test_part_two() {
    assert_eq!(part_two(&parse_input(&read_input())), "heqaabcc");
  }
}
