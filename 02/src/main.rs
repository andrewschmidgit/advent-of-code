use std::{path::PathBuf, fs, io::{Error, self}, env};


fn main() -> Result<(), io::Error> {
    let args: Vec<_> = env::args().collect();

    let filename = PathBuf::from(&args[1]);

    run(&filename)
}

fn run(filename: &PathBuf) -> Result<(), io::Error> {
    let input = get_input(filename)?;

    let possible = get_possible(&input);

    let sum: u32 = possible.into_iter().sum();

    println!("sum: {}", sum);

    Ok(())
}

fn get_input(filename: &PathBuf) -> Result<Vec<Game>, Error> {
    let contents = fs::read_to_string(filename)?;

    Ok(contents.lines().map(Game::new).collect())
}

fn get_possible(games: &[Game]) -> Vec<u32> {
    vec![]
}

#[derive(Debug)]
enum CubeColor {
    Red(u32),
    Green(u32),
    Blue(u32),
}

#[derive(Debug)]
enum CubeSet {
    One(CubeColor),
    Two(CubeColor, CubeColor),
    Three(CubeColor, CubeColor, CubeColor),
}

#[derive(Debug)]
struct Game {
    id: u32,
    sets: Vec<CubeSet>,
}

impl Game {
    pub fn new(line: &str) -> Game {
        unimplemented!();
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn name() {
        unimplemented!();
    }
}
