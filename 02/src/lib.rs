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
    let possible = get_possible(&input, set);

    let sum: u32 = possible.into_iter().sum();

    println!("sum: {}", sum);

    Ok(())
}

fn get_possible(games: &[Game], set: Set) -> Vec<u32> {
    games.iter().filter_map(|g| g.is_possible(&set)).collect()
}
