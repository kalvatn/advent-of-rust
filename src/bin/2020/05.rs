use std::time::Instant;

use common::io;

fn read_input() -> String {
  return io::read_input("2020-05");
}

fn get_seat(encoded: &str) -> (u16, u16) {
  let mut row: (u16, u16) = (0, 128);
  let mut col: (u16, u16) = (0, 8);
  for (i, c) in encoded.char_indices() {
    if i < 7 {
      match c {
        'F' => row.1 -= (row.1 - row.0) / 2,
        'B' => row.0 += (row.1 - row.0) / 2,
        _ => unreachable!("invalid"),
      }
    } else {
      match c {
        'L' => col.1 -= (col.1 - col.0) / 2,
        'R' => col.0 += (col.1 - col.0) / 2,
        _ => unreachable!("invalid"),
      }
    }
  }
  return (row.0, col.0);
}

fn get_seat_ids(input: &str) -> Vec<u16> {
  input
    .lines()
    .map(|l| {
      let (row, col) = get_seat(l);
      row * 8 + col
    })
    .collect()
}

fn part_one(input: &str) -> u16 {
  *get_seat_ids(input).iter().max().unwrap()
}

fn part_two(input: &str) -> u16 {
  let mut seat_ids = get_seat_ids(input);
  seat_ids.sort();
  let mut prev = seat_ids[0];
  for i in 1..seat_ids.len() {
    let x = seat_ids[i];
    if (x - prev) > 1 {
      return prev + 1;
    }
    prev = x;
  }
  unreachable!("impossiburu")
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

  #[test]
  fn test_get_row() {
    assert_eq!(get_seat("FBFBBFFRLR").0, 44);
    assert_eq!(get_seat("BFFFBBFRRR").0, 70);
    assert_eq!(get_seat("FFFBBBFRRR").0, 14);
    assert_eq!(get_seat("BBFFBBFRLL").0, 102);
  }

  #[test]
  fn test_get_col() {
    assert_eq!(get_seat("FBFBBFFRLR").1, 5);
    assert_eq!(get_seat("BFFFBBFRRR").1, 7);
    assert_eq!(get_seat("FFFBBBFRRR").1, 7);
    assert_eq!(get_seat("BBFFBBFRLL").1, 4);
  }

  #[test]
  fn test_part_one() {
    assert_eq!(part_one(&read_input()), 835);
  }

  #[test]
  fn test_part_two() {
    assert_eq!(part_two(&read_input()), 649);
  }
}
