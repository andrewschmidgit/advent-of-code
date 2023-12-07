use wait::{parse, parse_as_single};
use std::{env, fs};

fn main() {
    let args: Vec<_> = env::args().collect();
    let filename = args.get(1).expect("Give filename");
    let contents = fs::read_to_string(filename).expect("Not real file");

    let races = crate::parse(&contents);

    let error: usize = races.into_iter().map(|r| r.wins()).product();
    println!("solution 1 (margin of error): {}", error);

    let race = parse_as_single(&contents);
    let error = race.wins();

    println!("solution 2 (margin of error): {}", error);
}

