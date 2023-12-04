use grid::Grid;

mod element;
mod grid;
mod point;

pub fn run(contents: &str) {
    let grid: Grid = contents.parse().unwrap();

    let symbol_positions = grid.find_symbol_positions();

    let mut sum = 0;
    for sp in symbol_positions {
        let numbers = grid.find_part_numbers(sp);
        sum += numbers.into_iter().sum::<u32>();
    }

    println!("solution 1 (sum of part numbers): {}", sum);
}
