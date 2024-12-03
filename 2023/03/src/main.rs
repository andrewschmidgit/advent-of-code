use std::{env, fs};


fn main() {
    let args: Vec<_> = env::args().collect();
    let filename = args.get(1).expect("Should give me a filename as only arg");

    let contents = fs::read_to_string(filename).expect("filename should exist");

    gear::run(&contents);
}

