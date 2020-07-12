use common::io;

fn calc_paper(l: u32, w: u32, h: u32) -> u32 {
    let sides = vec![l * w, w * h, h * l];
    let min = sides.iter().min();
    (2 * sides[0] + 2 * sides[1] + 2 * sides[2]) + min.unwrap()
}
fn calc_ribbon(l: u32, w: u32, h: u32) -> u32 {
    let mut sides = vec![l, w, h];
    sides.sort();
    return (sides[0] * 2) + (sides[1] * 2) + (l * w * h);
}
fn main() {
    let input = io::read_input("2015-02");

    let mut sum_paper = 0;
    let mut sum_ribbon = 0;
    for line in input.split_whitespace() {
        let dim: Vec<u32> = line.split("x").map(|x| x.parse::<u32>().unwrap()).collect();
        let (x, y, z) = (dim[0], dim[1], dim[2]);
        sum_paper += calc_paper(x, y, z);
        sum_ribbon += calc_ribbon(x, y, z);
    }
    println!("{}", sum_paper);
    println!("{}", sum_ribbon);
}
