use grid::Grid;
use pattern::Pattern;
use std::error::Error;

mod grid;
mod pattern;
mod point;
mod vector;

pub fn run(contents: &str, search_text: &str) -> Result<(), Box<dyn Error>> {
    let grid: Grid<char> = contents.parse()?;

    let patterns: Vec<_> = search_text
        .split("\n\n")
        .filter_map(|s| s.parse::<Pattern>().ok())
        .collect();

    let matches: Vec<_> = patterns
        .iter()
        .filter_map(|p| Some((p, grid.find(p)?)))
        .collect();

    // for (p, indexes) in matches.iter() {
    //     println!("found {} occurrences of {:?}", indexes.len(), p);
    //     println!("indexes: {:?}", indexes);
    // }

    let what: Vec<_> = matches.into_iter().flat_map(|(_, is)| is).collect();

    println!("total occurrences: {}", what.len());

    Ok(())
}
