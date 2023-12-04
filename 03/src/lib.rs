use grid::Grid;

mod element;
mod grid;
mod point;

pub fn run(contents: &str) {
    let grid: Grid = contents.parse().unwrap();

    let symbol_positions = grid.find_symbol_positions();

    let mut part_number_sum = 0;
    let mut gear_ratio_sum = 0;
    for (c, sp) in symbol_positions {
        let part_numbers = grid.find_part_numbers(sp);
        part_number_sum += part_numbers.iter().sum::<u32>();

        let is_gear = c == '*';
        let has_two_numbers = part_numbers.len() == 2;
        if is_gear && has_two_numbers {
            let first = part_numbers[0];
            let second = part_numbers[1];
            gear_ratio_sum += first * second;
        }
    }

    println!("solution 1 (sum of part numbers): {}", part_number_sum);
    println!("solution 2 (sum of gear ratios): {}", gear_ratio_sum);
}
