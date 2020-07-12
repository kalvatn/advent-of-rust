use std::fs::File;
use std::io::Read;

pub fn read_input(filename: &str) -> String {
    let mut file = File::open(format!("inputs/{}", filename)).expect("unable to open file");
    let mut contents = String::new();

    file.read_to_string(&mut contents)
        .expect("unable to read contents");
    return contents;
}
