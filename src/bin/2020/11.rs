#![allow(
  unused_variables,
  unused_imports,
  unused_assignments,
  dead_code,
  deprecated,
  unused_parens
)]

use std::collections::{HashMap, HashSet};
use std::time::Instant;

use itertools::Itertools;
use lazy_static::lazy_static;
use regex::Regex;

use common::io;
use primes::PrimeSet;
use std::borrow::Borrow;

#[derive(Eq, PartialEq, Hash, Clone, Copy, Debug)]
struct Point {
  x: i8,
  y: i8,
}

impl Point {
  fn add(&self, other: Point) -> Point {
    Point {
      x: self.x + other.x,
      y: self.y + other.y,
    }
  }
  fn adj8(&self) -> Vec<Point> {
    lazy_static! {
      static ref NEIGHBOURS:Vec<(i8, i8)> = vec![
        (-1, -1), // NW
        (-1, 0), // N
        (-1, 1), // NE
        (0, -1), // W
        (0, 1), // E
        (1, -1), // SW
        (1, 0), // S
        (1, 1), // SE
       ];
    }
    NEIGHBOURS
      .borrow()
      .iter()
      .map(|n| self.add(Point { x: n.1, y: n.0 }))
      .into_iter()
      .collect()
  }
}

fn read_input() -> String {
  return io::read_input("2020-11");
}

fn parse_input(input: &str) -> Vec<Vec<char>> {
  return input.lines().map(|line| line.chars().collect()).collect();
}

fn mutate(layout: &Vec<Vec<char>>) -> Vec<Vec<char>> {
  let height = layout.len() - 1;
  let width = layout[0].len() - 1;
  let mut new_layout = layout.clone();
  for y in 0..=height {
    for x in 0..=width {
      let p = Point {
        y: y as i8,
        x: x as i8,
      };
      let adj = p
        .adj8()
        .iter()
        .cloned()
        .filter(|p| p.x >= 0 && p.x <= width as i8 && p.y >= 0 && p.y <= height as i8)
        .collect::<Vec<Point>>();
      let new_state = match layout[y][x] {
        'L' => {
          let no_adj_occupied = &adj
            .iter()
            .filter(|n| layout[n.y as usize][n.x as usize] == '#')
            .count()
            == &0;
          if no_adj_occupied {
            '#'
          } else {
            'L'
          }
        }
        '#' => {
          let four_occupied = &adj
            .iter()
            .filter(|n| layout[n.y as usize][n.x as usize] == '#')
            .count()
            >= &4;
          if four_occupied {
            'L'
          } else {
            '#'
          }
        }
        '.' => '.',
        _ => unreachable!("impossiburu"),
      };
      new_layout[y][x] = new_state;
    }
  }
  new_layout
}

fn print_layout(layout: &Vec<Vec<char>>) {
  for y in 0..layout.len() {
    for x in 0..layout[y].len() {
      print!("{}", layout[y][x]);
    }
    println!();
  }
}

fn part_one(input: &str) -> usize {
  let mut layout = parse_input(input);
  let mut seen: HashSet<Vec<Vec<char>>> = HashSet::new();
  seen.insert(layout.clone());
  let mut i = 0;
  loop {
    let new_layout = mutate(&layout);
    i += 1;
    let count = new_layout.iter().flatten().filter(|n| **n == '#').count();
    // println!("{} {}", i, count);
    // print_layout(&new_layout);
    layout = new_layout.to_owned();
    if seen.contains(&layout) {
      break;
    }
    seen.insert(layout.clone());
  }

  layout.iter().flatten().filter(|n| **n == '#').count()
}

