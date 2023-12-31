use std::{env, fs};

fn main() {
    let args: Vec<_> = env::args().collect();
    let filename = args.get(1).expect("Filename. Now.");
    let contents = fs::read_to_string(filename).expect("Contents. Now.");

    seed::run(&contents);
}
