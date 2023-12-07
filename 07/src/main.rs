use std::{env, fs};

fn main() {
    let args: Vec<_> = env::args().collect();
    let filename = args.get(1).expect("give filename");

    let contents = fs::read_to_string(filename).expect("file exist. now.");

    let mut hands = camel::parse(&contents);

    hands.sort();

    let winnings: usize = hands
        .iter()
        .enumerate()
        .map(|(i, h)| {
            let rank = i + 1;
            rank * h.bid as usize
        })
        .sum();

    println!("solution 1 (winnings): {}", winnings);
}