fn visible_seats(layout: &Vec<Vec<char>>, p0: Point) -> usize {
  lazy_static! {
    static ref NEIGHBOURS:Vec<(i8, i8)> = vec![
      (-1, -1), // NW
      (-1, 0), // N
      (-1, 1), // NE
      (0, -1), // W
      (0, 1), // E
      (1, -1), // SW
      (1, 0), // S
      (1, 1), // SE
     ];
  }
  let height = layout.len() - 1;
  let width = layout[0].len() - 1;

  let mut count = 0;
  for dir in NEIGHBOURS.borrow().iter() {
    let mut point = p0.add(Point { x: dir.0, y: dir.1 });
    while point.x >= 0 && point.x <= width as i8 && point.y >= 0 && point.y <= height as i8 {
      if layout[point.y as usize][point.x as usize] == 'L' {
        break;
      }
      if layout[point.y as usize][point.x as usize] == '#' {
        count += 1;
        break;
      }
      point = point.add(Point { x: dir.0, y: dir.1 });
    }
  }
  count
}

fn mutate2(layout: &Vec<Vec<char>>) -> Vec<Vec<char>> {
  let height = layout.len() - 1;
  let width = layout[0].len() - 1;
  let mut new_layout = layout.clone();
  for y in 0..=height {
    for x in 0..=width {
      let p = Point {
        y: y as i8,
        x: x as i8,
      };
      let new_state = match layout[y][x] {
        'L' => {
          let no_adj_occupied = visible_seats(layout, p) == 0;
          if no_adj_occupied {
            '#'
          } else {
            'L'
          }
        }
        '#' => {
          let four_occupied = visible_seats(layout, p) >= 5;
          if four_occupied {
            'L'
          } else {
            '#'
          }
        }
        '.' => '.',
        _ => unreachable!("impossiburu"),
      };
      new_layout[y][x] = new_state;
    }
  }
  new_layout
}

fn part_two(input: &str) -> usize {
  let mut layout = parse_input(input);
  let mut seen: HashSet<Vec<Vec<char>>> = HashSet::new();
  seen.insert(layout.clone());
  let mut i = 0;
  loop {
    let new_layout = mutate2(&layout);
    i += 1;
    let count = new_layout.iter().flatten().filter(|n| **n == '#').count();
    println!("{} {}", i, count);
    print_layout(&new_layout);
    layout = new_layout.to_owned();
    if seen.contains(&layout) {
      break;
    }
    seen.insert(layout.clone());
  }

  layout.iter().flatten().filter(|n| **n == '#').count()
}

