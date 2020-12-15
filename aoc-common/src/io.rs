use reqwest::blocking::Client;
use reqwest::Error;
use std::fs::File;
use std::io;
use std::io::Read;
use std::path::Path;

pub fn download_input(url: &str) -> Result<String, Error> {
  let client = Client::new();
  let cookie = read_cookie();

  println!("downloading input from {}", url);
  let response = client
    .get(url)
    .header("Cookie", format!("session={}", cookie))
    .send()?;
  let text = response.text()?;
  Ok(text)
}

fn read_cookie() -> String {
  let mut file = File::open("~/.aoc-session_cookie").expect("failed to open .session_cookie file");
  let mut contents = String::new();
  file
    .read_to_string(&mut contents)
    .expect("failed to read .session_cookie file");
  return contents;
}

pub fn read_input_for_benchmark(filename: &str) -> String {

  let dir = "inputs";
  let dir_path = Path::new(&dir);
  if !dir_path.exists() {
    std::fs::create_dir_all(dir_path).expect("error creating path");
  }
  let filepath = format!("{}/{}", dir, filename);
  let path = Path::new(&filepath);
  let file = match path.exists() {
    true => File::open(path),
    false => panic!(format!("unable to locate file {}", filepath))
  };
  let mut contents = String::new();

  file
    .unwrap()
    .read_to_string(&mut contents)
    .expect("unable to read contents");
  return contents;
}

pub fn read_stdin() -> String {

  let mut buffer = String::new();
  let stdin = io::stdin();
  let mut handle = stdin.lock();

  handle.read_to_string(&mut buffer).expect("error reading from stdin");
  buffer
}

pub fn read_input(filename: &str) -> String {
  if filename.is_empty() {
    panic!("empty filename")
  }
  let filepath = format!("inputs/{}", filename);
  let path = Path::new(&filepath);
  let file = match path.exists() {
    true => File::open(path),
    false => {
      let parts: Vec<u32> = filename
        .split('-')
        .map(|x: &str| x.parse().unwrap())
        .collect();
      let url = format!(
        "https://adventofcode.com/{}/day/{}/input",
        parts[0], parts[1]
      );
      let response = download_input(&url);
      let mut out = File::create(&path).expect("failed to create file");
      io::copy(&mut response.unwrap().as_bytes(), &mut out).expect("failed to copy contents");
      File::open(&path)
    }
  };
  let mut contents = String::new();

  file
    .unwrap()
    .read_to_string(&mut contents)
    .expect("unable to read contents");
  return contents;
}
