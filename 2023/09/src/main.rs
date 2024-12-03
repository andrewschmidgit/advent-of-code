use std::{env, fs};

fn main() {
    let args: Vec<_> = env::args().collect();
    let filename = args.get(1).expect("filename");
    let contents = fs::read_to_string(filename).expect("contents");

    mirage::run(&contents);
}
