use std::time::Instant;

use common::io;

fn unescape(s: &str) -> String {
    let mut out = String::new();

    let mut iter = s.chars();

    let open_quote = iter.next().unwrap();
    if open_quote != '"' {
        panic!(format!("String doesnt start with \": '{}'", s));
    }

    while let Some(c) = iter.next() {
        if c == BACKSLASH {
            let et = iter.next().unwrap();
            match et {
                BACKSLASH => out.push(BACKSLASH),
                QUOTE => out.push(QUOTE),
                'x' => {
                    let a = iter.next().unwrap();
                    let b = iter.next().unwrap();
                    let hex = u8::from_str_radix(&format!("{}{}", a, b, ), 16).unwrap();
                    out.push(hex as char);
                }
                _ => panic!(format!("unknown escape code: '{}'", et)),
            }
        } else {
            out.push(c);
        }
    }

    if let Some(c) = out.pop() {
        if c != QUOTE {
            panic!(format!("String doesnt end with \": '{}'", s));
        }
    }
    out
}

const QUOTE: char = '"';
const ESCAPED_QUOTE: char = '\"';
const BACKSLASH:char = '\\';

fn escape(s: &str) -> String {
    let mut out = String::new();
    out.push(ESCAPED_QUOTE);

    for c in s.chars() {
        match c {
            QUOTE => {
                out.push(BACKSLASH);
                out.push(QUOTE);
            }
            BACKSLASH => {
                out.push(BACKSLASH);
                out.push(BACKSLASH);
            }
            _ => {
                out.push(c);
            }
        }
    }

    out.push(ESCAPED_QUOTE);
    return out;
}

fn count_code_chars(s: &str) -> usize {
    s.len()
}

fn count_data_chars(s: &str) -> usize {
    let unescaped = unescape(s);
    return unescaped.chars().count();
}

fn count_encoded_chars(s: &str) -> usize {
    let escaped = escape(s);
    return escaped.chars().count();
}

fn part_one(input: &str) -> u32 {
    let parsed: Vec<&str> = input.lines().collect();
    let code_counts: Vec<usize> = parsed.iter().map(|s| count_code_chars(s)).collect();
    let data_counts: Vec<usize> = parsed.iter().map(|s| count_data_chars(s)).collect();
    let sum_code: u32 = code_counts.iter().sum::<usize>() as u32;
    let sum_data: u32 = data_counts.iter().sum::<usize>() as u32;
    return sum_code - sum_data;
}

fn part_two(input: &str) -> u32 {
    let parsed: Vec<&str> = input.lines().collect();
    let code_counts: Vec<usize> = parsed.iter().map(|s| count_code_chars(s)).collect();
    let encoded_counts: Vec<usize> = parsed.iter().map(|s| count_encoded_chars(s)).collect();
    let sum_code: u32 = code_counts.iter().sum::<usize>() as u32;
    let sum_encoded: u32 = encoded_counts.iter().sum::<usize>() as u32;
    return sum_encoded - sum_code;
}

fn read_input() -> String {
    return io::read_input("2015-08");
}

fn main() {
    let input = read_input();
    let p1_timer = Instant::now();
    println!("part one {} {}ms", part_one(&input), p1_timer.elapsed().as_millis());
    let p2_timer = Instant::now();
    println!("part two {} {}ms", part_two(&input), p2_timer.elapsed().as_millis());
    println!("total {}ms", p1_timer.elapsed().as_millis())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_count_code_chars() {
        assert_eq!(count_code_chars("\"\""), 2);
        assert_eq!(count_code_chars("\"abc\""), 5);
        assert_eq!(count_code_chars("\"aaa\\\"aaa\""), 10);
        assert_eq!(count_code_chars("\"\\x27\""), 6);
    }

    #[test]
    fn test_count_data_chars() {
        assert_eq!(count_data_chars("\"\""), 0);
        assert_eq!(count_data_chars("\"abc\""), 3);
        assert_eq!(count_data_chars("\"aaa\\\"aaa\""), 7);
        assert_eq!(count_data_chars("\"\\x27\""), 1);
    }

    #[test]
    fn test_count_encoded_chars() {
        assert_eq!(count_encoded_chars("\"\""), 6);
        assert_eq!(count_encoded_chars("\"abc\""), 9);
        assert_eq!(count_encoded_chars("\"aaa\\\"aaa\""), 16);
        assert_eq!(count_encoded_chars("\"\\x27\""), 11);
    }

    #[test]
    fn test_part_one() {
        assert_eq!(part_one(&read_input()), 1350);
    }

    #[test]
    fn test_part_two() {
        assert_eq!(part_two(&read_input()), 2085);
    }
}
