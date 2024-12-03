use set::Set;

use crate::{game::Game, color::Color};

mod color;
mod game;
mod set;

pub fn run(contents: &str) -> Result<(), String> {
    let input: Vec<Game> = contents
        .lines()
        .map(|l| l.parse::<Game>())
        .collect::<Result<Vec<Game>, String>>()?;

    let set = Set::new(vec![Color::Red(12), Color::Green(13), Color::Blue(14)]);
    let possible: u32 = input.iter().filter_map(|g| g.is_possible(&set)).sum();
    let power: u32 = input.iter().map(|g| g.get_min_set().power()).sum();

    println!("solution 1: {}", possible);
    println!("solution 2: {}", power);

    Ok(())
}
