use common::io;
use md5;

fn main() {
    let input = io::read_input("2015-04");
    let mut i = 1;
    let key = &input;
    let mut five_zeroes_found = false;
    loop {
        let hash: String = format!("{:x}", md5::compute(format!("{}{}", key, i)));
        if !five_zeroes_found && hash.starts_with("00000") {
            five_zeroes_found = true;
            println!("{}", i);
        }
        if hash.starts_with("000000") {
            println!("{}", i);
            break;
        }
        i += 1;
    }
}
