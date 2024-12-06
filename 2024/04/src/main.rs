use std::{env, error::Error, fs};

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<_> = env::args().collect();
    let filename = args.get(1).expect("should provide a filename");
    let pattern_file = args.get(2).expect("should provide a pattern file");
    let contents = fs::read_to_string(filename)?;
    let pattern_contents = fs::read_to_string(pattern_file)?;

    search::run(&contents, &pattern_contents)?;

    Ok(())
}
