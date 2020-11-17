use common::io;

fn parse_directions(input: &str) -> Vec<char> {
    return input.chars().collect();
}

fn directions_to_int(directions: Vec<char>) -> Vec<i32> {
    return directions
        .iter()
        .map(|c| -> i32 {
            match c {
                '(' => 1,
                ')' => -1,
                _ => 0,
            }
        })
        .collect();
}

fn part_one(directions: Vec<char>) -> i32 {
    return directions_to_int(directions).iter().sum();
}

fn part_two(directions: Vec<char>) -> usize {
    return directions_to_int(directions)
        .iter()
        .scan(0, |acc, &n| {
            *acc = *acc + n;
            Some(*acc)
        })
        .take_while(|x| x != &-1)
        .count()
        + 1;
}

fn main() {
    let input = io::read_input("2015-01");
    let directions: Vec<char> = parse_directions(&*input);
    println!("{}", part_one(directions.clone()));
    println!("{}", part_two(directions));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_directions() {
        assert_eq!(parse_directions("(())"), vec!['(', '(', ')', ')']);
    }

    #[test]
    fn test_directions_to_int() {
        assert_eq!(
            directions_to_int(vec!['(', '(', ')', ')']),
            vec![1, 1, -1, -1]
        );
    }

    #[test]
    fn test_part_one() {
        assert_eq!(part_one(vec!['(', '(', ')', ')']), 0)
    }

    #[test]
    fn test_part_two() {
        assert_eq!(part_two(vec![')']), 1);
        assert_eq!(part_two(vec!['(', ')', '(', ')', ')']), 5);
    }
}
