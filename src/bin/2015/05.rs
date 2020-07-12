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
fn is_allowed(string: String) -> bool {
    return match string.as_str() {
        "ab" => false,
        "cd" => false,
        "pq" => false,
        "xy" => false,
        _ => true,
    };
}

fn is_nice(string: &str) -> bool {
    let mut vowels = 0;
    let mut consecutive = 0;
    let mut prev: (usize, char) = (0, '1');
    for chars in string.chars().enumerate() {
        let index = chars.0;
        let ch = chars.1;
        if is_vowel(ch) {
            vowels += 1;
        }
        if index > 0 {
            if prev.1 == ch {
                consecutive += 1;
            }
            let lol = format!("{}{}", prev.1, ch);
            if !is_allowed(lol) {
                return false;
            }
        }
        prev = (index, ch);
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

fn main() {
    let input = io::read_input("2015-05");
    let mut nice_1 = 0;
    let mut nice_2 = 0;
    for line in input.split_whitespace() {
        if is_nice(line) {
            nice_1 += 1;
        }
        if is_nice_2(line) {
            nice_2 += 1;
        }
    }
    println!("{}", nice_1);
    println!("{}", nice_2);
}
