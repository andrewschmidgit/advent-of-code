use std::{env, fs};

fn main() {
    let args: Vec<_> = env::args().collect();
    let filename = args.get(1).expect("give filename");

    let contents = fs::read_to_string(filename).expect("file exist. now.");

    let winnings = camel::run(&contents, false);
    println!("solution 1 (winnings): {}", winnings);

    let winnings = camel::run(&contents, true);
    println!("solution 2 (winnings with jokers): {}", winnings);
}
