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

use common::io;


#[derive(Debug)]
struct Point3D {
  x: usize,
  y: usize,
  z: i32
}

fn within_bounds(n1:usize, n2:i32) -> bool {
  let n3 = n1 as i32 + n2;
  n3 >= 0  && n3 < SIZE as i32
}

impl Point3D {

  fn add(&self, other:Point3D) -> Point3D {
    Point3D {
      x : self.x + other.x,
      y : self.y + other.y,
      z : self.z + other.z
    }
  }


  fn adj(&self) -> Vec<Point3D> {
    let mut neighbours = vec![];
    let diffs:Vec<i32> = vec![-1, 0, 1];
    for x in &diffs {
      for y in &diffs {
        for z in &diffs {
          if within_bounds(self.x, *x) && within_bounds(self.y,*y) {
            let nb = Point3D {
              x: (self.x as i32 + x) as usize,
              y: (self.y as i32 + y) as usize,
              z: self.z + z
            };
            if !(nb.x == self.x && nb.y == self.y && nb.z == self.z) {
              neighbours.push(nb);
            }

          }
        }
      }
    }
    neighbours
  }
}


const SIZE:usize = 21;

fn read_input() -> String {
  return io::read_input("2020-17");
}

fn parse_input(input: &str) -> HashMap<i32, Vec<Vec<char>>> {
  let mut map = HashMap::new();

  let mut grid = new_layer();
  let parsed = input.lines().map(|line| line.chars().collect()).collect::<Vec<Vec<char>>>();
  let middle = (SIZE / 2) - (parsed.len() / 2);
  for y in 0..parsed.len() {
    for x in 0..parsed.len() {
      grid[middle+y][middle+x] = parsed[y][x];
    }
  }
  map.insert(0, grid);
  map
}

fn new_layer() -> Vec<Vec<char>> {
  let mut layer = vec![];

  for y in  0..SIZE {
    layer.push(vec![]);
    for x in 0..SIZE {
      layer[y].push('.');
    }
  }
  layer
}

fn mutate(layers: &HashMap<i32, Vec<Vec<char>>>) -> HashMap<i32, Vec<Vec<char>>> {

  let mut new_layers = layers.clone();
  let min_z = layers.keys().min().unwrap()-1;
  let max_z = layers.keys().max().unwrap()+1;
  new_layers.entry(min_z).or_insert(new_layer());
  new_layers.entry(max_z).or_insert(new_layer());

  for z in min_z..=max_z {
    for y in 0..SIZE {
      for x in 0..SIZE {
        let p = Point3D { x, y, z };
        let is_active = new_layers.get(&p.z).unwrap()[p.y][p.x] == '#';
        let nb = p.adj();
        let mut count_active_nb = 0;
        for n in nb {
          if layers.contains_key(&n.z) {
            if layers.get(&n.z).unwrap()[n.y][n.x] == '#' {
              count_active_nb += 1;
            }
          }
        }
        if is_active {
          if count_active_nb == 2 || count_active_nb == 3 {
            // println!("{:?} {:?}", p, count_active_nb);
            new_layers.get_mut(&p.z).unwrap()[p.y][p.x] = '#';
          } else {
            new_layers.get_mut(&p.z).unwrap()[p.y][p.x] = '.';
          }
        } else {
          if count_active_nb == 3 {
            // println!("{:?} {:?}", p, count_active_nb);
            new_layers.get_mut(&p.z).unwrap()[p.y][p.x] = '#';
          } else {
            new_layers.get_mut(&p.z).unwrap()[p.y][p.x] = '.';

          }
        }
      }
    }
  }


  new_layers
}

fn print_layers(layers:&HashMap<i32, Vec<Vec<char>>>) {
  let min_z: i32 = *layers.keys().min().unwrap();
  let max_z: i32 = *layers.keys().max().unwrap();
  for z in min_z..=max_z {
    println!("layer {}", z);
    let grid = layers.get(&z).unwrap();
    for y in 0..SIZE {
      for x in 0..SIZE {
        print!("{}", grid[y][x])
      }
      println!();
    }
  }
}

fn part_one(input: &str) -> usize {
  let mut layers = parse_input(input);

  for cycle in 0..6 {
    println!("cycle {}", cycle);
    // print_layers(&layers);
    // println!();
    layers = mutate(&layers);
  }

  layers.values().flatten().flatten().filter(|c| **c == '#').count()
}

fn part_two(input: &str) -> usize {
  return 0;
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

  const TEST_INPUT: &str = ".#.
..#
###";

  #[test]
  fn test_adj() {
    let p = Point3D {
      x: 1,
      y: 2,
      z: 3
    };
    assert_eq!(p.adj().len(), 26);
  }

  #[test]
  fn test_part_one() {
    assert_eq!(part_one(TEST_INPUT), 112);
    // assert_eq!(part_one(&read_input()), 0);
  }

  #[test]
  fn test_part_two() {
    assert_eq!(part_two(TEST_INPUT), 0);
    // assert_eq!(part_two(&read_input()), 0);
  }
}
