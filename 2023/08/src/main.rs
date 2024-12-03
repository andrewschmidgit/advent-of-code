use std::{env, error::Error};
use std::fs;

fn main() -> Result<(), Box<dyn Error>>{
    let args: Vec<_> = env::args().collect();
    let filename = args.get(1).expect("filename plz");
    let contents = fs::read_to_string(filename)?;

    wasteland::run(&contents)?;

    Ok(())
}