fn main() {
  let input = read_input();
  println!("{:?}", input);

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

  const TEST_INPUT: &str = "L.LL.LL.LL
LLLLLLL.LL
L.L.L..L..
LLLL.LL.LL
L.LL.LL.LL
L.LLLLL.LL
..L.L.....
LLLLLLLLLL
L.LLLLLL.L
L.LLLLL.LL";

  #[test]
  fn test_point_adj8() {
    let p0 = Point { y: 0, x: 0 };
    assert_eq!(p0.adj8().len(), 8);
    assert_eq!(
      p0.adj8(),
      vec![
        Point { y: -1, x: -1 },
        Point { y: -1, x: 0 },
        Point { y: -1, x: 1 },
        Point { y: 0, x: -1 },
        Point { y: 0, x: 1 },
        Point { y: 1, x: -1 },
        Point { y: 1, x: 0 },
        Point { y: 1, x: 1 },
      ]
    );
  }

  #[test]
  fn test_visible_seats() {
    let state = ".......#.
...#.....
.#.......
.........
..#L....#
....#....
.........
#........
...#.....";
    let point = Point { y: 4, x: 3 };
    let state = parse_input(state);
    assert_eq!(state[point.y as usize][point.x as usize], 'L');
    assert_eq!(visible_seats(&state, point), 8);

    let state = ".##.##.
#.#.#.#
##...##
...L...
##...##
#.#.#.#
.##.##.";
    let point = Point { y: 3, x: 3 };
    let state = parse_input(state);
    assert_eq!(state[point.y as usize][point.x as usize], 'L');
    assert_eq!(visible_seats(&state, point), 0);

    let state = ".............
.L.L.#.#.#.#.
.............";
    let point = Point { y: 1, x: 1 };
    let state = parse_input(state);
    assert_eq!(state[point.y as usize][point.x as usize], 'L');
    assert_eq!(visible_seats(&state, point), 0);

    let state = "#.##.##.##
#######.##
#.#.#..#..
####.##.##
#.##.##.##
#.#####.##
..#.#.....
##########
#.######.#
#.#####.##";
    let state = parse_input(state);
    let point = Point { y: 0, x: 0 };
    assert_eq!(state[point.y as usize][point.x as usize], '#');
    assert_eq!(visible_seats(&state, point), 3);

    let state = "#.LL.LL.L#
#LLLLLL.LL
L.L.L..L..
LLLL.LL.LL
L.LL.LL.LL
L.LLLLL.LL
..L.L.....
LLLLLLLLL#
#.LLLLLL.L
#.LLLLL.L#";
    let state = parse_input(state);
    let point = Point { y: 2, x: 0 };
    assert_eq!(state[point.y as usize][point.x as usize], 'L');
    assert_eq!(visible_seats(&state, point), 1);
    let point = Point { y: 0, x: 2 };
    assert_eq!(state[point.y as usize][point.x as usize], 'L');
    assert_eq!(visible_seats(&state, point), 1);
  }

  #[test]
  fn test_mutate() {
    let mut layout = vec![vec!['L', '.', 'L', 'L', '.', 'L', 'L', '.', 'L', 'L']];
    let expected = vec![vec!['#', '.', '#', '#', '.', '#', '#', '.', '#', '#']];
    assert_eq!(mutate(&mut layout), expected)
  }

  #[test]
  fn test_mutate_multi() {
    let state1 = "#.##.##.##
#######.##
#.#.#..#..
####.##.##
#.##.##.##
#.#####.##
..#.#.....
##########
#.######.#
#.#####.##";
    let state2 = "#.LL.L#.##
#LLLLLL.L#
L.L.L..L..
#LLL.LL.L#
#.LL.LL.LL
#.LLLL#.##
..L.L.....
#LLLLLLLL#
#.LLLLLL.L
#.#LLLL.##";
    let mut layout = parse_input(state1);
    let expected = parse_input(state2);
    assert_eq!(mutate(&mut layout), expected)
  }

  #[test]
  fn test_mutate2() {
    let state1 = "L.LL.LL.LL
LLLLLLL.LL
L.L.L..L..
LLLL.LL.LL
L.LL.LL.LL
L.LLLLL.LL
..L.L.....
LLLLLLLLLL
L.LLLLLL.L
L.LLLLL.LL";
    let state2 = "#.##.##.##
#######.##
#.#.#..#..
####.##.##
#.##.##.##
#.#####.##
..#.#.....
##########
#.######.#
#.#####.##";
    let state3 = "#.LL.LL.L#
#LLLLLL.LL
L.L.L..L..
LLLL.LL.LL
L.LL.LL.LL
L.LLLLL.LL
..L.L.....
LLLLLLLLL#
#.LLLLLL.L
#.LLLLL.L#";
    let state4 = "#.L#.##.L#
#L#####.LL
L.#.#..#..
##L#.##.##
#.##.#L.##
#.#####.#L
..#.#.....
LLL####LL#
#.L#####.L
#.L####.L#";
    assert_eq!(mutate2(&parse_input(state1)), parse_input(state2));
    assert_eq!(mutate2(&parse_input(state2)), parse_input(state3));
    assert_eq!(mutate2(&parse_input(state3)), parse_input(state4));
  }

  #[test]
  fn test_parse_input() {
    assert_eq!(
      parse_input(TEST_INPUT)[0],
      vec!['L', '.', 'L', 'L', '.', 'L', 'L', '.', 'L', 'L']
    )
  }

  #[test]
  fn test_part_one() {
    assert_eq!(part_one(TEST_INPUT), 37);
    assert_eq!(part_one(&read_input()), 2441);
  }

  #[test]
  fn test_part_two() {
    assert_eq!(part_two(TEST_INPUT), 26);
    // assert_eq!(part_two(&read_input()), 0);
  }
}
