use grid::Grid;
use std::error::Error;

mod grid;
mod point;
mod vector;

pub fn run(contents: &str, search_text: &str) -> Result<(), Box<dyn Error>> {
    let grid: Grid<char> = contents.parse()?;

    let search_result = grid
        .search(&search_text.chars().collect::<Vec<_>>())
        .ok_or("No occurrences found")?;

    println!("found occurences: {}", search_result.len());

    Ok(())
}
