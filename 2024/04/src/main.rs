use std::{env, error::Error, fs};

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<_> = env::args().collect();
    let filename = args.get(1).expect("should provide a filename");
    let search_text = args.get(2).expect("should provide a query");
    let contents = fs::read_to_string(filename)?;

    search::run(&contents, search_text)?;

    Ok(())
}
