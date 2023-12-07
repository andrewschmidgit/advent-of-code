use hand::Hand;

mod card;
mod hand;

pub fn parse(s: &str) -> Vec<Hand> {
    s.lines()
        .filter_map(|l| l.parse().ok())
        .collect()
}
