use std::{env, fs, error::Error};

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<_> = env::args().collect();
    let filename = args.get(1).expect("Filename");
    let contents = fs::read_to_string(filename).expect("file to be there");

    cosmic::run(&contents)?;

    Ok(())
}
