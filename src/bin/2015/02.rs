use common::io;

fn calc_paper(l: u32, w: u32, h: u32) -> u32 {
  let sides = vec![l * w, w * h, h * l];
  let min = sides.iter().min();
  (2 * sides[0] + 2 * sides[1] + 2 * sides[2]) + min.unwrap()
}

fn calc_ribbon(l: u32, w: u32, h: u32) -> u32 {
  let mut sides: Vec<u32> = vec![l, w, h];
  sides.sort();
  return (sides[0] * 2) + (sides[1] * 2) + (l * w * h);
}

fn parse_line(line: &str) -> Vec<u32> {
  return line.split("x").map(|x| x.parse::<u32>().unwrap()).collect();
}

fn part_one(lines: Vec<&str>) -> u32 {
  let sum_paper = lines
    .iter()
    .map(|line| parse_line(line))
    .map(|dim| calc_paper(dim[0], dim[1], dim[2]))
    .sum();
  return sum_paper;
}

fn part_two(lines: Vec<&str>) -> u32 {
  let sum_paper = lines
    .iter()
    .map(|line| parse_line(line))
    .map(|dim| calc_ribbon(dim[0], dim[1], dim[2]))
    .sum();
  return sum_paper;
}

fn main() {
  let input = io::read_input("2015-02");
  let lines: Vec<&str> = input.lines().collect();
  let sum_paper = part_one(lines.clone());
  let sum_ribbon = part_two(lines.clone());
  println!("part one : {}", sum_paper);
  println!("part two : {}", sum_ribbon);
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn test_parse_line() {
    assert_eq!(parse_line("2x3x4"), [2, 3, 4]);
    assert_eq!(parse_line("1x1x10"), [1, 1, 10]);
  }

  #[test]
  fn test_calc_paper() {
    assert_eq!(calc_paper(2, 3, 4), 52 + 6);
    assert_eq!(calc_paper(1, 1, 10), 42 + 1);
  }

  #[test]
  fn test_calc_ribbon() {
    assert_eq!(calc_ribbon(2, 3, 4), 10 + 24);
    assert_eq!(calc_ribbon(1, 1, 10), 4 + 10);
  }

  #[test]
  fn test_part_one() {
    assert_eq!(part_one(Vec::from(["2x3x4", "1x1x10"])), 58 + 43);
  }

  #[test]
  fn test_part_two() {
    assert_eq!(part_two(Vec::from(["2x3x4", "1x1x10"])), 34 + 14);
  }
}
