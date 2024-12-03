use std::fs;

fn main() {
    let args: Vec<_> = std::env::args().collect();
    let filename = args.get(1).expect("Should provide input filename");
    let contents = fs::read_to_string(filename).expect("File should exist");

    scratch::run(&contents);
}
