use std::time::Instant;

use common::io;

fn read_input() -> String {
  return io::read_input("2020-03");
}

fn parse_input(input: &str) -> Vec<Vec<bool>> {
  let max_y = input.lines().count();
  let max_x = input.lines().nth(0).unwrap().len();
  let mut map = vec![vec![false; max_x]; max_y];
  for (y, line) in input.lines().enumerate() {
    for (x, col) in line.chars().enumerate() {
      let tree = match col {
        '.' => false,
        '#' => true,
        _ => panic!("impossiburu"),
      };
      map[y][x] = tree;
    }
  }
  return map;
}

fn trees_encountered_for_slope(map: &Vec<Vec<bool>>, slope: &(usize, usize)) -> u32 {
  let (max_y, max_x) = (map.len(), map[0].len());
  let mut pos = (0, 0);
  let mut trees = 0;
  while pos.0 < max_y {
    if map[pos.0][pos.1] {
      trees += 1;
    }
    let y = pos.0 + slope.0;
    let x = (pos.1 + slope.1) % max_x;
    pos = (y, x);
  }
  return trees;
}
fn trees_encountered_for_slopes(map: &Vec<Vec<bool>>, slopes: &Vec<(usize, usize)>) -> u32 {
  slopes
    .iter()
    .map(|slope| trees_encountered_for_slope(map, slope))
    .fold(1, |acc, trees| acc * trees)
}

fn part_one(input: &str) -> u32 {
  let map = parse_input(input);
  return trees_encountered_for_slope(&map, &(1, 3));
}

fn part_two(input: &str) -> u32 {
  let map = parse_input(input);
  trees_encountered_for_slopes(&map, &vec![(1, 1), (1, 3), (1, 5), (1, 7), (2, 1)])
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

  const TEST_INPUT: &str = "..##.......
#...#...#..
.#....#..#.
..#.#...#.#
.#...##..#.
..#.##.....
.#.#.#....#
.#........#
#.##...#...
#...##....#
.#..#...#.#";

  #[test]
  fn test_parse_input() {
    let map = parse_input(TEST_INPUT);
    assert!(map[0][3]);
    assert!(map[0][3]);
    assert!(map[1][0]);
    assert!(map[1][4]);
    assert!(map[3][10]);
  }

  #[test]
  fn test_part_one() {
    assert_eq!(part_one(TEST_INPUT), 7);
    assert_eq!(part_one(&read_input()), 232);
  }

  #[test]
  fn test_part_two() {
    assert_eq!(part_two(TEST_INPUT), 336);
    assert_eq!(part_two(&read_input()), 3952291680);
  }
}
