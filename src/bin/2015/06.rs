use common::io;
use lazy_static::lazy_static;
use regex::Regex;

#[derive(Debug)]
enum Action {
    TurnOn,
    TurnOff,
    Toggle,
}

impl Action {
    fn from_string(string: &str) -> Action {
        match string {
            "turn on" => Action::TurnOn,
            "turn off" => Action::TurnOff,
            "toggle" => Action::Toggle,
            _ => panic!("impossiburu"),
        }
    }
}

#[derive(Debug)]
struct Instruction {
    row_start: usize,
    col_start: usize,
    row_end: usize,
    col_end: usize,
    action: Action,
}
impl Instruction {
    fn from_string(string: &str) -> Instruction {
        lazy_static! {
            static ref RE: Regex =
                Regex::new(r"^(turn on|turn off|toggle) (\d+),(\d+) through (\d+),(\d+)$").unwrap();
        }
        RE.captures(string)
            .map(|cap| Instruction {
                row_start: cap[2].parse().unwrap(),
                row_end: cap[4].parse::<usize>().unwrap() + 1,
                col_start: cap[3].parse().unwrap(),
                col_end: cap[5].parse::<usize>().unwrap() + 1,
                action: Action::from_string(&cap[1]),
            })
            .unwrap()
    }
}

fn main() {
    let input = io::read_input("2015-06");
    let mut instructions: Vec<Instruction> = vec![];
    for line in input.split("\n") {
        let ins = Instruction::from_string(line);
        instructions.push(ins);
    }
    let mut grid = [[0u32; 1000]; 1000];
    let mut grid2 = [[0u32; 1000]; 1000];
    for instruction in instructions {
        for y in instruction.row_start..instruction.row_end {
            for x in instruction.col_start..instruction.col_end {
                match instruction.action {
                    Action::TurnOn => {
                        grid[y][x] = 1;
                        grid2[y][x] += 1;
                    }
                    Action::TurnOff => {
                        grid[y][x] = 0;
                        let cur = grid2[y][x];
                        grid2[y][x] -= if cur > 0 { 1 } else { 0 };
                    }
                    Action::Toggle => {
                        let cur = grid[y][x];
                        grid[y][x] = if cur == 0 { 1 } else { 0 };
                        grid2[y][x] += 2;
                    }
                }
            }
        }
    }
    let mut count = 0;
    let mut brightness = 0;
    for y in 0..grid.len() {
        for x in 0..grid.len() {
            if grid[y][x] == 1 {
                count += 1;
            }
            brightness += grid2[y][x];
        }
    }
    println!("{}", count);
    println!("{}", brightness);
}
