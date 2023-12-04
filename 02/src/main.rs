use std::{path::PathBuf, fs, env};

fn main() -> Result<(), std::io::Error> {
    let args: Vec<_> = env::args().collect();

    let filename = PathBuf::from(&args[1]);
    let contents = fs::read_to_string(filename)?;

    cube::run(&contents);

    Ok(())
}
