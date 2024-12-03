use history::History;

mod history;

pub fn run(contents: &str) {
    let histories: Vec<History> = contents
        .lines()
        .filter_map(|l| l.parse().ok())
        .collect();

    let sum = histories
        .iter()
        .fold(0, |acc, h| acc + h.predict_next());

    println!("solution 1 (next sum): {}", sum);

    let sum = histories
        .iter()
        .fold(0, |acc, h| acc + h.predict_prev());

    println!("solution 2 (prev sum): {}", sum);
}
