use common::io;
fn main() {
    let input = io::read_input("2015-01");
    let mut floor: i32 = 0;
    let mut basement: bool = false;
    let mut index: u32 = 0;
    for ch in input.chars() {
        if ch == '(' {
            floor += 1;
        } else {
            floor -= 1;
        }
        if !basement {
            index += 1;
        }
        if floor == -1 {
            basement = true;
        }
    }
    println!("{}", floor);
    println!("{}", index);
}
