use common::io;
use std::collections::HashSet;

#[derive(Debug)]
enum Direction {
    LEFT,
    RIGHT,
    UP,
    DOWN,
}
#[derive(Eq, PartialEq, Hash, Clone)]
struct Point {
    pub x: i32,
    pub y: i32,
}

fn main() {
    let input = io::read_input("2015-03");
    let lol: Vec<Direction> = input
        .chars()
        .map(|c| match c {
            '>' => Direction::RIGHT,
            '<' => Direction::LEFT,
            '^' => Direction::UP,
            'v' => Direction::DOWN,
            _ => panic!("impossiburu"),
        })
        .collect();

    let mut santa = Point { x: 0, y: 0 };
    let mut houses: HashSet<Point> = HashSet::new();
    for dir in &lol {
        match dir {
            Direction::UP => santa.y -= 1,
            Direction::DOWN => santa.y += 1,
            Direction::LEFT => santa.x -= 1,
            Direction::RIGHT => santa.x += 1,
        };
        houses.insert(santa.clone());
    }
    println!("{}", houses.len());

    let mut santa = Point { x: 0, y: 0 };
    let mut robo = Point { x: 0, y: 0 };
    let mut houses: HashSet<Point> = HashSet::new();
    let mut i: i32 = 0;
    for dir in &lol {
        if i % 2 == 0 {
            match dir {
                Direction::UP => santa.y -= 1,
                Direction::DOWN => santa.y += 1,
                Direction::LEFT => santa.x -= 1,
                Direction::RIGHT => santa.x += 1,
            };
            houses.insert(santa.clone());
        } else {
            match dir {
                Direction::UP => robo.y -= 1,
                Direction::DOWN => robo.y += 1,
                Direction::LEFT => robo.x -= 1,
                Direction::RIGHT => robo.x += 1,
            };
            houses.insert(robo.clone());
        }
        i += 1;
    }
    println!("{}", houses.len());
}
