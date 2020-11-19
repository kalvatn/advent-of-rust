use std::time::Instant;

use common::io;

fn is_vowel(ch: char) -> bool {
    return match ch {
        'a' => true,
        'e' => true,
        'i' => true,
        'o' => true,
        'u' => true,
        _ => false,
    };
}

fn is_allowed(string: &str) -> bool {
    return match string {
        "ab" => false,
        "cd" => false,
        "pq" => false,
        "xy" => false,
        _ => true,
    };
}

fn is_xx(xx: &str) -> bool {
    return xx.len() == 2 && xx.chars().nth(0) == xx.chars().nth(1);
}

fn is_nice(string: &str) -> bool {
    let mut vowels = 0;
    let mut consecutive = 0;
    let mut prev: (usize, char) = (0, '?');
    for chars in string.chars().enumerate() {
        let (index, c) = chars;
        if is_vowel(c) {
            vowels += 1;
        }
        if index > 0 {
            let ab = format!("{}{}", prev.1, c);
            if is_xx(&ab) {
                consecutive += 1
            }
            if !is_allowed(&ab) {
                return false;
            }
        }
        prev = (index, c);
    }
    return vowels >= 3 && consecutive >= 1;
}

struct LetterPair {
    index: usize,
    string: String,
}

fn is_nice_2(string: &str) -> bool {
    let vec: Vec<char> = string.chars().collect();
    let mut combos: Vec<LetterPair> = vec![];
    let mut has_pattern = false;
    let mut has_repeat = false;
    for i in 1..vec.len() - 1 {
        let prev = vec[i - 1];
        let cur = vec[i];
        let next = vec[i + 1];
        if prev == next {
            has_pattern = true;
        }
        combos.push(LetterPair {
            index: i - 1,
            string: format!("{}{}", prev, cur),
        });
        combos.push(LetterPair {
            index: i,
            string: format!("{}{}", cur, next),
        });
    }
    for combo in &combos {
        for combo2 in &combos {
            if combo.index < combo2.index && combo2.index - 1 > combo.index {
                if combo2.string == combo.string {
                    has_repeat = true;
                    break;
                }
            }
        }
        if has_repeat {
            break;
        }
    }
    return has_pattern && has_repeat;
}

fn read_input() -> String {
    return io::read_input("2015-05");
}

fn part_one(input: &str) -> usize {
    return input.split_whitespace().filter(|s| is_nice(s)).count();
}

fn part_two(input: &str) -> usize {
    return input.split_whitespace().filter(|s| is_nice_2(s)).count();
}

fn main() {
    let input = &read_input();
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
    fn test_is_vowel() {
        "aeiou".chars().for_each(|c| assert!(is_vowel(c), true));
        "bcdfghjklmnpqrstvwxzy"
            .chars()
            .for_each(|c| assert!(!is_vowel(c)));
    }

    #[test]
    fn test_is_allowed() {
        vec!["ab", "cd", "pq", "xy"]
            .iter()
            .for_each(|s| assert!(!is_allowed(s)));
    }

    #[test]
    fn test_is_xx() {
        vec!["xx", "yy", "aa", "bb", "dd"]
            .iter()
            .for_each(|s| assert!(is_xx(s)));
        vec!["xy", "bc", "ax", "ay", "la", "abcd"]
            .iter()
            .for_each(|s| assert!(!is_xx(s)));
    }

    #[test]
    fn test_is_nice() {
        vec!["ugknbfddgicrmopn", "aaa"]
            .iter()
            .for_each(|s| assert!(is_nice(s)));
        vec!["jchzalrnumimnmhp", "haegwjzuvuyypxyu", "dvszwmarrgswjxmb"]
            .iter()
            .for_each(|s| assert!(!is_nice(s)));
    }

    #[test]
    fn test_is_nice_2() {
        vec!["qjhvhtzxzqqjkmpb", "xxyxx"]
            .iter()
            .for_each(|s| assert!(is_nice_2(s)));
        vec!["uurcxstgmygtbstg", "ieodomkazucvgmuy"]
            .iter()
            .for_each(|s| assert!(!is_nice_2(s)));
    }

    #[test]
    fn test_part_one() {
        assert_eq!(part_one(&read_input()), 255);
    }

    #[test]
    fn test_part_two() {
        assert_eq!(part_two(&read_input()), 55);
    }
}
