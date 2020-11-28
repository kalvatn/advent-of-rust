use std::collections::{HashMap, HashSet, VecDeque};
use std::fmt::Debug;
use std::iter::FromIterator;
use std::str::FromStr;
use std::time::Instant;

use lazy_static::lazy_static;
use regex::Regex;

use common::io;

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Clone, Hash)]
struct Location {
  name: String,
}

#[derive(Debug, Eq, PartialEq)]
struct Route {
  a: Location,
  b: Location,
  length: usize,
}

struct WeightedGraph<T>
where
  T: std::hash::Hash,
{
  edges: HashMap<T, HashMap<T, usize>>,
}

#[derive(Debug, Eq, PartialEq, Clone, Hash)]
struct Path<T>
where
  T: std::clone::Clone,
{
  path: Vec<T>,
  distance: usize,
}

#[derive(Eq, PartialEq, Clone, Hash)]
struct Node<T>
where
  T: std::clone::Clone,
{
  value: T,
  distance: usize,
  parent: Option<Box<Node<T>>>,
}

impl<T: std::cmp::Eq + std::hash::Hash + std::clone::Clone + Debug> WeightedGraph<T> {
  pub fn new() -> Self {
    WeightedGraph {
      edges: HashMap::new(),
    }
  }

  pub fn connect(&mut self, source: T, target: T, weight: usize) {
    self
      .edges
      .entry(source.clone())
      .or_insert(HashMap::new())
      .entry(target.clone())
      .or_insert(weight);
    self
      .edges
      .entry(target.clone())
      .or_insert(HashMap::new())
      .entry(source.clone())
      .or_insert(weight);
  }

  fn construct_path(&self, node: Node<T>, path: Path<T>) -> Path<T> {
    let mut new_path = path.path;
    new_path.push(node.value.clone());
    let path1 = Path {
      path: new_path,
      distance: path.distance + node.distance,
    };
    if node.parent.is_none() {
      return path1;
    }
    return self.construct_path(*node.parent.unwrap(), path1);
  }

  pub fn find_best_path_visit_all(
    &self,
    start: T,
    shortest: bool,
    the_best: usize,
  ) -> Option<Path<T>> {
    let mut best = the_best.clone();
    let keys: HashSet<&T> = self.edges.keys().into_iter().collect();

    let mut paths: Vec<Path<T>> = Vec::new();
    let mut visited: HashSet<Node<T>> = HashSet::new();
    let mut queue: VecDeque<Node<T>> = VecDeque::new();
    queue.push_back(Node {
      value: start,
      distance: 0,
      parent: None,
    });

    while !queue.is_empty() {
      let current = queue.pop_back().unwrap();

      if visited.contains(&current) {
        continue;
      }
      let path = self.construct_path(
        current.clone(),
        Path {
          path: vec![],
          distance: 0,
        },
      );
      if path.path.len() > keys.len() {
        continue;
      }

      if shortest {
        if path.distance > best {
          continue;
        }
      } else {
        if path.path.len() >= keys.len() && path.distance < best {
          continue;
        }
      }

      let path_keys: HashSet<&T> = HashSet::from_iter(path.path.iter());
      if path_keys == keys {
        let is_better: bool;
        if shortest {
          is_better = path.distance < best;
        } else {
          is_better = path.distance > best;
        }

        if is_better {
          println!("found better path {:?}", path);
          best = path.distance;
          paths.push(path);
          // paths.insert(path);
        }
      }

      visited.insert(current.clone());
      self
        .edges
        .get(&current.value)
        .unwrap()
        .iter()
        .for_each(|(k, v)| {
          let node = Node {
            value: k.clone(),
            distance: *v,
            parent: Option::from(Box::from(current.clone())),
          };
          queue.push_back(node);
        });
    }

    if paths.is_empty() {
      return None;
    }
    let mut paths: Vec<Path<T>> = paths.into_iter().collect();

    if shortest {
      paths.sort_by(|a, b| a.distance.cmp(&b.distance));
    } else {
      paths.sort_by(|a, b| b.distance.cmp(&a.distance));
    }
    return Some(paths[0].clone());
  }
}

impl FromStr for Route {
  type Err = ();

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    lazy_static! {
      static ref RE: Regex = Regex::new(r"^(\w+) to (\w+) = (\d+)$").unwrap();
    }
    let loc = RE
      .captures(s)
      .map(|cap| Route {
        a: Location {
          name: cap[1].to_string(),
        },
        b: Location {
          name: cap[2].to_string(),
        },
        length: cap[3].parse::<usize>().unwrap(),
      })
      .unwrap();
    Ok(loc)
  }
}

fn parse_input(input: &str) -> Vec<Route> {
  return input
    .lines()
    .map(|line| Route::from_str(&line).unwrap())
    .collect();
}

fn part_one(input: &str) -> usize {
  let routes = parse_input(input);
  let mut graph: WeightedGraph<String> = WeightedGraph::new();
  routes.iter().for_each(|route| {
    graph.connect(
      route.a.name.to_string(),
      route.b.name.to_string(),
      route.length,
    )
  });

  let mut best: Path<String> = Path {
    path: vec![],
    distance: usize::max_value(),
  };
  for x in graph.edges.keys() {
    let newbest = graph.find_best_path_visit_all(x.clone(), true, best.distance);
    if newbest.is_some() {
      let path = newbest.unwrap();
      let dist = path.distance;
      if dist < best.distance {
        best = path;
      }
    }
  }
  println!("{:?}", best);
  best.distance
}

fn part_two(input: &str) -> usize {
  let routes = parse_input(input);
  let mut graph: WeightedGraph<String> = WeightedGraph::new();
  routes.iter().for_each(|route| {
    graph.connect(
      route.a.name.to_string(),
      route.b.name.to_string(),
      route.length,
    )
  });

  let mut best: Path<String> = Path {
    path: vec![],
    distance: usize::min_value(),
  };
  for x in graph.edges.keys() {
    let newbest = graph.find_best_path_visit_all(x.clone(), false, best.distance);
    if newbest.is_some() {
      let path = newbest.unwrap();
      let dist = path.distance;
      if dist > best.distance {
        best = path;
      }
    }
  }
  println!("{:?}", best);
  best.distance
}

fn read_input() -> String {
  return io::read_input("2015-09");
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

  const TEST_INPUT: &str = "London to Dublin = 464
London to Belfast = 518
Dublin to Belfast = 141";

  #[test]
  fn test_route_from_str() {
    assert_eq!(
      Route::from_str("London to Dublin = 464").unwrap(),
      Route {
        a: Location {
          name: "London".to_string()
        },
        b: Location {
          name: "Dublin".to_string()
        },
        length: 464,
      }
    )
  }

  #[test]
  fn test_parse_input() {
    assert_eq!(
      parse_input(TEST_INPUT),
      vec![
        Route {
          a: Location {
            name: "London".to_string()
          },
          b: Location {
            name: "Dublin".to_string()
          },
          length: 464,
        },
        Route {
          a: Location {
            name: "London".to_string()
          },
          b: Location {
            name: "Belfast".to_string()
          },
          length: 518,
        },
        Route {
          a: Location {
            name: "Dublin".to_string()
          },
          b: Location {
            name: "Belfast".to_string()
          },
          length: 141,
        },
      ]
    )
  }

  #[test]
  fn test_part_one() {
    assert_eq!(part_one(TEST_INPUT), 605);
    assert_eq!(part_one(&read_input()), 207);
  }

  #[test]
  fn test_part_two() {
    assert_eq!(part_two(TEST_INPUT), 982);
    assert_eq!(part_two(&read_input()), 804);
  }
}
