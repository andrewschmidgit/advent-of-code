use std::{env, error::Error, fs};

use map::Map;

mod map;

fn main() -> Result<(), Box<dyn Error>> {
    let filename = env::args().skip(1).next().expect("filename");
    let input = fs::read_to_string(&filename).expect("file");

    let map: Map = input.parse()?;

    let (score, rating) = map.score().expect("should score");

    println!("pt1 score: {}", score);
    println!("pt2 rating: {}", rating);

    Ok(())
}
