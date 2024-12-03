use std::{env, error::Error, fs};

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<_> = env::args().collect();
    let filename = args.get(1).expect("should provide filename");
    let contents = fs::read_to_string(filename).unwrap();

    mull::run(&contents)?;

    Ok(())
}
